# Configuration

kasmctl uses a YAML configuration file with kubeconfig-style contexts to manage connections to multiple Kasm deployments.

## Config File Location

kasmctl stores its configuration in a YAML file at the platform-specific config directory:

| Platform | Path |
|---|---|
| Linux | `~/.config/kasmctl/config.yaml` |
| macOS | `~/Library/Application Support/kasmctl/config.yaml` |
| Windows | `%APPDATA%\kasmctl\config.yaml` |

The path can be overridden with the `KASMCTL_CONFIG` environment variable.

## Contexts

Contexts store server URL and API credentials, allowing you to quickly switch between Kasm deployments.

## Managing Contexts

### Add or update a context

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

### List contexts

```sh
kasmctl config get-contexts
```

## Environment Variables

| Variable | Description |
|---|---|
| `KASMCTL_CONFIG` | Override the config file path |
| `KASMCTL_API_KEY` | API key (used with `--server` flag) |
| `KASMCTL_API_SECRET` | API secret (used with `--server` flag) |

## Config File Format

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

## See also

- [Command Reference](commands.md)
- [Sessions](sessions.md)
- [Images](images.md)
- [Zones](zones.md)
- [Agents](agents.md)
- [Servers](servers.md)
