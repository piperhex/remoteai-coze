# Coze 的 Rust 本地运行包

> For English documentation, see [README_EN.md](README_EN.md). Русская версия — [README_RU.md](README_RU.md).

Remote AI 通过左侧「插件」下方的 `coze` 入口安装此可选组件。首次打开只检查状态，用户点击下载后才获取独立签名包。主安装包不包含运行程序、编辑器或向量模型。

没有打包或启动上游 Go 后端，也不需要 Docker、MySQL、Redis、Elasticsearch、Milvus、Python 或 Node.js 运行环境。

## 独立仓库与启动

源码位于 https://github.com/piperhex/remoteai-coze；Remote AI 将此仓库作为 `apps/coze` 子模块。
所有依赖、翻译、测试和发布工具均在本仓库内维护，可以直接 clone 后构建，无需 Remote AI 源码。

下载并解压完整发布包后，创建自己的配置文件（请勿提交模型密钥）：

```json
{
  "dataDir": "F:/coze-data",
  "proxyUrl": "https://your-model-provider.example/v1",
  "proxyToken": "YOUR_API_KEY"
}
```

运行 `remoteai-coze.exe --config config.json`（macOS/Linux 使用 `./remoteai-coze`），
打开终端返回的 `url` 即可使用编辑器。`dataDir` 必须是绝对路径；本机代理可使用
`http://127.0.0.1:<port>/v1`，远程服务须使用 HTTPS，并支持模型列表与 Chat Completions。
启动时自动生成编辑器凭据，MCP 状态保存在数据目录的 `runtime.json`。
独立模式不依赖 stdin 或 Remote AI 进程，按 Ctrl+C 结束。编辑器仍只监听本机。
桌面集成继续使用兼容的 `--serve` 和 `--coze-mcp=<runtime.json>` 协议。


## 当前实现与边界

| 上游职责 | Rust 实现 | 当前能力 |
| --- | --- | --- |
| 模型管理 | `src/proxy.rs` | 自动使用 Remote AI 的 Codex GUI 本地代理、模型目录和当前账户；不复制上游密钥 |
| 知识库 | `src/knowledge/`、`src/embedding.rs` | 文档/网页/图片/扫描 PDF、来源刷新、原图、分段编辑、混合检索、引用；后台进度与取消 |
| 工作流 | `src/workflow/` | 可视化编排、Chatflow 人机交互与恢复、变量/聚合/意图分类、循环控制、会话节点、知识读写、SQL、运行轨迹；上游画布适配 |
| 代码节点 | `src/code_runtime/` | JavaScript、TypeScript、Python 子集；Rust 编译/解释，不调用系统 Python 或 Node |
| 智能体与会话 | `src/agents/`、`src/proxy/` | 知识问答、自主工具调用、按工具授权写表、工具历史、多轮记忆、真实流式输出与停止 |
| 应用与发布 | `src/projects.rs`、`src/publications.rs` | 应用资源组合、入口运行、递归冻结发布依赖、历史版本切换；知识文档和表记录持续更新 |
| 协作访问 | `src/collaboration_access/` | 应用范围的查看/使用/编辑权限、到期和撤销、草稿及运行历史隔离；本机访问链接 |
| 模板与备份 | `src/catalog.rs`、`src/backup.rs` | 本机模板广场、连同依赖导出/导入、全量数据库备份恢复，包含文档原文件、索引和历史 |
| 工具与 MCP | `src/plugins.rs`、`src/mcp.rs` | 应用/工作流/智能体/知识库/工具/提示词/表的 PC 调用及 Chatflow 恢复；自动注册两个 Codex Home |
| 数据表 | `src/tables/` | 字段类型、分页、事务批量导入、增删改查、隔离 SQL、自然语言生成查询并预览 |

这是面向桌面使用的功能迁移，不是上游 Go HTTP/Thrift 接口或数据库的替代服务。画布导入会拒绝尚无对应实现的节点，不能静默跳过。详细节点映射见 [工作流迁移说明](docs/coze-workflow-parity.md)，知识库和 SQL 的范围见 [知识库迁移说明](docs/coze-knowledge-parity.md)。

