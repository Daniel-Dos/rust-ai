---
description: Executar o projeto NATS Pub/Sub com OpenCode
agent: build
---

Execute os testes completos do projeto. IMPORTANTE: Execute TODOS os passos abaixo na ordem EXATA:

## Step 1: Verificar NATS
```bash
docker ps --filter "name=nats-server" || docker run --rm -d -p 4222:4222 --name nats-server nats
docker ps --filter "name=nats-server"
```

## Step 2: Verificar OpenCode Server
```bash
curl -s http://localhost:4096/health || echo "OpenCode não está rodando"
```

## Step 3: Executar testes unitários (OBRIGATÓRIO)
```bash
cargo test
```
Espere o resultado. Todos os testes devem passar.

## Step 4: Teste de integração producer/consumer (OBRIGATÓRIO)
Execute exatamente esta sequência:
```bash
cargo run --bin consumer &
CONSUMER_PID=$!
sleep 3
cargo run --bin producer "Test from CI!"
sleep 2
kill $CONSUMER_PID 2>/dev/null || true
```

## Step 5: Verificar output
O output deve mostrar:
- `Published to NATS: Test from CI!`
- `Received: Test from CI!`

## Step 6: Resultado esperado
Retorne um resumo com:
- Quantidade de testes unitários que passaram
- Se a mensagem "Test from CI!" foi publicada e recebida corretamente

NÃO invente mensagens diferentes. Use exatamente "Test from CI!"