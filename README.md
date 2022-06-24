![Imersão Full Stack && Full Cycle](https://events-fullcycle.s3.amazonaws.com/events-fullcycle/static/site/img/grupo_4417.png)

![rust](https://github.com/fernandomarca/rust-codepix/blob/main/prisma/r.jpg)

## Sobre o repositório

Esse repositório contém o código do Simulador modulo da imersão 8 da FullCycle escrito em Rust para referência didáticas.

Se for útil para seus estudo Rust dê uma estrelinha.

O projeto conta com a integração:

- [x] Kafka/rdkafka.
- [x] serde_json.
- [x] serde.
- [x] tokio.

## Instruções de inicialização

1 - Ambos os projetos dependem do Apache-kafka para comunicação, então este é o primeiro serviço que deve ser inicializado. Para tal:
- entre no diretório do apache-kafka e execute "docker-compose up -d" pressupõe-se que tenha docker e docker-compose instalados.

2 - Por último execute o simulador. "cargo run"  
- Utilizamos Tokio runtime dependência para execução assíncrona e multi-threads.