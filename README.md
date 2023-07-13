# 概要 Overview

.env を操作する CLI ユーティリティです。

CLI utility for .env or something like this.

# 始め方 Getting Started

cargo を使用してインストールします。Try cargo install.

```sh
cargo install --git https://github.com/mass10/r-env --branch main
```

もしくは、単独で実行可能なバイナリファイルをダウンロードできます。Or executable file available.

```sh
wget https://github.com/mass10/r-env/releases/latest/download/r-env
```

# 凡例 Examples

### 環境変数を表示 する Print environment variables

```sh
r-env cmd.exe /C SET
```

### .env をダンプする Dump .env

```sh
r-env --dump
```

### ファイルを指定してダンプ Dump specified .env file

```sh
r-env --dump --file .env.development
```
