# AGENTS.md

## Projeto: rust-ai

Exemplo simples de **NATS Pub/Sub** em Rust, demonstrando comunicação assíncrona entre processos.

## Instruções para IA

- Sempre considerar `ai-context.md`
- Priorizar simplicidade - não criar arquitetura complexa
- Deve criar testes unitários para novas funcionalidades
- Executar `cargo test` após alterações
- Seguir as skills em `.opencode/skills/*`

---

## Execução

### 1. Pré-requisitos

- Rust instalado (`rustc`, `cargo`)
- Docker para subir o NATS Server

### 2. Subir o NATS Server

```bash
docker run --rm -p 4222:4222 nats
```

O NATS estará disponível em `nats://localhost:4222`

### 3. Executar testes unitários

```bash
cargo test
```

### 4. Executar o Consumer (assina mensagens)

```bash
cargo run --bin consumer
```

Output esperado:
```
Listening on 'demo.events'...
```

### 5. Executar o Producer (publica mensagens)

Em outro terminal:

```bash
cargo run --bin producer "Hello, World!"
```

Output esperado:
```
Published: Hello, World!
```

O consumer receberá e exibirá:
```
Received: Hello, World!
```

---

## Estrutura do Projeto

```
src/
├── producer.rs    # Publica mensagens no NATS
├── consumer.rs    # Assina mensagens do NATS
└── main.rs        # Placeholder (não usado)
```

## Fluxo de Dados

```
Producer ── JSON ──> NATS Server ──> Consumer
                      (demo.events)
```

1. Producer cria um `Event { message }`
2. Serializa para JSON com `serde_json`
3. Publica no subject `demo.events`
4. Consumer recebe a mensagem
5. Deserializa e imprime o conteúdo

## Dependências Principais

- `async-nats`: Cliente NATS assíncrono
- `serde` + `serde_json`: Serialização JSON
- `tokio`: Runtime assíncrono
- `futures-util`: Utilitários para streams

## Skills Disponíveis

- `.opencode/skills/rust-basics/`: Boas práticas em Rust
- `.opencode/skills/nats-pubsub/`: Padrões NATS pub/sub
