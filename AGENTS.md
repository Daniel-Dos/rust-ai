# AGENTS.md

## Instructions

- Sempre considerar ai-context.md
- Priorizar simplicidade
- Não criar arquitetura complexa
- Deve criar testes unitários
- Executar `cargo test` após alterações

## Execução

### 1. Subir o NATS
```bash
docker run --rm -p 4222:4222 nats
```

### 2. Executar testes
```bash
cargo test
```

### 3. Executar consumer (em outro terminal)
```bash
cargo run --bin consumer
```

### 4. Executar producer (em outro terminal)
```bash
cargo run --bin producer "Hello, World!"
```

## Skills

- .opencode/skills/*