# Zones

Zones represent deployment regions or clusters in Kasm. kasmctl supports listing and inspecting zones.

## List Zones

```sh
kasmctl get zones
kasmctl get zones --name "default"
```

## Get a Specific Zone

```sh
kasmctl get zone <ZONE_ID>
```

## Filter Options

`get zones` accepts the following filters:

| Option | Description |
|---|---|
| `--name <NAME>` | Filter by zone name (case-insensitive substring match) |

## See also

- [Command Reference](commands.md)
- [Agents](agents.md)
- [Servers](servers.md)
- [Configuration](configuration.md)
