#!/bin/bash

echo "To exit press ctrl+P + ctrl+Q. DO NOT use ctrl+C as that will stop the server."
docker logs terraria-server
docker attach terraria-server
