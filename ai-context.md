# AI Context

## Visão Geral

Este projeto demonstra **pub/sub com NATS** em Rust, um padrão de mensageria onde:
- **Publisher** envia mensagens para um topic (subject)
- **Subscriber** recebe todas as mensagens desse topic
- NATS Server atua como intermediário (message broker)

## Arquitetura

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│  Producer   │ ──────> │    NATS     │ ──────> │  Consumer   │
│  (publica)  │  JSON   │   Server    │  JSON   │  (assina)   │
└─────────────┘         └─────────────┘         └─────────────┘
                          demo.events
```

### Componentes

| Componente | Arquivo | Responsabilidade |
|------------|---------|------------------|
| Producer   | `src/producer.rs` | Serializa eventos e publica no NATS |
| Consumer   | `src/consumer.rs` | Assina subject e deserializa eventos |
| Event      | struct em ambos | `{ message: String }` serializado como JSON |

## Estrutura de Arquivos

```
rust-ai/
├── Cargo.toml          # Dependências do projeto
├── src/
│   ├── producer.rs      # Binary: publica mensagens
│   ├── consumer.rs      # Binary: recebe mensagens
│   └── main.rs          # Placeholder (não usado)
├── ai/
│   ├── specs/           # Especificações
│   │   └── nats-pubsub.md
│   ├── rules.md         # Regras de código
│   └── architecture.md  # Decisões arquiteturais
├── .opencode/skills/    # Skills para assistentes
└── AGENTS.md            # Este arquivo
```

## Detalhes de Implementação

### Producer
- Conecta ao NATS em `nats://localhost:4222`
- Lê mensagem do argumento ou usa default "Hello NATS!"
- Serializa para JSON
- Publica no subject `demo.events`
- Faz `flush()` para garantir entrega

### Consumer
- Conecta ao NATS
- Assina o subject `demo.events`
- Fica em loop infinito esperando mensagens
- Desserializa JSON e imprime

### Serialização
```rust
// Event struct compartilhado (definido em ambos arquivos)
#[derive(Serialize, Deserialize)]
struct Event {
    message: String,
}
```

## Referências e Recursos

- **NATS**: https://nats.io
- **async-nats**: https://github.com/nats-io/async-nats
- **Specs**: `ai/specs/nats-pubsub.md`
- **Regras de Código**: `ai/rules.md`
- **Decisões Arquiteturais**: `ai/architecture.md`
- **Skills**: `.opencode/skills/*`
