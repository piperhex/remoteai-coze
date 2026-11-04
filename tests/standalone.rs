use remoteai_coze::proxy::ModelProxy;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::Duration,
};

#[test]
fn standalone_accepts_https_without_widening_the_desktop_boundary() {
    assert!(ModelProxy::standalone("https://example.com/v1", "test".into()).is_ok());
    assert!(ModelProxy::new("https://example.com/v1", "test".into()).is_err());
    for url in [
        "http://example.com/v1",
        "https://key@example.com/v1",
        "https://example.com/v1?key=x",
    ] {
        assert!(ModelProxy::standalone(url, "test".into()).is_err());
    }
}

#[test]
fn standalone_survives_closed_stdin_and_writes_mcp_state() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("config.json");
    std::fs::write(&config, serde_json::json!({
        "dataDir": root.path().join("data"), "proxyUrl": "http://127.0.0.1:1/v1", "proxyToken": "test"
    }).to_string()).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_remoteai-coze"))
        .arg("--config")
        .arg(config)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let ready: serde_json::Value = serde_json::from_str(&line).unwrap();
        let state: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.path().join("data/runtime.json")).unwrap())
                .unwrap();
        assert!(ready["url"].as_str().unwrap().contains("/#token="));
        assert_eq!(state["token"].as_str().unwrap().len(), 64);
        let response = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap()
            .get(format!("{}/health", state["url"].as_str().unwrap()))
            .send()
            .unwrap();
        assert!(response.status().is_success());
        assert!(child.try_wait().unwrap().is_none());
    }));
    child.kill().unwrap();
    child.wait().unwrap();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}
