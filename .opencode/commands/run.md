---
description: Executar o projeto NATS Pub/Sub com OpenCode
agent: build
---

Execute os testes do projeto:

1. Verificar se NATS está rodando (iniciar se necessário):
```bash
docker ps --filter "name=nats-server" || docker run --rm -d -p 4222:4222 --name nats-server nats
docker ps --filter "name=nats-server"
```

2. Verificar se OpenCode Server está rodando (na porta 4096):
```bash
curl -s http://localhost:4096/health || echo "OpenCode não está rodando na porta 4096"
```

3. Executar cargo test (testes unitários):
```bash
cargo test
```

4. Executar teste de integração producer/consumer:
```bash
# Iniciar consumer em background
cargo run --bin consumer &
CONSUMER_PID=$!
sleep 2

# Executar producer com mensagem específica
cargo run --bin producer "Test from CI!"

# Aguardar consumer receber
sleep 2

# Verificar logs (deve mostrar "Received: Test from CI!")

# Limpar
kill $CONSUMER_PID 2>/dev/null || true
```

5. Verificar resultado esperado:
```
Output deve conter:
- Published to NATS: Test from CI!
- Received: Test from CI!
```

IMPORTANTE: Use exatamente "Test from CI!" como mensagem, não invente outras mensagens.