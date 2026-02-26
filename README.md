# kasmctl

A command-line tool for managing [Kasm Workspaces](https://kasm.com), inspired by `kubectl`.

## Features

- **Session management** — get, create, delete, stop, pause, and resume sessions
- **Image management** — get, create, update, and delete workspace images
- **Zone management** — list and inspect deployment zones
- **Agent management** — list, inspect, and update docker agents
- **Server management** — get, create, update, and delete servers
- **Multi-context configuration** — manage multiple Kasm servers with kubeconfig-style contexts
- **Flexible output** — table, JSON, and YAML output formats
- **Shell completions** — generate completions for bash, zsh, fish, and more

## Installation

### Pre-built binaries

#### Quick install

```sh
curl -sSL https://raw.githubusercontent.com/PhilipCramer/kasmctl/main/packaging/install.sh | sh
```

The installer automatically detects your platform, downloads the latest release, and verifies the checksum. You can customize the install with environment variables:

| Variable | Description | Default |
|---|---|---|
| `KASMCTL_VERSION` | Version to install (e.g. `0.1.0`) | latest |
| `KASMCTL_INSTALL` | Install directory | `/usr/local/bin` |

#### Manual download

Download the archive for your platform from the [latest release](https://github.com/PhilipCramer/kasmctl/releases/latest):

| Platform | Archive |
|---|---|
| Linux x86_64 | `kasmctl-linux-amd64.tar.gz` |
| Linux x86_64 (static/musl) | `kasmctl-linux-amd64-musl.tar.gz` |
| Linux ARM64 | `kasmctl-linux-arm64.tar.gz` |
| Linux ARM64 (static/musl) | `kasmctl-linux-arm64-musl.tar.gz` |
| macOS Intel | `kasmctl-darwin-amd64.tar.gz` |
| macOS Apple Silicon | `kasmctl-darwin-arm64.tar.gz` |

Then extract and install:

```sh
tar xzf kasmctl-*.tar.gz
sudo mv kasmctl /usr/local/bin/
kasmctl --version
```

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
kasmctl get session <SESSION_ID> --user <USER_ID>
```

### Create a session

```sh
kasmctl create session --image <IMAGE_ID>
```

### Delete a session

```sh
kasmctl delete session <SESSION_ID>
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

### List images

```sh
kasmctl get images
kasmctl get images --enabled
kasmctl get images --name ubuntu
```

### Get a specific image

```sh
kasmctl get image <IMAGE_ID>
```

### Create an image

```sh
kasmctl create image --name kasmweb/terminal:1.18.0 --friendly-name "Terminal"
```

### Update an image

```sh
kasmctl update image <IMAGE_ID> --friendly-name "New Name" --enabled false
```

### Delete an image

```sh
kasmctl delete image <IMAGE_ID>
```

### List zones

```sh
kasmctl get zones
kasmctl get zones --name "default"
```

### Get a specific zone

```sh
kasmctl get zone <ZONE_ID>
```

### List docker agents

```sh
kasmctl get agents
kasmctl get agents --enabled
kasmctl get agents --zone <ZONE_ID>
```

### Get a specific docker agent

```sh
kasmctl get agent <AGENT_ID>
```

### Update a docker agent

```sh
kasmctl update agent <AGENT_ID> --enabled false
kasmctl update agent <AGENT_ID> --cores-override 4.0 --memory-override 8589934592
```

### List servers

```sh
kasmctl get servers
kasmctl get servers --enabled
kasmctl get servers --zone <ZONE_ID> --connection-type ssh
```

### Get a specific server

```sh
kasmctl get server <SERVER_ID>
```

### Create a server

```sh
kasmctl create server \
  --friendly-name "My Server" \
  --hostname 10.0.0.1 \
  --connection-type ssh \
  --connection-port 22 \
  --zone <ZONE_ID>
```

### Update a server

```sh
kasmctl update server <SERVER_ID> --friendly-name "New Name" --enabled false
```

### Delete a server

```sh
kasmctl delete server <SERVER_ID>
```

### Shell completions

```sh
kasmctl completion bash >> ~/.bashrc
kasmctl completion zsh >> ~/.zshrc
kasmctl completion fish > ~/.config/fish/completions/kasmctl.fish
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
| `get session <ID> --user <USER>` | Get details for a specific session |
| `get sessions [FILTERS]` | List all sessions, optionally filtered |
| `get image <ID>` | Get details for a specific image |
| `get images [FILTERS]` | List all available workspace images, optionally filtered |
| `create session --image <ID> [--user <ID>]` | Create a new session from a workspace image |
| `create image --name <NAME> --friendly-name <NAME> [OPTIONS]` | Create a new workspace image |
| `update image <ID> [OPTIONS]` | Update an existing workspace image |
| `get zone <ID>` | Get details for a specific zone |
| `get zones [FILTERS]` | List all zones, optionally filtered |
| `get agent <ID>` | Get details for a specific docker agent |
| `get agents [FILTERS]` | List all docker agents, optionally filtered |
| `get server <ID>` | Get details for a specific server |
| `get servers [FILTERS]` | List all servers, optionally filtered |
| `create server --friendly-name <NAME> --hostname <HOST> --connection-type <TYPE> --connection-port <PORT> --zone <ZONE_ID>` | Create a new server |
| `update agent <ID> [OPTIONS]` | Update a docker agent |
| `update server <ID> [OPTIONS]` | Update an existing server |
| `delete session <ID>` | Delete a session |
| `delete image <ID>` | Delete an image |
| `delete server <ID>` | Delete a server |
| `stop session <ID>` | Stop a session (frees memory/CPU, keeps disk) |
| `stop sessions [FILTERS] [-y]` | Stop multiple sessions matching filters |
| `pause session <ID>` | Pause a session (retains memory, stops CPU) |
| `pause sessions [FILTERS] [-y]` | Pause multiple sessions matching filters |
| `resume session <ID>` | Resume a stopped or paused session |
| `resume sessions [FILTERS] [-y]` | Resume multiple sessions matching filters |
| `config set-context <NAME>` | Add or update a context |
| `config use-context <NAME>` | Switch the active context |
| `config get-contexts` | List all configured contexts |
| `completion <SHELL>` | Generate shell completions (bash, zsh, fish, etc.) |

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

### Image filter options

`get images` accepts the following filters:

| Option | Description |
|---|---|
| `--enabled` | Only show enabled images |
| `--disabled` | Only show disabled images |
| `--name <NAME>` | Filter by friendly name (case-insensitive substring match) |
| `--image-type <TYPE>` | Filter by image type / source (e.g. `Container`, `Server`) |

### Zone filter options

`get zones` accepts the following filters:

| Option | Description |
|---|---|
| `--name <NAME>` | Filter by zone name (case-insensitive substring match) |

### Agent filter options

`get agents` accepts the following filters:

| Option | Description |
|---|---|
| `--zone <ZONE>` | Filter by zone ID (exact match) |
| `--enabled` | Only show enabled agents |
| `--disabled` | Only show disabled agents |
| `--status <STATUS>` | Filter by agent status (case-insensitive) |

### Server filter options

`get servers` accepts the following filters:

| Option | Description |
|---|---|
| `--zone <ZONE>` | Filter by zone ID (exact match) |
| `--connection-type <TYPE>` | Filter by connection type (case-insensitive exact match) |
| `--enabled` | Only show enabled servers |
| `--disabled` | Only show disabled servers |
| `--name <NAME>` | Filter by friendly name (case-insensitive substring match) |

### Create image options

`create image` requires `--name` and `--friendly-name`. All other options are optional:

| Option | Description |
|---|---|
| `--name <NAME>` | Docker image name (e.g. `kasmweb/terminal:1.18.0`) **(required)** |
| `--friendly-name <NAME>` | Human-readable display name **(required)** |
| `--description <TEXT>` | Image description |
| `--cores <CORES>` | Number of CPU cores to allocate |
| `--memory <BYTES>` | Memory in bytes to allocate |
| `--enabled <BOOL>` | Whether the image is enabled (default: `true`) |
| `--image-src <SRC>` | Image source type (default: `Container`) |
| `--docker-registry <URL>` | Docker registry URL |
| `--run-config <JSON>` | Run configuration JSON |
| `--exec-config <JSON>` | Exec configuration JSON |
| `--image-type <TYPE>` | Image type (e.g. `Container`, `Server`) |

### Update image options

`update image <ID>` accepts any combination of the following options. Only specified fields are changed:

| Option | Description |
|---|---|
| `--name <NAME>` | Docker image name |
| `--friendly-name <NAME>` | Human-readable display name |
| `--description <TEXT>` | Image description |
| `--cores <CORES>` | Number of CPU cores |
| `--memory <BYTES>` | Memory in bytes |
| `--enabled <BOOL>` | Enable or disable the image |
| `--image-src <SRC>` | Image thumbnail source path |
| `--docker-registry <URL>` | Docker registry URL |
| `--run-config <JSON>` | Docker run config override (JSON) |
| `--exec-config <JSON>` | Docker exec config override (JSON) |
| `--hidden <BOOL>` | Hide the image from users |

### Create server options

`create server` requires `--friendly-name`, `--hostname`, `--connection-type`, `--connection-port`, and `--zone`. All other options are optional:

| Option | Description |
|---|---|
| `--friendly-name <NAME>` | Human-readable name **(required)** |
| `--hostname <HOST>` | Server hostname or IP **(required)** |
| `--connection-type <TYPE>` | Connection type (ssh, rdp, vnc, kasmvnc) **(required)** |
| `--connection-port <PORT>` | Connection port **(required)** |
| `--zone <ZONE>` | Zone ID to assign the server to **(required)** |
| `--enabled <BOOL>` | Whether the server is enabled (default: `true`) |
| `--connection-username <USER>` | Connection username |
| `--connection-info <INFO>` | Connection info/credentials |
| `--max-simultaneous-sessions <N>` | Maximum simultaneous sessions |
| `--max-simultaneous-users <N>` | Maximum simultaneous users |
| `--pool-id <ID>` | Pool ID |

### Update agent options

`update agent <ID>` accepts any combination of the following options. Only specified fields are changed:

| Option | Description |
|---|---|
| `--enabled <BOOL>` | Enable or disable the agent |
| `--cores-override <CORES>` | Override CPU cores allocation |
| `--memory-override <BYTES>` | Override memory allocation in bytes |
| `--gpus-override <GPUS>` | Override GPU allocation |
| `--auto-prune-images <POLICY>` | Auto-prune images policy |

### Update server options

`update server <ID>` accepts any combination of the following options. Only specified fields are changed:

| Option | Description |
|---|---|
| `--friendly-name <NAME>` | Human-readable name |
| `--hostname <HOST>` | Server hostname or IP |
| `--enabled <BOOL>` | Enable or disable the server |
| `--connection-type <TYPE>` | Connection type |
| `--connection-port <PORT>` | Connection port |
| `--connection-username <USER>` | Connection username |
| `--connection-info <INFO>` | Connection info/credentials |
| `--max-simultaneous-sessions <N>` | Maximum simultaneous sessions |
| `--max-simultaneous-users <N>` | Maximum simultaneous users |
| `--zone-id <ID>` | Zone ID |
| `--pool-id <ID>` | Pool ID |

### Resource aliases

Session resources accept `kasm` (singular) and `kasms` (plural) as aliases:

```sh
kasmctl get kasm <ID> --user <USER>  # same as: get session <ID> --user <USER>
kasmctl get kasms             # same as: get sessions
kasmctl stop kasm <ID>        # same as: stop session <ID>
kasmctl stop kasms --status running  # same as: stop sessions --status running
kasmctl delete kasm <ID>      # same as: delete session <ID>
```

Agent resources accept `docker-agent` (singular) and `docker-agents` (plural) as aliases:

```sh
kasmctl get docker-agent <ID>  # same as: get agent <ID>
kasmctl get docker-agents      # same as: get agents
kasmctl update docker-agent <ID> --enabled false  # same as: update agent <ID> --enabled false
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
