# Spec: REST API

## Objective
Criar API REST para enviar mensagens ao OpenCode e receber respostas.

## Endpoints

### POST /message
Envia mensagem, publica no NATS e envia ao OpenCode.

**Request:**
```json
{ "message": "string" }
```

**Response:**
```json
{ "message": "Message received successfully" }
```

### GET /message
Retorna a última resposta do OpenCode.

**Response:**
```json
{ "message": "resposta do OpenCode" }
```

## Behavior
- POST: pubblica no NATS + envía ao OpenCode + retorna confirmation
- GET: retorna última resposta do OpenCode

## Constraints
- Simples
- Sem autenticação
- Separado em `rest.rs`