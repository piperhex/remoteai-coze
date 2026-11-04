use crate::{
    api::Services,
    database::Database,
    embedding::LocalEmbedder,
    error::{Error, Result},
    proxy::ModelProxy,
    server::Host,
};
use serde::Deserialize;
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Desktop bootstrapping remains compatible with protocol 1. Standalone files may omit the token.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Bootstrap {
    data_dir: PathBuf,
    #[serde(default)]
    token: String,
    proxy_url: String,
    proxy_token: String,
}

/// Serve the same editor and API in desktop-managed or independent process mode.
pub fn serve(config: Bootstrap, managed: bool) -> Result<()> {
    if !config.data_dir.is_absolute() || (managed && config.token.is_empty()) {
        return Err(Error::Invalid);
    }
    let executable = std::env::current_exe()?;
    let root = executable.parent().ok_or(Error::Invalid)?;
    let token = if config.token.is_empty() {
        format!(
            "{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        )
    } else {
        config.token
    };
    let db = Database::open(&config.data_dir)?;
    db.connect()?.execute(
        "UPDATE runs SET status='interrupted',error='上次运行已中断。' WHERE status='running'",
        [],
    )?;
    let proxy = if managed {
        ModelProxy::new(&config.proxy_url, config.proxy_token)?
    } else {
        ModelProxy::standalone(&config.proxy_url, config.proxy_token)?
    };
    let services = Arc::new(Services::new(
        db,
        Arc::new(LocalEmbedder::new(&root.join("models"))),
        proxy,
    ));
    let host = Arc::new(Host::bind(token.clone(), root.join("ui"))?);
    if let Err(error) = crate::external::restore(&services) {
        eprintln!("Coze external tools were not restarted: {error}");
    }
    announce(&host, &config.data_dir, &token, managed)?;
    if managed {
        watch_parent()?;
    }
    host.run(services)
}

fn announce(host: &Host, data_dir: &Path, token: &str, managed: bool) -> Result<()> {
    let url = if managed {
        host.url.clone()
    } else {
        format!("{}/#token={token}", host.url)
    };
    if !managed {
        write_state(data_dir, &host.url, token)?;
    }
    println!(
        "{}",
        serde_json::json!({"url":url,"version":env!("CARGO_PKG_VERSION")})
    );
    std::io::stdout().flush()?;
    Ok(())
}

fn write_state(root: &Path, url: &str, token: &str) -> Result<()> {
    let mut temporary = tempfile::NamedTempFile::new_in(root)?;
    // tempfile creates a private file on Unix; persisting retains its permissions.
    serde_json::to_writer(
        &mut temporary,
        &serde_json::json!({"url":url,"token":token}),
    )
    .map_err(|_| Error::Invalid)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist(root.join("runtime.json"))
        .map_err(|error| Error::File(error.error))?;
    Ok(())
}

fn watch_parent() -> Result<()> {
    std::thread::Builder::new()
        .name("coze-parent".into())
        .spawn(|| {
            let mut line = String::new();
            loop {
                match std::io::stdin().read_line(&mut line) {
                    Ok(0) | Err(_) => std::process::exit(0),
                    Ok(_) => line.clear(),
                }
            }
        })?;
    Ok(())
}
