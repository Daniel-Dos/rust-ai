---
description: Executar o projeto NATS Pub/Sub
agent: build
---

Verificar se o NATS Server está rodando. Se não estiver, iniciar:
`docker run --rm -d -p 4222:4222 --name nats-server nats`

Executar o Consumer (em outro terminal):
`cargo run --bin consumer`

Executar o Producer (em outro terminal):
`cargo run --bin producer -- --random`