# Clima

Aplicativo desktop pessoal para consultar o clima atual e a previsão de cinco dias pela API Open-Meteo. O backend é Rust com Tauri v2; a interface é TypeScript, CSS e Vite, sem framework.

## Pré-requisitos

- Rust e a toolchain MSVC do Visual Studio Build Tools
- Node.js LTS e npm
- WebView2 Runtime (normalmente já instalado no Windows 10 e 11)

## Desenvolvimento

Na raiz do projeto:

```sh
npm install
npm run tauri dev
```

## Gerar o aplicativo

```sh
npm run tauri build
```

No Windows, o executável fica em `src-tauri/target/release/clima.exe`. O instalador NSIS fica em `src-tauri/target/release/bundle/nsis/`.

## Configuração

As preferências guardam cidade padrão, última cidade pesquisada e unidade. No Windows, o arquivo existente continua em `%APPDATA%\weather-cli\config.toml`.

As chamadas de geocodificação e previsão usam a Open-Meteo e precisam de internet. A interface e os recursos visuais são empacotados localmente.
