# kasmctl

A command-line tool for managing [Kasm Workspaces](https://kasm.com), inspired by `kubectl`.

## Features

- **Session management** — list, inspect, create, delete, stop, pause, and resume sessions
- **Image browsing** — list and inspect available workspace images
- **Multi-context configuration** — manage multiple Kasm servers with kubeconfig-style contexts
- **Flexible output** — table, JSON, and YAML output formats

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2024).

```sh
git clone https://github.com/PhilipCramer/kasmctl.git
cd kasmctl
cargo install --path .
```

## Quick start

### Configure a context

```sh
kasmctl config set-context my-server \
  --server https://kasm.example.com \
  --api-key <API_KEY> \
  --api-secret <API_SECRET>
```

### Switch contexts

```sh
kasmctl config use-context my-server
```

### List sessions

```sh
kasmctl get sessions
```

### Create a session

```sh
kasmctl create session --image <IMAGE_ID>
```

### Delete a session

```sh
kasmctl delete session <SESSION_ID>
```

### List images

```sh
kasmctl get images
```

### Stop, pause, or resume a session

```sh
kasmctl stop session <SESSION_ID>
kasmctl pause session <SESSION_ID>
kasmctl resume session <SESSION_ID>
```

## Usage

```
kasmctl [OPTIONS] <COMMAND>
```

### Global options

| Option | Description |
|---|---|
| `-o, --output <FORMAT>` | Output format: `table`, `json`, `yaml` (default: `table`) |
| `--context <NAME>` | Override the active context |
| `--server <URL>` | Override server URL (requires `KASMCTL_API_KEY` and `KASMCTL_API_SECRET` env vars) |

### Commands

| Command | Description |
|---|---|
| `get sessions [ID] [--status <STATUS>]` | List all sessions or get a specific session |
| `get images [ID]` | List all images or get a specific image |
| `create session --image <ID>` | Create a new session from a workspace image |
| `delete session <ID>` | Delete a session |
| `stop session <ID>` | Stop a session (frees memory/CPU, keeps disk) |
| `pause session <ID>` | Pause a session (retains memory, stops CPU) |
| `resume session <ID>` | Resume a stopped or paused session |
| `config set-context <NAME>` | Add or update a context |
| `config use-context <NAME>` | Switch the active context |
| `config get-contexts` | List all configured contexts |

## Configuration

kasmctl stores its configuration in a YAML file at the platform-specific config directory (e.g. `~/.config/kasmctl/config.yaml` on Linux). The path can be overridden with the `KASMCTL_CONFIG` environment variable.

Contexts store server URL and API credentials, allowing you to quickly switch between Kasm deployments:

```yaml
current-context: production
contexts:
  - name: production
    context:
      server: https://kasm.example.com
      api_key: <key>
      api_secret: <secret>
  - name: dev
    context:
      server: https://kasm-dev.example.com
      api_key: <key>
      api_secret: <secret>
      insecure: true
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
