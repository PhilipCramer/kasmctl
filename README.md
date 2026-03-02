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
- **Health check** — verify connectivity and authentication with `kasmctl health`
- **Deployment overview** — monitor sessions, users, errors, and agent resources with `kasmctl top`
- **Shell completions** — generate completions for bash, zsh, fish, and more

## Installation

### Pre-built binaries

#### Quick install

```sh
curl -sSL https://raw.githubusercontent.com/PhilipCramer/kasmctl/main/scripts/install.sh | sh
```

The installer automatically detects your platform, downloads the latest release, and verifies the checksum. You can customize the install with environment variables:

| Variable | Description | Default |
|---|---|---|
| `KASMCTL_VERSION` | Version to install (e.g. `0.1.0`) | latest |
| `KASMCTL_INSTALL` | Install directory | `/usr/local/bin` |

#### Debian/Ubuntu (.deb)

Download the `.deb` package for your architecture from the [latest release](https://github.com/PhilipCramer/kasmctl/releases/latest):

```sh
# amd64
sudo dpkg -i kasmctl_*_amd64.deb

# arm64
sudo dpkg -i kasmctl_*_arm64.deb
```

The `.deb` package installs the binary to `/usr/bin/` and shell completions for bash, zsh, and fish.

#### Manual download

Download the archive for your platform from the [latest release](https://github.com/PhilipCramer/kasmctl/releases/latest):

| Platform | Archive |
|---|---|
| Linux x86_64 (.deb) | `kasmctl_<version>-1_amd64.deb` |
| Linux x86_64 (static/musl) | `kasmctl-linux-amd64-musl.tar.gz` |
| Linux ARM64 (.deb) | `kasmctl_<version>-1_arm64.deb` |
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

### Check server health

```sh
kasmctl health
kasmctl health -o json
```

### Deployment overview

```sh
kasmctl top              # summary + agent table
kasmctl top agents       # agent resource table only
kasmctl top -o yaml
```

### Shell completions

```sh
kasmctl completion bash >> ~/.bashrc
kasmctl completion zsh >> ~/.zshrc
kasmctl completion fish > ~/.config/fish/completions/kasmctl.fish
```

## Documentation

For detailed usage of each resource, see the full documentation:

- [Command Reference](docs/commands.md) — global options, full command table, resource aliases
- [Sessions](docs/sessions.md) — session lifecycle, filters, bulk operations
- [Images](docs/images.md) — image management, create/update options
- [Zones](docs/zones.md) — deployment zones
- [Agents](docs/agents.md) — docker agent management
- [Servers](docs/servers.md) — server management, create/update options
- [Configuration](docs/configuration.md) — contexts, environment variables, config file format

## Configuration

kasmctl stores its configuration in a YAML file at the platform-specific config directory (e.g. `~/.config/kasmctl/config.yaml` on Linux). The path can be overridden with the `KASMCTL_CONFIG` environment variable.

See [Configuration](docs/configuration.md) for full details on contexts, environment variables, and config file format.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
