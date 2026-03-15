# Images

Images define the workspace environments available in Kasm. kasmctl supports listing, creating, updating, and deleting images.

## List Images

```sh
kasmctl get images
kasmctl get images --enabled
kasmctl get images --name ubuntu
```

## Get a Specific Image

```sh
kasmctl get image <IMAGE>
```

`<IMAGE>` accepts an exact image ID, an ID prefix, or a case-insensitive friendly name (e.g. `"Terminal"`).

## Create an Image

```sh
kasmctl create image --name kasmweb/terminal:1.18.0 --friendly-name "Terminal"
```

## Update an Image

```sh
kasmctl update image <IMAGE> --friendly-name "New Name" --enabled false
```

`<IMAGE>` accepts an exact image ID, an ID prefix, or a case-insensitive friendly name.

## Delete an Image

```sh
kasmctl delete image <IMAGE>
```

`<IMAGE>` accepts an exact image ID, an ID prefix, or a case-insensitive friendly name.

## Filter Options

`get images` accepts the following filters:

| Option | Description |
|---|---|
| `--enabled` | Only show enabled images |
| `--disabled` | Only show disabled images |
| `--name <NAME>` | Filter by friendly name (case-insensitive substring match) |
| `--image-type <TYPE>` | Filter by image type / source (e.g. `Container`, `Server`) |

Multiple filters can be combined and are applied with AND logic.

## Create Options

`create image` requires `--name` and `--friendly-name`. All other options are optional:

| Option | Description |
|---|---|
| `--name <NAME>` | Docker image name (e.g. `kasmweb/terminal:1.18.0`) **(required)** |
| `--friendly-name <NAME>` | Human-readable display name **(required)** |
| `--description <TEXT>` | Image description |
| `--cores <CORES>` | Number of CPU cores to allocate |
| `--memory <MEMORY>` | Memory to allocate (e.g. `3GB`, `512MB`, or raw bytes) |
| `--enabled <BOOL>` | Whether the image is enabled (default: `true`) |
| `--image-src <SRC>` | Image source type (default: `Container`) |
| `--docker-registry <URL>` | Docker registry URL |
| `--run-config <JSON>` | Run configuration JSON |
| `--exec-config <JSON>` | Exec configuration JSON |
| `--image-type <TYPE>` | Image type (e.g. `Container`, `Server`) |

## Update Options

`update image <IMAGE>` accepts any combination of the following options. Only specified fields are changed:

| Option | Description |
|---|---|
| `--name <NAME>` | Docker image name |
| `--friendly-name <NAME>` | Human-readable display name |
| `--description <TEXT>` | Image description |
| `--cores <CORES>` | Number of CPU cores |
| `--memory <MEMORY>` | Memory to allocate (e.g. `3GB`, `512MB`, or raw bytes) |
| `--enabled <BOOL>` | Enable or disable the image |
| `--image-src <SRC>` | Image thumbnail source path |
| `--docker-registry <URL>` | Docker registry URL |
| `--run-config <JSON>` | Docker run config override (JSON) |
| `--exec-config <JSON>` | Docker exec config override (JSON) |
| `--hidden <BOOL>` | Hide the image from users |

## See also

- [Command Reference](commands.md)
- [Sessions](sessions.md)
- [Configuration](configuration.md)
