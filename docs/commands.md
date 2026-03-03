# Command Reference

Full reference for all kasmctl commands, global options, and resource aliases.

```
kasmctl [OPTIONS] <COMMAND>
```

## Global Options

| Option | Description |
|---|---|
| `-o, --output <FORMAT>` | Output format: `table`, `json`, `yaml` (default: `table`) |
| `--context <NAME>` | Override the active context |
| `--server <URL>` | Override server URL (requires `KASMCTL_API_KEY` and `KASMCTL_API_SECRET` env vars) |
| `--insecure` | Skip TLS certificate verification (for self-signed certificates) |

## Commands

| Command | Description |
|---|---|
| `get session <ID>` | Get details for a specific session |
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
| `exec session <ID> [OPTIONS] -- <CMD>...` | Execute a command inside a session |
| `exec sessions [FILTERS] [OPTIONS] [-y] -- <CMD>...` | Execute a command across multiple sessions |
| `config set-context <NAME>` | Add or update a context |
| `config use-context <NAME>` | Switch the active context |
| `config get-contexts` | List all configured contexts |
| `health` | Check connectivity and authentication to the Kasm server |
| `top` | Show deployment summary: sessions, users, errors, and agent resources |
| `top agents` | Show agent resource utilization only |
| `completion <SHELL>` | Generate shell completions (bash, zsh, fish, etc.) |

## Resource Aliases

Session resources accept `kasm` (singular) and `kasms` (plural) as aliases:

```sh
kasmctl get kasm <ID>         # same as: get session <ID>
kasmctl get kasms             # same as: get sessions
kasmctl stop kasm <ID>        # same as: stop session <ID>
kasmctl stop kasms --status running  # same as: stop sessions --status running
kasmctl delete kasm <ID>      # same as: delete session <ID>
kasmctl exec kasm <ID> -- <CMD>...   # same as: exec session <ID> -- <CMD>...
```

Agent resources accept `docker-agent` (singular) and `docker-agents` (plural) as aliases:

```sh
kasmctl get docker-agent <ID>  # same as: get agent <ID>
kasmctl get docker-agents      # same as: get agents
kasmctl update docker-agent <ID> --enabled false  # same as: update agent <ID> --enabled false
```

## See also

- [Sessions](sessions.md)
- [Images](images.md)
- [Zones](zones.md)
- [Agents](agents.md)
- [Servers](servers.md)
- [Configuration](configuration.md)
