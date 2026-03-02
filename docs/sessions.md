# Sessions

Sessions represent running Kasm workspaces. kasmctl supports full lifecycle management of sessions.

## List Sessions

```sh
kasmctl get sessions
kasmctl get sessions --status running
```

## Get a Specific Session

```sh
kasmctl get session <SESSION_ID> --user <USER_ID>
```

## Create a Session

```sh
kasmctl create session --image <IMAGE_ID>
```

## Delete a Session

```sh
kasmctl delete session <SESSION_ID>
```

## Stop, Pause, and Resume

```sh
kasmctl stop session <SESSION_ID>
kasmctl pause session <SESSION_ID>
kasmctl resume session <SESSION_ID>
```

## Bulk Operations

Stop, pause, or resume multiple sessions at once using filters:

```sh
kasmctl stop sessions --status running
kasmctl pause sessions --idle-for 2h --yes
kasmctl resume sessions --user <USER_ID>
kasmctl get sessions --image <IMAGE_ID> --created-after "2024-01-01 00:00:00"
```

## Filter Options

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

## See also

- [Command Reference](commands.md)
- [Images](images.md)
- [Configuration](configuration.md)
