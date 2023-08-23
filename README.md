# Overview 概要

CLI utility for .env or something like this. .env を操作する CLI ユーティリティです。

# Getting Started 始め方

Try cargo install. cargo を使用してインストールします。

```sh
cargo install --git https://github.com/mass10/r-env --branch main
```

Or executable file available. もしくは、単独で実行可能なバイナリファイルをダウンロードできます。

```sh
wget https://github.com/mass10/r-env/releases/latest/download/r-env
```

# Examples 凡例

### Print environment variables. 環境変数を表示する

```sh
r-env cmd.exe /C SET
```

### Dump .env. .env をダンプする

```sh
r-env --dump
```

### Dump specified .env file. ファイルを指定してダンプする

```sh
r-env --dump --file .env.development
```
