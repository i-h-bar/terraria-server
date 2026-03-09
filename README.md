# Terraria Server

A Dockerised Terraria dedicated server exposed via a [playit.gg](https://playit.gg) tunnel, with a TCP guard proxy sitting between the tunnel and the server to limit abusive connections.

## Architecture

```
Internet → playit tunnel → tcp-guard (TCP_GUARD_IP) → terraria-server (TERRARIA_SERVER_IP)
```

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

# Filename of the world to load on startup (not rquired but the server will start in interactive mode without, needing you to run console.sh)
TERRARIA_WORLD=YourWorld.wld

# Your playit.gg agent secret key (required)
PLAYIT_SECRET_KEY=your_secret_key_here
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
| `TERRARIA_WORLDS_DIR` | Yes | — | Host path mounted as the worlds directory |
| `TERRARIA_WORLD` | No | — | World filename to load on startup (e.g. `MyWorld.wld`). If unset the server starts in interactive mode |
| `TERRARIA_MAX_PLAYERS` | No | `16` | Maximum number of players allowed on the server |
| `TERRARIA_VERSION` | No | `1455` | Terraria server version to download at build time |

### Playit Tunnel

| Variable | Required | Default | Description |
|---|---|---|---|
| `PLAYIT_SECRET_KEY` | Yes | — | Secret key from the playit.gg dashboard |

### TCP Guard

| Variable | Required | Default | Description |
|---|---|---|---|
| `TCP_GUARD_MAX_CONN` | No | `2` | Maximum simultaneous connections per source IP |
| `TCP_GUARD_PROXY_TIMEOUT` | No | `3h` | How long an idle connection is kept open. Use nginx time format: `30m`, `3h`, `1h30m` |
| `TCP_GUARD_CONNECT_TIMEOUT` | No | `5s` | How long to wait for the upstream (terraria-server) to accept a connection |
| `TCP_GUARD_PREREAD_TIMEOUT` | No | `5s` | How long to wait for the client to send data after connecting. Connections that never send data are dropped |

### Docker Network

| Variable | Required | Default | Description |
|---|---|---|---|
| `TERRARIA_SERVER_IP` | No | `172.18.0.5` | Static IP assigned to the terraria-server container |
| `TCP_GUARD_IP` | No | `172.18.0.10` | Static IP assigned to the tcp-guard container. Set this as the tunnel destination in the playit.gg dashboard |
| `DOCKER_SUBNET` | No | `172.18.0.0/24` | Subnet for the internal Docker network. Change if it conflicts with your host network |

---

## Troubleshooting

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

### Player gets immediately disconnected or can't reconnect after dropping

They may have hit the `TCP_GUARD_MAX_CONN` limit. This can happen if two players share the same public IP (same household) or a player reconnects before the old session fully closes.

Raise the limit in `.env`:

```env
TCP_GUARD_MAX_CONN=4
```

Then restart the tcp guard: `docker stop tcp-guard` and the `docker start tcp-guard`

---

### Server starts but loads the wrong world / starts in interactive mode

- Check `TERRARIA_WORLD` in `.env` matches the exact filename including extension (e.g. `MyWorld.wld`)
- Check `TERRARIA_WORLDS_DIR` points to the directory that contains that file
- Attach to the console with `./console.sh` to see the server output

---

### World files are not saved / missing after restart

Check that `TERRARIA_WORLDS_DIR` is set to a path that exists on the host and that Docker has permission to mount it.

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