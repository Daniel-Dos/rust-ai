# Architecture

## Estrutura

```
src/
  main.rs              # Entry point
  rest.rs             # Módulo REST
  rest/
    rest_api.rs      # REST API (axum)
  service.rs         # Módulo Service
  service/
    opencode_service.rs  # OpenCode service
  nats.rs            # Módulo NATS
  nats/
    producer.rs    # Producer NATS
    consumer.rs   # Consumer NATS
```

## Fluxo

```
POST /message ──> OpenCode ──> NATS ──> Consumer
              (envia)      (resposta)
GET /message ──> retorna última resposta do OpenCode
```

## Detalhes

- `main.rs`: Entry point que chama `rest_api::main()`
- `rest/rest_api.rs`: REST API (POST/GET /message)
- `service/opencode_service.rs`: Criar sessão, enviar/receber mensagens do OpenCode
- `nats/producer.rs`: Publicar no NATS
- `nats/consumer.rs`: Escuta NATS e exibe mensagens

- Comunicação via NATS (demo.events)

## Separação de Responsabilidades

Arquitetura limpa seguindo SOLID:
- `rest/rest_api.rs`: HTTP handlers (Single Responsibility)
- `service/opencode_service.rs`: Lógica OpenCode (Open/Closed)
- `main.rs`: Entry point (Dependency Inversion)