FROM debian:bookworm-slim

ARG TERRARIA_VERSION=1456

WORKDIR /terraria

RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    && rm -rf /var/lib/apt/lists/*

RUN wget -O terraria-server.zip https://terraria.org/api/download/pc-dedicated-server/terraria-server-${TERRARIA_VERSION}.zip \
    && unzip terraria-server.zip \
    && cp -r ${TERRARIA_VERSION}/Linux/. . \
    && rm -rf terraria-server.zip ${TERRARIA_VERSION} \
    && chmod +x TerrariaServer.bin.x86_64

EXPOSE 7777

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

CMD ["/entrypoint.sh"]