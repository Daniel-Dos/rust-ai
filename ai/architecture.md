# Architecture

## Estrutura

```
src/
  producer.rs        # Entry point
  rest.rs            # REST API (axum)
  opencode_service.rs # OpenCode service
  consumer.rs       # Escuta NATS
```

## Fluxo

```
POST /message ──> OpenCode ──> NATS ──> Consumer
              (envia)      (resposta)
GET /message ──> retorna última resposta do OpenCode
```

## Detalhes

- O `rest.rs` é responsável por:
  - REST API (POST/GET /message)
  - Publicar mensagens no NATS

- O `opencode_service.rs` é responsável por:
  - Criar sessão no OpenCode
  - Enviar mensagens ao OpenCode
  - Receber e publicar respostas no NATS

- O `consumer.rs` escuta o NATS e exibe as mensagens

- Comunicação via NATS (demo.events)

## Separação de Responsabilidades

Arquitetura limpa seguindo SOLID:
- `rest.rs`: HTTP handlers (Single Responsibility)
- `opencode_service.rs`: Lógica OpenCode (Open/Closed)
- `producer.rs`: Entry point (Dependency Inversion)