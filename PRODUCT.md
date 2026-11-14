# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Uso pessoal no desktop, consultado rapidamente várias vezes ao dia para conferir a temperatura e se vai chover. Interpretei “app pessoal” como prioridade para uma pessoa; não há necessidade de múltiplas contas confirmada.

## Product Purpose

Mostrar o clima atual e a previsão de cinco dias para uma cidade, facilitando decisões cotidianas de relance.

## Positioning

Aplicativo desktop local que consulta geocodificação e previsão da Open-Meteo sem exigir chave de API.

## Operating Context

O foco é Windows. A interface HTML/CSS/TypeScript é empacotada no WebView do Tauri, não servida como site. A internet é necessária para buscar o clima; a interface e os recursos visuais são locais.

## Capabilities and Constraints

- Busca de cidades, clima atual, detalhes meteorológicos e previsão para cinco dias.
- Alternância entre unidades métricas e imperiais, atualização manual e automática, e cidade padrão.
- Preferências em `config.toml`, preservando o caminho existente de `ProjectDirs::from("", "", "weather-cli")`.
- API de clima Open-Meteo, sem chave de API. Não há geolocalização automática confirmada.
- Interface em português do Brasil, sem dependências de CDN. Não adicionar dependências Rust além das necessárias ao Tauri sem justificativa.

## Brand Commitments

O nome é Clima. O tom solicitado é calmo, limpo e agradável. A interface não usa emoji.

## Evidence on Hand

- A implementação Rust existente de geocodificação, forecast, cache e configuração é a fonte de comportamento a preservar.
- `assets/icon.png` é o ícone fornecido e deve permanecer no projeto.
- Não há depoimentos, métricas de produto ou outras alegações comerciais fornecidas.

## Product Principles

- Temperatura e chance de chuva são as informações mais rápidas de encontrar.
- Mostrar estados atuais com clareza e manter os dados disponíveis em falhas de atualização.
- Preservar as preferências locais sem exigir cadastro.
- Usar dados e alegações apenas quando fornecidos pelo serviço ou pelo usuário.

## Accessibility & Inclusion

Atender ao contraste WCAG AA, manter foco de teclado visível e nomes acessíveis nos controles, usar alvos clicáveis de pelo menos 32 px e respeitar `prefers-reduced-motion`.
