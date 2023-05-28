# r-env
CLI utility for .env or something like this.

# Getting Started

Install

```sh
cargo install --git https://github.com/mass10/r-env --branch main
```

Or download single binary file.

```sh
wget https://github.com/mass10/r-env/releases/latest/download/r-env
```

# Examples

### Print environment variables.

```sh
r-env cmd.exe /C SET
```

### Dump .env

```sh
r-env --dump
```

### Dump .env.development

```sh
r-env --dump --file .env.development
```
