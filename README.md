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
kasmctl get sessions --status running
```

### Get a specific session

```sh
kasmctl get session <SESSION_ID>
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

### Get a specific image

```sh
kasmctl get image <IMAGE_ID>
```

### Stop, pause, or resume a session

```sh
kasmctl stop session <SESSION_ID>
kasmctl pause session <SESSION_ID>
kasmctl resume session <SESSION_ID>
```

### Bulk session operations

Stop, pause, or resume multiple sessions at once using filters:

```sh
kasmctl stop sessions --status running
kasmctl pause sessions --idle-for 2h --yes
kasmctl resume sessions --user <USER_ID>
kasmctl get sessions --image <IMAGE_ID> --created-after "2024-01-01 00:00:00"
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
| `--insecure` | Skip TLS certificate verification (for self-signed certificates) |

### Commands

| Command | Description |
|---|---|
| `get session <ID>` | Get details for a specific session |
| `get sessions [FILTERS]` | List all sessions, optionally filtered |
| `get image <ID>` | Get details for a specific image |
| `get images` | List all available workspace images |
| `create session --image <ID> [--user <ID>]` | Create a new session from a workspace image |
| `delete session <ID>` | Delete a session |
| `stop session <ID>` | Stop a session (frees memory/CPU, keeps disk) |
| `stop sessions [FILTERS] [-y]` | Stop multiple sessions matching filters |
| `pause session <ID>` | Pause a session (retains memory, stops CPU) |
| `pause sessions [FILTERS] [-y]` | Pause multiple sessions matching filters |
| `resume session <ID>` | Resume a stopped or paused session |
| `resume sessions [FILTERS] [-y]` | Resume multiple sessions matching filters |
| `config set-context <NAME>` | Add or update a context |
| `config use-context <NAME>` | Switch the active context |
| `config get-contexts` | List all configured contexts |

### Session filter options

Bulk commands (`stop sessions`, `pause sessions`, `resume sessions`) and `get sessions` accept the following filters:

| Option | Description |
|---|---|
| `--status <STATUS>` | Filter by session status (case-insensitive) |
| `--image <IMAGE_ID>` | Filter by image ID |
| `--user <USER_ID>` | Filter by user ID |
| `--host <HOSTNAME>` | Filter by hostname |
| `--created-before <DATETIME>` | Sessions created before this time (`YYYY-MM-DD HH:MM:SS`) |
| `--created-after <DATETIME>` | Sessions created after this time |
| `--idle-since <DATETIME>` | Sessions idle (no keepalive) since this time |
| `--idle-for <DURATION>` | Sessions idle for at least this duration (e.g. `30m`, `2h`, `1d`) |
| `-y, --yes` | Skip confirmation prompt (bulk operations only) |

Multiple filters can be combined and are applied with AND logic.

### Resource aliases

Session resources accept `kasm` (singular) and `kasms` (plural) as aliases:

```sh
kasmctl get kasm <ID>        # same as: get session <ID>
kasmctl get kasms             # same as: get sessions
kasmctl stop kasm <ID>        # same as: stop session <ID>
kasmctl stop kasms --status running  # same as: stop sessions --status running
kasmctl delete kasm <ID>      # same as: delete session <ID>
```

## Configuration

kasmctl stores its configuration in a YAML file at the platform-specific config directory (e.g. `~/.config/kasmctl/config.yaml` on Linux). The path can be overridden with the `KASMCTL_CONFIG` environment variable.

Contexts store server URL and API credentials, allowing you to quickly switch between Kasm deployments:

```yaml
current-context: production
contexts:
  - name: production
    server: https://kasm.example.com
    api-key: <key>
    api-secret: <secret>
  - name: dev
    server: https://kasm-dev.example.com
    api-key: <key>
    api-secret: <secret>
    insecure-skip-tls-verify: true
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
