#!/bin/bash
set -e

echo "=== Verificando NATS ==="
if ! docker ps --filter "name=nats-server" | grep -q nats-server; then
    echo "Iniciando NATS..."
    docker run --rm -d -p 4222:4222 --name nats-server nats
fi

echo "=== Rodando Producer e Consumer ==="
cargo run --bin producer &
PRODUCER_PID=$!

cargo run --bin consumer &
CONSUMER_PID=$!

echo "Producer PID: $PRODUCER_PID"
echo "Consumer PID: $CONSUMER_PID"
echo "=== Pronto! ==="
echo "API: http://localhost:8080"

sleep 2

echo "=== Testando API ==="
curl -X POST http://localhost:8080/message \
  -H "Content-Type: application/json" \
  -d '{"message":"Olá!"}'

echo ""
echo "=== Teste concluído ==="

wait