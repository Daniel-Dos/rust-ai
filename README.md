# rust-ai

Exemplo simples de **NATS Pub/Sub** em Rust, demonstrando comunicação assíncrona entre processos.

## Como funciona

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│  Producer   │ ──────> │    NATS     │ ──────> │  Consumer   │
│  (publica)  │  JSON   │   Server    │  JSON   │  (assina)   │
└─────────────┘         └─────────────┘         └─────────────┘
                          demo.events
```

## Pré-requisitos

- Rust (rustc, cargo)
- Docker
- [opencode](https://opencode.ai) (opcional, para Assistance de IA)

## Instalação

```bash
# Clonar o repositório
git clone https://github.com/Daniel-Dos/rust-ai.git
cd rust-ai

# Build
cargo build
```

## Execução

### 1. Subir o NATS Server

```bash
docker run --rm -p 4222:4222 nats
```

### 2. Executar o Consumer (assina mensagens)

```bash
cargo run --bin consumer
```

Output esperado:
```
Listening on 'demo.events'...
```

### 3. Executar o Producer (publica mensagens)

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

## Testes

```bash
cargo test
```

## Usando com opencode

Este projeto inclui arquivos de configuração para assistentes de IA (`opencode`):

```bash
# Iniciar sessão com contexto do projeto
opencode

# Exemplo de comandos:
/review           # Revisar código
/build           # Compilar projeto
/test            # Executar testes
```

### Arquivos de configuração

- `AGENTS.md` - Instruções para IA
- `ai-context.md` - Contexto e arquitetura do projeto
- `.opencode/skills/` - Skills especializadas

## Estrutura do Projeto

```
src/
├── producer.rs    # Publica mensagens no NATS
├── consumer.rs    # Assina mensagens do NATS
└── main.rs        # Placeholder
```

## Tecnologias

- **async-nats**: Cliente NATS assíncrono
- **serde**: Serialização JSON
- **tokio**: Runtime assíncrono
