# Servers

Servers are physical or virtual machines registered in Kasm for running workspaces. kasmctl supports full CRUD operations on servers.

## List Servers

```sh
kasmctl get servers
kasmctl get servers --enabled
kasmctl get servers --zone <ZONE_ID> --connection-type ssh
```

## Get a Specific Server

```sh
kasmctl get server <SERVER_ID>
```

## Create a Server

```sh
kasmctl create server \
  --friendly-name "My Server" \
  --hostname 10.0.0.1 \
  --connection-type ssh \
  --connection-port 22 \
  --zone <ZONE_ID>
```

## Update a Server

```sh
kasmctl update server <SERVER_ID> --friendly-name "New Name" --enabled false
```

## Delete a Server

```sh
kasmctl delete server <SERVER_ID>
```

## Filter Options

`get servers` accepts the following filters:

| Option | Description |
|---|---|
| `--zone <ZONE>` | Filter by zone ID (exact match) |
| `--connection-type <TYPE>` | Filter by connection type (case-insensitive exact match) |
| `--enabled` | Only show enabled servers |
| `--disabled` | Only show disabled servers |
| `--name <NAME>` | Filter by friendly name (case-insensitive substring match) |

## Create Options

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

## Update Options

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

## See also

- [Command Reference](commands.md)
- [Zones](zones.md)
- [Agents](agents.md)
- [Configuration](configuration.md)