Python 使用 Monty 的受限 Python 语法和标准库，支持上游 `async def main(args)` 入口；不支持任意第三方 pip 包或宿主文件/网络访问。每次在独立 Rust 子进程中执行，有内存和时间上限。TypeScript 在本机转译后进入受限 QuickJS。代码语言兼容性不能描述成完整 CPython/Node 环境。

模板广场是本机可分享的模板库，没有连接 Coze 的在线商店、计费和审核服务。协作编辑链接用于本机发布应用；跨电脑工具调用需单独开启“外部调用”，不替代云组织登录或团队管理。扫描文件和图片识别使用所选视觉模型及现有代理路由，不能描述为必定离线。

发布固定整条依赖链的资源定义，不会随子工作流重新发布而漂移。知识文档、数据表字段和记录仍持续更新。单项导出和模板导出包含引用配置及字段约束；「备份与恢复」导出完整数据库，恢复要求相同数据库结构，并使旧分享链接失效。

## 运行与数据

- Tauri 命令只委托阻塞工作线程。下载、解压、进程启动、配置访问及状态检查不占用窗口线程。
- 编辑器监听 `127.0.0.1` 的随机端口，需要每次启动生成的凭据。可另行开启工具专用端口；此端口不提供编辑器、备份或管理操作。
- 启动信息通过 stdin 传递；父进程管道关闭后运行包退出。Windows 后台启动不弹出控制台。
- 桌面连接使用 Codex GUI 的 Responses 接口，统一承载文字、图像和工具调用；模型目录自动补齐客户端版本。
  独立测试连接普通 `/v1` 代理时保留 Chat Completions 兼容。中断或失败的流不会记作成功回答。
- 数据位于应用数据目录的 `coze/data/coze.sqlite3`。WAL、外键和事务保证索引失败不留下半份文档，删除文档同时清理向量和全文记录。
- 组件位于 `coze/packages/<version>`，更换包不删除 `coze/data`。未完成的下载不产生安装完成标记。
- 向量模型为固定版本的 `multilingual-e5-small`，由 Candle 在 CPU 上运行；首次使用知识库时才载入内存，不再下载模型。
- MCP 工具 `coze_list` 返回当前发布内容和输入结构，`coze_call` 调用对应内容，`coze_resume` 回答暂停的 Chatflow。草稿和停用内容不可调用；工具名稳定，因此新发布内容不要求重启会话。
- JavaScript 无文件、网络、模块加载等宿主能力，设有内存、栈、时间和输出限制。外部访问使用单独的 HTTPS 节点，不携带本地代理凭据。

## 开发验证

### 在 Codex GUI 中调用

1. 从左侧打开 Coze，创建并发布所需工作流、智能体、知识库或工具。
2. 回到 Codex GUI，在输入框输入 `#` 或 `#名称`，选择弹出的已发布内容。
3. 写明任务和输入后发送。例如选择 `#库存查询`，再输入“查询商品 A 的库存”。

选择项会作为引用保留在消息、队列和编辑草稿中。模型通过已经注册的 `remoteai_coze` MCP 查询输入要求并调用对应资源；名称重复时仍以选中项的 ID 为准。输入 `#` 只启动已经安装的组件，不触发自动下载；首次安装仍需从 Coze 入口按需下载。

### 提供给其他应用和电脑

进入资源的“设置 → 外部调用”，选择“仅本机应用”或“允许其他电脑”，开启服务并创建专用调用密钥。每把密钥只授权一个已发布资源，支持 1–365 天有效期和随时停用。密钥只显示一次，设置中提供可复制的 MCP 配置和 API 示例。

