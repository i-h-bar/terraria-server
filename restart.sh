#!/bin/bash

docker stop terraria-server
echo "Stopped"

docker start terraria-server
echo "Started"
