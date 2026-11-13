# Instruções do projeto: Clima

App desktop pessoal de clima, feito em Rust com interface gráfica (egui/eframe). Não é um projeto público, então priorize simplicidade e clareza em vez de over-engineering.

## Stack
- Rust edition 2021
- UI: `eframe` + `egui`
- HTTP: `reqwest` (json, rustls-tls) com timeout de 10s
- Async: `tokio` (runtime guardado dentro do `ClimaApp`)
- Serialização: `serde`, `serde_json`, `toml`
- Erros: `anyhow` com `.context("...")` em português
- Datas: `chrono`
- API de clima: Open-Meteo (sem API key)

## Estrutura de arquivos
- `main.rs`: configura a janela e inicia o eframe
- `app.rs`: struct `ClimaApp` e `impl eframe::App`
- `ui.rs`: funções que desenham cada parte da tela
- `api.rs`: chamadas HTTP (geocoding e forecast)
- `models.rs`: structs Serde das respostas da API
- `config.rs`: leitura e escrita do config.toml
- `weather_code.rs`: códigos WMO para descrição em PT-BR e emoji
- `theme.rs`: cores, fontes e estilo visual

Respeite essa organização. Não crie arquivos novos sem necessidade; se criar, explique por quê.

## Regras de código
- Nunca use `unwrap()` ou `expect()` fora de testes.
- A UI nunca pode bloquear: chamadas de rede sempre em task do tokio, com resultado enviado por canal e `ctx.request_repaint()` ao terminar.
- Funções pequenas, com uma responsabilidade só.
- Comentários curtos em português, só onde o código não é óbvio.
- Mensagens de erro e textos da interface sempre em português do Brasil.
- Não adicione dependências novas sem avisar e justificar.

## Fluxo de trabalho
- Em mudanças maiores, apresente um plano curto antes de implementar e espere minha confirmação.
- Faça uma coisa por vez. Não refatore código que não tem relação com o pedido.
- Antes de dizer que terminou, rode `cargo fmt`, `cargo clippy` (sem warnings), `cargo build` e `cargo test`, e corrija o que falhar.
- Ao adicionar lógica nova, adicione testes unitários quando fizer sentido.
- Mantenha a versão do `Cargo.toml` coerente com as tags de release (ex: `v0.1.0` = `version = "0.1.0"`).

## Estilo de resposta
- Fale em português, direto ao ponto.
- Ao final, resuma em poucas linhas o que foi alterado e em quais arquivos.