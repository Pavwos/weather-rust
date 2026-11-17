# Instruções do projeto: Clima

App desktop pessoal de clima, feito em Rust com Tauri v2 e interface web local. Não é um projeto público, então priorize simplicidade e clareza em vez de over-engineering.

## Stack
- Rust edition 2021
- Desktop: Tauri v2
- UI: TypeScript estrito, HTML e CSS vanilla com Vite (sem framework)
- HTTP: `reqwest` (json, rustls-tls) com timeout de 10s
- Serialização: `serde`, `serde_json`, `toml`
- Erros: `anyhow` com `.context("...")` em português
- Datas: `chrono`
- API de clima: Open-Meteo (sem API key)

## Estrutura de arquivos
- `index.html`, `vite.config.ts`, `tsconfig.json`, `package.json`: entrada e configuração do frontend
- `src/main.ts`: inicialização, eventos e orquestração
- `src/api.ts`: wrappers tipados de `invoke()`
- `src/state.ts`: modelos e estado da tela
- `src/render.ts`: renderização do DOM
- `src/icons.ts`: ícones meteorológicos em SVG local
- `src/format.ts`: formatação com `Intl` em pt-BR
- `src/styles/`: tokens, estilos base e componentes
- `src-tauri/src/main.rs`: inicialização mínima que chama `lib::run()`
- `src-tauri/src/lib.rs`: builder Tauri e registro dos comandos
- `src-tauri/src/commands.rs`: comandos Tauri assíncronos
- `src-tauri/src/api.rs`: chamadas de geocodificação e forecast
- `src-tauri/src/models.rs`: modelos Serde da API e do frontend
- `src-tauri/src/config.rs`: leitura e escrita do `config.toml`
- `src-tauri/src/rain_alert.rs`: detecção determinística de chuva e verificação em segundo plano
- `src-tauri/src/weather_code.rs`: códigos WMO para descrição e chave de ícone

Respeite essa organização. Não crie arquivos novos sem necessidade; se criar, explique por quê.

## Regras de código
- Nunca use `unwrap()` ou `expect()` em Rust fora de testes.
- A UI nunca pode bloquear: chamadas ao backend usam `invoke()` assíncrono; rede fica nos comandos Tauri em Rust. Não faça chamadas de rede síncronas no frontend.
- TypeScript deve permanecer `strict`, sem `any`.
- Não use emoji na interface; use os SVGs locais do projeto.
- Não use CDN: fontes, scripts, estilos e ícones devem ser locais ou do sistema.
- Mantenha as permissões e capabilities do Tauri no mínimo necessário; não habilite plugins sem uso.
- Avisos de chuva usam o plugin oficial de notificações do Tauri e permanecem desativados por padrão. O agendamento de 15 minutos e as verificações após buscas manuais rodam no backend, inclusive com a janela oculta.
- A bandeja é criada no backend; com avisos ativos, fechar oculta a janela, e o menu permite abrir ou sair.
- Justifique dependências Rust novas: `tauri-plugin-notification` envia notificações nativas, `tokio` com `time` agenda verificações, e os recursos `tray-icon`/`image-png` do Tauri fornecem a bandeja com o ícone local.
- Funções pequenas, com uma responsabilidade só.
- Comentários curtos em português, só onde o código não é óbvio.
- Mensagens de erro e textos da interface sempre em português do Brasil.
- Não adicione dependências novas no Rust sem avisar e justificar.

## Fluxo de trabalho
- Em mudanças maiores, apresente um plano curto antes de implementar e espere minha confirmação.
- Faça uma coisa por vez. Não refatore código que não tem relação com o pedido.
- Antes de dizer que terminou, rode `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo build` e `cargo test` em `src-tauri/`, além de `npm run build`, e corrija o que falhar.
- Ao adicionar lógica nova, adicione testes unitários quando fizer sentido.
- Mantenha a versão do `src-tauri/Cargo.toml` coerente com as tags de release (ex: `v0.2.0` = `version = "0.2.0"`).
- Não altere o workflow de release do GitHub Actions sem pedido explícito.

## Estilo de resposta
- Fale em português, direto ao ponto.
- Ao final, resuma em poucas linhas o que foi alterado e em quais arquivos.
