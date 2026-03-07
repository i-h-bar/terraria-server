#!/bin/bash

WORLD_DIR="/root/.local/share/Terraria/Worlds"
mkdir -p "$WORLD_DIR"

ARGS="-port ${TERRARIA_PORT:-7777} -maxplayers ${TERRARIA_MAX_PLAYERS:-16} -worldpath $WORLD_DIR"

if [ -n "$TERRARIA_WORLD" ]; then
    ARGS="$ARGS -world $WORLD_DIR/$TERRARIA_WORLD"
fi

exec ./TerrariaServer.bin.x86_64 $ARGS