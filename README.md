# Terraria Server

A Dockerised Terraria dedicated server exposed via a [playit.gg](https://playit.gg) tunnel, with a TCP guard proxy sitting between the tunnel and the server, and an automatic world backup service.

## Architecture

```
Internet → playit tunnel → tcp-guard (TCP_GUARD_IP) → terraria-server (TERRARIA_SERVER_IP)
```

The `backups` container watches the worlds directory and automatically backs up `.wld` files whenever they are saved by the server.

## Prerequisites

- Docker and Docker Compose
- A [playit.gg](https://playit.gg) account with a tunnel configured

## Setup

### 1. Configure the playit.gg tunnel

In the playit.gg dashboard, set the tunnel's local destination to:

```
172.18.0.10:7777
```

> If you change `TCP_GUARD_IP` in your `.env`, use that IP here instead.

### 2. Create a `.env` file

Copy the example below and fill in your values:

```env
# Path to the directory containing your world files (required)
TERRARIA_WORLDS_DIR=/path/to/your/worlds/

# Filename of the world to load on startup (not required but the server will start in interactive mode without, needing you to run console.sh)
TERRARIA_WORLD=YourWorld.wld

# Your playit.gg agent secret key (required)
PLAYIT_SECRET_KEY=your_secret_key_here

# Directory where world backups will be stored (optional, defaults to /opt/terraria/backups)
# BACKUP_WORLDS_DIR=/path/to/your/backups/
```

### 3. Start the server

```bash
./run.sh
```

The script validates that all required environment variables are set, builds the images, and starts all containers.

---

## Environment Variables

### Terraria Server

| Variable | Required | Default | Description |
|---|---|---|---|
| `TERRARIA_WORLDS_DIR` | Yes | — | Host path mounted as the worlds directory. On Windows, use a `/mnt/c/...` path to point to a location on the Windows filesystem (e.g. `/mnt/c/Users/you/terraria-worlds`) |
| `TERRARIA_WORLD` | No | — | World filename to load on startup (e.g. `MyWorld.wld`). If unset the server starts in interactive mode |
| `TERRARIA_MAX_PLAYERS` | No | `16` | Maximum number of players allowed on the server |
| `TERRARIA_VERSION` | No | `1456` | Terraria server version to download at build time |

### Playit Tunnel

| Variable | Required | Default | Description |
|---|---|---|---|
| `PLAYIT_SECRET_KEY` | Yes | — | Secret key from the playit.gg dashboard |

### TCP Guard

| Variable | Required | Default | Description |
|---|---|---|---|
| `TCP_GUARD_PROXY_TIMEOUT` | No | `30m` | How long an idle connection is kept open. Use nginx time format: `30m`, `3h`, `1h30m` |
| `TCP_GUARD_CONNECT_TIMEOUT` | No | `5s` | How long to wait for the upstream (terraria-server) to accept a connection |
| `TCP_GUARD_PREREAD_TIMEOUT` | No | `5s` | How long to wait for the client to send data after connecting. Connections that never send data are dropped |

### Backups

| Variable | Required | Default | Description |
|---|---|---|---|
| `BACKUP_WORLDS_DIR` | No | `/opt/terraria/backups` | Host path where backup files will be written. On Windows, set this to a `/mnt/c/...` path to make backups accessible from the Windows filesystem (e.g. `/mnt/c/Users/you/terraria-backups`) |
| `MAX_SAVES` | No | `10` | Maximum number of backups to keep per world file. Oldest are deleted when the limit is reached |

### Docker Network

| Variable | Required | Default | Description |
|---|---|---|---|
| `TERRARIA_SERVER_IP` | No | `172.18.0.5` | Static IP assigned to the terraria-server container |
| `TCP_GUARD_IP` | No | `172.18.0.10` | Static IP assigned to the tcp-guard container. Set this as the tunnel destination in the playit.gg dashboard |
| `DOCKER_SUBNET` | No | `172.18.0.0/24` | Subnet for the internal Docker network. Change if it conflicts with your host network |

---

## Troubleshooting

### World files not found / backups not appearing (Windows)

On Windows, Docker paths must use the WSL2 `/mnt/c/` prefix to reference the Windows filesystem. Using a Windows-style path (e.g. `C:\Users\you\worlds`) will not work.

Set your paths in `.env` like this:

```env
TERRARIA_WORLDS_DIR=/mnt/c/Users/you/terraria-worlds
BACKUP_WORLDS_DIR=/mnt/c/Users/you/terraria-backups
```

If `BACKUP_WORLDS_DIR` is left at its default (`/opt/terraria/backups`), the backups are stored inside the WSL2 VM and can be accessed from Windows Explorer at `\\wsl$\docker-desktop-data` — but setting an explicit `/mnt/c/...` path is easier.

---

### Docker network subnet is already in use

Error during `docker compose up` mentioning the subnet is already in use by another network.

Set `DOCKER_SUBNET` in your `.env` to a free subnet:

```env
DOCKER_SUBNET=172.19.0.0/24
```

Also update `TERRARIA_SERVER_IP` and `TCP_GUARD_IP` to addresses within the new subnet. If you've previously started the stack you'll also need to remove the old network before restarting:

```bash
./remove.sh
docker network prune
./run.sh
```

---

### Players can't connect

1. Check the playit.gg dashboard — the tunnel destination must be `<TCP_GUARD_IP>:7777` (default `172.18.0.10:7777`)
2. Confirm all containers are running: `docker ps`
3. Check tcp-guard logs for refused connections: `docker logs tcp-guard`

---

### Server starts but loads the wrong world / starts in interactive mode

- Check `TERRARIA_WORLD` in `.env` matches the exact filename including extension (e.g. `MyWorld.wld`)
- Check `TERRARIA_WORLDS_DIR` points to the directory that contains that file
- Attach to the console with `./console.sh` to see the server output

---

### World files are not saved / missing after restart

Check that `TERRARIA_WORLDS_DIR` is set to a path that exists on the host and that Docker has permission to mount it.

---

### Backups are not being created

- Check that `BACKUP_WORLDS_DIR` is set and the path exists on the host
- Check backup container logs: `docker logs terraria-backups`
- The backup service only copies `.wld` files, triggered when the server saves the world

---

### tcp-guard fails to start

Check logs with `docker logs tcp-guard`. A common cause is an invalid value for one of the `TCP_GUARD_*` timeout variables — nginx requires a specific time format (e.g. `5s`, `30m`, `3h`). Plain numbers without a unit will cause nginx to reject the config.

---

## Other Scripts

| Script | Description |
|---|---|
| `run.sh` | Build and start all containers |
| `stop.sh` | Stop all containers |
| `restart.sh` | Restart all containers |
| `remove.sh` | Stop and remove all containers |
| `logs.sh` | Tail container logs |
| `console.sh` | Attach to the Terraria server console |
