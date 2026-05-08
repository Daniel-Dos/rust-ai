# Architecture

## Estrutura

```
src/
  main.rs        # Placeholder
  producer.rs    # REST API + NATS + OpenCode
  consumer.rs    # Escuta NATS
```

## Fluxo

```
POST /message ──> OpenCode ──> NATS ──> Consumer
              (envia)      (resposta)
GET /message ──> retorna última resposta do OpenCode
```

## Detalhes

- O `producer.rs` é responsável por:
  - REST API (POST/GET /message)
  - Publicar mensagens no NATS
  - Enviar mensagens ao OpenCode
  - Receber e publicar respostas no NATS

- O `consumer.rs` escuta o NATS e exibe as mensagens

- Comunicação via NATS (demo.events)

## Componentes Integrados

Tudo está integrado no `producer.rs` para manter simples:
- REST API com axum
- Cliente NATS
- Cliente HTTP para OpenCode
- Gerenciamento de sessão