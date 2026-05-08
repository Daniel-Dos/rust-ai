---
description: Executar o projeto NATS Pub/Sub com OpenCode
agent: build
---

Execute os testes do projeto:

1. Iniciar NATS (se não estiver rodando):
```bash
docker ps --filter "name=nats-server" || docker run --rm -d -p 4222:4222 --name nats-server nats
```

2. Executar producer em background:
```bash
cargo run --bin producer &
sleep 2
```

3. Executar consumer em background:
```bash
cargo run --bin consumer &
sleep 2
```

4. Testar API:
```bash
curl -X POST http://localhost:8080/message \
  -H "Content-Type: application/json" \
  -d '{"message":"Test from CI!"}'
```

5. Verificar se o consumer recebeu a mensagem:
```bash
# O output deve mostrar "Received: Test from CI!"
```

6. Executar testes unitários:
```bash
cargo test
```

7. Limpar processos:
```bash
pkill -f "cargo run" || true
```