# AI Context

## Arquitetura

Este é um exemplo simples de NATS pub/sub em Rust com dois binários:

- **producer**: Publica mensagens no subject `demo.events`
- **consumer**: Assina e imprime mensagens do subject `demo.events`

## Referências

- ai/specs/nats-pubsub.md
- ai/rules.md
- ai/architecture.md
- .opencode/skills/*

## Estrutura

```
src/
├── consumer.rs    # Assinante NATS
├── producer.rs   # Publicador NATS
└── main.rs       # Binário placeholder
```

## Fluxo

1. Producer serializa `Event` para JSON
2. Publica no subject `demo.events`
3. Consumer recebe e deserializa
4. Ambos usam `async-nats` com `tokio`