- **MCP**：使用 Streamable HTTP 地址 `http://<电脑地址>:15461/mcp`，设置请求头 `Authorization: Bearer <调用密钥>`。支持 `coze_list`、`coze_call`、`coze_resume`。协议协商支持 2025-03-26、2025-06-18、2025-11-25；不提供旧版独立 SSE 端点。
- **API**：向 `http://<电脑地址>:15461/api/run` 发送 POST JSON，使用相同授权头。工作流和工具传 `{ "input": { ... } }`，智能体传 `{ "message": "..." }`，知识库传 `{ "query": "..." }`；密钥自动确定资源。继续聊天时，将返回的 `conversationId` 作为 `conversation_id` 传入。
- **Chatflow 恢复**：向 `/api/resume` 发送 `{ "run_id": "...", "interaction_id": "...", "input": { ... } }`，只能恢复同一密钥创建的运行。
- 本机使用 `127.0.0.1`；其他电脑使用本机可达的局域网或 VPN 地址，必要时放行选定端口。跨公网通过 VPN 或 HTTPS 网关接入，不自动建立公网隧道；此接口面向服务端或桌面客户端，不允许浏览器跨域直接访问。
- 连接范围和端口是本机公共设置，关闭会暂停全部外部调用。Remote AI 和 Coze 必须保持运行；下次启动 Coze 会恢复监听设置。端口冲突时可从设置重新开启。网络设置独立于内容备份，恢复内容不会顺带开放网络端口。
- 编辑器启动凭据与外部调用密钥分开；外部密钥不能访问管理 API、草稿或其他密钥的运行记录。停止发布后原密钥不能继续运行此内容；知识库文档与数据表记录仍按既有发布规则持续更新。

### 验证命令

在仓库根目录执行：

```powershell
cargo test --manifest-path Cargo.toml
cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings
node node_modules/typescript/bin/tsc -p ui/tsconfig.json
node node_modules/vite/bin/vite.js build --config ui/vite.config.ts
npm test
npx playwright install chromium
npm run test:e2e
node scripts/prepare-coze-model.mjs
cargo test --manifest-path Cargo.toml --test local_embedding -- --ignored
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --bins --manifest-path Cargo.toml
node scripts/package-coze.mjs target/debug/remoteai-coze.exe .codex-tmp/coze-package
node scripts/verify-windows-runtime.mjs .codex-tmp/coze-package/remoteai-coze.exe
node scripts/verify-windows-runtime.mjs .codex-tmp/coze-package/remoteai-coze-python.exe
node scripts/smoke-coze.mjs .codex-tmp/coze-package/remoteai-coze.exe
```

需要 Rust 1.96 或更新版本。`package-coze.mjs` 要求新的输出目录，并同时验证主程序及 Python 节点程序。macOS/Linux 去掉 `.exe`。冒烟验证使用临时数据和测试模型代理，不修改用户知识库或真实模型配置；本地向量推理使用真实权重。

Windows 构建使用静态 C 运行库，并检查两个程序的 DLL 导入，确保用户无需另装 Visual C++ 运行库。
其他平台不设置上面的 `RUSTFLAGS`，也不运行 Windows DLL 检查。

`scripts/smoke-coze-live.mjs` 可连接桌面启动的真实模型代理，验证流式输出、取消、带引用问答、工具调用与历史、SQL、代码节点、图片及扫描 PDF。
它还测试整库恢复，因此必须使用空的专用验收工作区。传入 `runtime.json`、模型 ID、报告路径、测试图片和扫描 PDF；
两份图片资料应包含 `Verification code: VX-84620`。报告仅记录测试结果，不记录运行凭据。

## 独立发布

`.github/workflows/ci.yml` 在 main 推送和 PR 中执行完整检查，并构建 Windows x64/ARM64、
macOS x64/ARM64、Linux x64 的 ZIP。只有 `coze-v*` 标签或手动勾选 publish 才签名并发布。
发布位置为本仓库的 GitHub Releases，已发布包不可覆盖。

发布前同步更新 `Cargo.toml`、`Cargo.lock`、`package.json`、`package-lock.json` 和 `runtime.json`。
仓库 Secret `COZE_SIGNING_PRIVATE_KEY` 保存专用签名私钥；`runtime.json.publicKey` 是对应公钥。
维护者应妥善备份私钥，不能提交到 Git。主应用发布只检查已发布包，不再调用本仓库的构建。
Remote AI 的 `shared/coze/runtime.json` 单独锁定已验证的版本、公钥和协议；发布成功后再更新。

开发检查：`npm ci`、`npm run check`、`npm run test:e2e`、`cargo fmt --check`、
`cargo test --locked`、`cargo clippy --locked --all-targets -- -D warnings`。
先运行 `npm run prepare:model`，再执行真实向量测试和完整打包冒烟测试。
