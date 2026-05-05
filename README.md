# fledge-plugin-port

Find and kill processes bound to a TCP port — wraps `lsof` (macOS/Linux) and `netstat` (Windows) with a friendlier interface.

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-port
```

## Usage

```bash
fledge port 3000              # show what's bound to port 3000
fledge port show 3000         # same as above (explicit subcommand)
fledge port kill 3000         # kill the process bound to port 3000 (SIGTERM)
fledge port kill 3000 --force # send SIGKILL immediately
fledge port list              # list all bound ports (with PIDs and commands)
```

Running `fledge port` with no arguments is equivalent to `fledge port list`.

## Platform support

| Platform       | Backend   |
|----------------|-----------|
| macOS / Linux  | `lsof`    |
| Windows        | `netstat` + `taskkill` |

## Build from source

```bash
cargo build --release
```

The binary is produced at `target/release/fledge-port`.

## License

MIT
