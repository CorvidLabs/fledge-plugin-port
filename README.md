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
fledge port kill 3000         # kill the process bound to port 3000
fledge port list              # list all bound ports (with PIDs and commands)
```

## License

MIT
