# Jalink

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)

Jalink 是使用 Rust 编写的 IM 后端。

## 配置

```bash
cp .env.example .env
```

获取自己的 [GitHub 客户端 ID 和 Secret](https://github.com/settings/applications/new) ，在回调 URL 里面填上 `http://localhost:8000/oauth/github/callback` 。

然后在 `.env` 里面填上 *GITHUB_CLIENT_ID* 和 *GITHUB_CLIENT_SECRET*。

数据库使用本地的 PostgreSQL，如果数据库的 URL 不同还请修改。

Jalink 使用 Diesel 的 CLI 来管理数据库，在运行项目前请确保已安装 [diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) 并运行以下命令初始化数据库：

```bash
diesel migration run
```

## 运行

```
cargo run
```

# 特别感谢

*Jalink* 的名字来源于 [Kry4tal](https://github.com/Kry4tal) 。

# 作者

**Jalink** © [LJason](https://github.com/LJason77) ，根据 [Anti 996](./LICENSE.NPL) 许可发布。

