# Agents

Agents are Docker agents that run workspace containers in Kasm. kasmctl supports listing, inspecting, and updating agents.

## List Agents

```sh
kasmctl get agents
kasmctl get agents --enabled
kasmctl get agents --zone <ZONE_ID>
```

## Get a Specific Agent

```sh
kasmctl get agent <AGENT_ID>
```

## Update an Agent

```sh
kasmctl update agent <AGENT_ID> --enabled false
kasmctl update agent <AGENT_ID> --cores-override 4.0 --memory-override 8589934592
```

## Filter Options

`get agents` accepts the following filters:

| Option | Description |
|---|---|
| `--zone <ZONE>` | Filter by zone ID (exact match) |
| `--enabled` | Only show enabled agents |
| `--disabled` | Only show disabled agents |
| `--status <STATUS>` | Filter by agent status (case-insensitive) |

## Update Options

`update agent <ID>` accepts any combination of the following options. Only specified fields are changed:

| Option | Description |
|---|---|
| `--enabled <BOOL>` | Enable or disable the agent |
| `--cores-override <CORES>` | Override CPU cores allocation |
| `--memory-override <BYTES>` | Override memory allocation in bytes |
| `--gpus-override <GPUS>` | Override GPU allocation |
| `--auto-prune-images <POLICY>` | Auto-prune images policy |

## See also

- [Command Reference](commands.md)
- [Zones](zones.md)
- [Servers](servers.md)
- [Configuration](configuration.md)
