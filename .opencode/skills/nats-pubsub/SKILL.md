---
name: nats-pubsub
description: Pub/Sub simples com NATS usando Rust
---

# NATS Pub/Sub

## When to use

- Comunicação assíncrona local
- Eventos simples

## Rules

- Producer publica no subject
- Consumer assina o subject
- Payload em JSON

## Rust

- Usar async-nats
- Usar tokio