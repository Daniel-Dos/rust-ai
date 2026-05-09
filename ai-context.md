# AI Context

## Visão Geral

Este projeto demonstra **pub/sub com NATS** em Rust integrado com REST API e OpenCode:
- **Producer** = REST API que recebe POST, envia ao OpenCode e publica no NATS
- **Consumer** = escuta o NATS e exibe as respostas
- **NATS Server** = message broker
- **OpenCode** = processa as mensagens e retorna respostas

## Arquitetura

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│  REST      │ ──────> │  OpenCode  │ ──────> │    NATS    │ ──────> │  Consumer  │
│  POST      │         │ (processa) │          │         │           │
└─────────────┘         └─────────────┘         └─────────────┘         └─────────────┘
  localhost:8080         localhost:4096        demo.events
```

### Componentes

| Componente | Arquivo | Responsabilidade |
|------------|---------|------------------|
| Producer   | `src/rest.rs` | REST API + Publica no NATS + Chama OpenCode |
| OpenCode   | `src/opencode_service.rs` | Lógica OpenCode (sessão, mensagens) |
| Consumer   | `src/consumer.rs` | Assina NATS e exibe mensagens |
| NatsEvent  | struct | `{ message: String }` serializado como JSON |

## Fluxo de Dados

1. Client envía POST `/message` com `{ "message": "Olá!" }`
2. Producer publica no NATS (para o consumer)
3. Producer envía mensagem ao OpenCode via POST `/session/:id/message`
4. OpenCode retorna resposta
5. Producer publicar resposta no NATS
6. Consumer recebe e exibe

## API Endpoints

### POST /message
```json
{ "message": "Olá!" }
```
Retorna:
```json
{ "message": "Message received successfully" }
```

### GET /message
Retorna a última resposta do OpenCode:
```json
{ "message": "resposta do OpenCode" }
```

## Estrutura de Arquivos

```
rust-ai/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point
│   ├── rest.rs             # Módulo REST
│   ├── rest/
│   │   └── rest_api.rs    # REST API (axum)
│   ├── service.rs         # Módulo Service
│   ├── service/
│   │   └── opencode_service.rs  # OpenCode service
│   ├── nats.rs            # Módulo NATS
│   ├── nats/
│   │   ├── producer.rs   # Producer NATS
│   │   └── consumer.rs  # Consumer NATS
└── AGENTS.md
```

## Implementação

### Producer
- POST /message: publica no NATS + chama OpenCode + retorna confirmation
- GET /message: retorna última resposta do OpenCode

### Consumer
- Conecta ao NATS em `demo.events`
- Desserializa e exibe mensagens

### Serialização
```rust
#[derive(Serialize, Deserialize)]
struct NatsEvent {
    message: String,
}
```

## Referências

- **NATS**: https://nats.io
- **OpenCode**: https://opencode.ai/docs/pt-br/server/#mensagens
- **async-nats**: https://github.com/nats-io/async-nats