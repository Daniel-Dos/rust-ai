---
name: rest-api
description: Boas práticas para desenvolvimento de APIs RESTful
---

# REST API

## When to use
- Comunicação síncrona entre cliente e servidor
- Operações CRUD (Create, Read, Update, Delete)
- Integração com front-end ou outros serviços
## Rules
- Usar verbos HTTP corretamente (GET, POST, PUT, DELETE)
- Utilizar URLs semânticas e consistentes
- Retornar códigos de status HTTP apropriados
- Documentar a API usando OpenAPI ou similar
## Rust
- Usar frameworks mais simples somente para post e get 
- Utilizar serde para serialização/deserialização de JSON
- Implementar testes para endpoints da API

## Style
- Código simples
- Funções pequenas