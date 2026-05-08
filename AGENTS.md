# AGENTS.md

## Projeto: rust-ai

Exemplo de **NATS Pub/Sub** em Rust com REST API e integração com OpenCode.

## Instruções para IA

- Sempre considerar `ai-context.md`
- Priorizar simplicidade - não criar arquitetura complexa
- Deve criar testes unitários para novas funcionalidades
- Executar `cargo test` após alterações
- Seguir as skills em `.opencode/skills/*`
- Seguir as specs em `ai/specs/*`

---

## Execução

### 1. Pré-requisitos

- Rust instalado (`rustc`, `cargo`)
- Docker para subir o NATS Server
- OpenCode instalado (`opencode`)

### 2. Subir os Serviços

```bash
# Terminal 1: NATS Server
docker run --rm -p 4222:4222 nats

# Terminal 2: OpenCode Server
opencode serve --port 4096
```

### 3. Executar testes unitários

```bash
cargo test
```

### 4. Executar o Consumer (assina mensagens do NATS)

```bash
cargo run --bin consumer
```

### 5. Executar o Producer (REST API + Publica no NATS)

```bash
cargo run --bin producer
```

---

## API Endpoints

### POST /message
Envia mensagem e publicação no NATS.

```bash
curl -X POST http://localhost:8080/message \
  -H "Content-Type: application/json" \
  -d '{"message": "Olá!"}'
```

Resposta:
```json
{"message": "Message received successfully"}
```

### GET /message
Retorna a última resposta do OpenCode.

```bash
curl http://localhost:8080/message
```

---

## Fluxo de Dados

```
POST /message ──> OpenCode ──> NATS ──> Consumer
              (envia)      (resposta)
```

1. Client envía POST com mensagem
2. Producer publica no NATS
3. Producer envía mensagem ao OpenCode
4. OpenCode retorna resposta
5. Producer publica resposta no NATS
6. Consumer recebe e exibe resposta

---

## Estrutura do Projeto

```
src/
├── producer.rs        # Entry point
├── rest.rs          # REST API (axum)
├── opencode_service.rs  # OpenCode service
└── consumer.rs      # Assina mensagens do NATS
```

---

## Dependências Principais

- `async-nats`: Cliente NATS assíncrono
- `serde` + `serde_json`: Serialização JSON
- `tokio`: Runtime assíncrono
- `axum`: Framework HTTP

---

## Skills Disponíveis

- `.opencode/skills/rust-basics/`: Boas práticas em Rust
- `.opencode/skills/nats-pubsub/`: Padrões NATS pub/sub
- `.opencode/skills/rest-api/`: Construção de APIs REST