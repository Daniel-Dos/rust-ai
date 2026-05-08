# Spec: OpenCode Integration

## Objective
Integrar o Producer com o OpenCode para processar mensagens e retornar respostas.

## Behavior
- Ao receber POST /message, o producer envia a mensagem ao OpenCode via POST /session/:id/message
- O OpenCode processa e retorna a resposta
- A resposta do OpenCode é publicada no NATS para o consumer
- O GET /message retorna a última resposta recebida do OpenCode

## API Reference
- POST /session/:id/message - https://opencode.ai/docs/pt-br/server/#mensagens
- Body: `{ "parts": [{ "type": "text", "text": "mensagem" }] }`

## Constraints
- Simples
- Sem arquitetura complexa
- Separado em `opencode_service.rs`