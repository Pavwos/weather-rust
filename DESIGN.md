---
name: Clima
description: Leitura rápida do tempo com uma atmosfera que acompanha o céu.
world: "digital-design-canon-dark-first-developer-console"
colors:
  dark-background: "#101820"
  light-background: "#e8eff0"
  deep-slate: "#253a4a"
  ink-dark: "#e8eef0"
  ink-light: "#17252d"
  muted-dark: "#b6c4ca"
  muted-light: "#40545e"
  accent-dark: "#edcc82"
  accent-light: "#795512"
  rain-dark: "#8fc9d4"
  rain-light: "#176f81"
  action-dark: "#d7e5e8"
  action-ink-dark: "#18232c"
typography:
  display:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(3.75rem, 19vw, 4.5rem)"
    fontWeight: 400
    lineHeight: 1.05
    letterSpacing: "-0.045em"
  headline:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(1.75rem, 7vw, 2.3rem)"
    fontWeight: 560
    lineHeight: 1.08
    letterSpacing: "-0.04em"
  title:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.1rem"
    fontWeight: 650
    letterSpacing: "-0.04em"
  body:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.45
  label:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.75rem"
  action:
    fontFamily: "IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 650
rounded:
  sm: "0.6rem"
  pill: "999px"
spacing:
  space-1: "0.25rem"
  space-2: "0.5rem"
  space-3: "0.75rem"
  space-4: "1rem"
  space-5: "1.25rem"
  space-6: "1.5rem"
  space-8: "2rem"
  space-10: "2.5rem"
components:
  button-primary:
    backgroundColor: "{colors.action-dark}"
    textColor: "{colors.action-ink-dark}"
    rounded: "{rounded.sm}"
    typography: "{typography.action}"
    padding: "0 1rem"
    height: "2.75rem"
  button-unit-active:
    backgroundColor: "{colors.action-dark}"
    textColor: "{colors.action-ink-dark}"
    rounded: "{rounded.pill}"
    typography: "{typography.body}"
    padding: "0 0.5rem"
    height: "2rem"
  input-search:
    backgroundColor: "{colors.dark-background}"
    textColor: "{colors.ink-dark}"
    rounded: "{rounded.sm}"
    typography: "{typography.body}"
    padding: "0 1rem"
    height: "2.75rem"
---

# Design System: Clima

## Overview

**Creative North Star: "Leitura do céu"**

Clima é uma interface Operate para consultar o tempo de relance. A composição prioriza a temperatura atual, mantém condição e métricas compactas e apresenta a previsão como uma lista cronológica, sem cartões aninhados. A linguagem é calma e funcional: grafite, separadores finos e controles familiares.

A atmosfera é um campo radial discreto, ligado à condição atual e ao período do dia. Ela colore o ambiente sem deslocar conteúdo nem competir com temperatura ou precipitação. A IBM Plex Sans variável é empacotada localmente; os pictogramas meteorológicos e de ação são SVGs inline.

**Key Characteristics:**
- Temperatura como foco visual dominante; chuva fácil de localizar.
- Atmosfera de fundo sensível ao céu, inclusive à diferença entre dia e noite.
- Lista meteorológica direta, com separadores finos em vez de cartões.
- Uma única família tipográfica local e ícones SVG de traço.

## Colors

A paleta combina um grafite azulado escuro e uma alternativa clara com texto de alto contraste; dourado sinaliza o destaque e azul-ciano identifica chuva.

### Primary
- **Grafite de observação** (#101820): fundo base do app-shell e tema escuro.
- **Lousa atmosférica** (#253a4a): centro profundo do gradiente escuro de fundo.
- **Destaque de luz** (#edcc82): ícones meteorológicos e acento de busca no tema escuro.

### Secondary
- **Chuva fria** (#8fc9d4): probabilidade de chuva e foco visível no tema escuro.
- **Destaque diurno** (#795512): acento correspondente no tema claro.
- **Chuva diurna** (#176f81): destaque de precipitação e foco no tema claro.

### Neutral
- **Névoa clara** (#e8eff0): fundo do tema claro.
- **Branco atmosférico** (#e8eef0): texto principal no tema escuro.
- **Grafite de leitura** (#17252d): texto principal no tema claro.
- **Cinza de nuvem** (#b6c4ca / #40545e): texto secundário nos temas escuro e claro.
- **Superfície de ação** (#d7e5e8): ação selecionada no tema escuro; no tema claro, a superfície ativa inverte para #1d3b46 com texto #f0f7f8.
- **Linhas e controles:** bordas usam `rgb(194 218 227 / 18%)` no escuro e `rgb(31 66 78 / 22%)` no claro; superfícies de controle permanecem translúcidas ou tonais.
- **Estados:** erro usa #ffd1c9 sobre `rgb(102 46 41 / 52%)` no escuro e #762f24 sobre `rgb(255 226 219 / 88%)` no claro; aviso usa #c6e7c7 / #245c32.

**The Atmosphere-Follows-the-Sky Rule.** A luz radial muda com a condição e o período do dia, mas permanece sutil e atrás dos dados.

## Typography

- **Display Font:** IBM Plex Sans (com ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont e Segoe UI como fallbacks)
- **Body Font:** IBM Plex Sans (com a mesma pilha local de fallbacks)

**Character:** Uma família variável sem serifa mantém a leitura coesa e direta. Os números meteorológicos usam algarismos tabulares; tamanhos e pesos variam por função, não por troca de família.

### Hierarchy
- **Display** (400, `clamp(3.75rem, 19vw, 4.5rem)`, 1.05): temperatura atual, com unidade menor alinhada ao topo e espaço inferior para evitar o corte dos algarismos.
- **Headline** (560, `clamp(1.75rem, 7vw, 2.3rem)`, 1.08): título de boas-vindas responsivo.
- **Title** (650, 1.1rem, entrelinha padrão): nome da cidade; títulos de seção usam 0.95rem e peso 650.
- **Body** (400, 0.875rem, 1.45): texto geral; descrições e condições descem para escalas compactas.
- **Label** (600, 0.75rem): ações, máximas/mínimas e rótulos compactos; metadados variam de 0.67rem a 0.82rem.

**The Temperature-First Rule.** Preserve a temperatura como único foco dominante; condição, métricas e previsão ficam em níveis tipográficos menores.

## Layout

O topo mantém marca e controles de unidade/atualização, seguido pela busca. Em telas estreitas, a leitura atual precede a previsão em uma coluna; a faixa de métricas mantém três colunas e os horários do sol duas, com rótulo e horário agrupados verticalmente. A partir de 720px, `.weather-view` muda para duas colunas fluidas (0.88fr / 1.12fr), com intervalo responsivo de 1.5rem a 2.5rem. O conteúdo principal tem largura máxima de 72rem.

No mobile, o app-shell usa 0.5rem no topo, 1rem nas laterais e 0.75rem na base; no desktop, passa a 2rem no topo e laterais fluidas entre 2rem e 6rem. O ritmo vem da escala de 0.25rem a 2.5rem. A previsão é uma lista em linhas de pelo menos 2.5rem no mobile e 3.75rem no desktop, com um separador superior por dia.

A faixa "Próximas horas" fica antes da lista diária e rola horizontalmente em uma única linha, com itens de largura compacta e alinhamento por snap. Em telas estreitas, mantém a largura do conteúdo e revela os horários seguintes por rolagem, sem comprimir os dados meteorológicos.

## Elevation & Depth

O sistema não usa sombras. A profundidade vem do gradiente atmosférico radial, da opacidade das superfícies de controle e de bordas discretas; os dados permanecem planos, sem painéis ou cartões elevados.

## Shapes

Controles e campos usam cantos suaves de 0.6rem; alternância de unidades e botões de ícone usam silhueta pill (999px). Bordas de um pixel separam linhas e campos. A previsão não é recortada em cartões; os SVGs usam traço arredondado, sem preenchimentos decorativos.

## Components

### Buttons
- **Caráter:** ações compactas, familiares e claramente selecionáveis.
- **Shape:** raio de 0.6rem; unidade ativa é pill.
- **Primary:** busca usa fundo ativo, texto contrastante, altura 2.75rem e padding horizontal de 1rem.
- **Hover / Focus:** busca aumenta brilho no hover; controles mudam cor/superfície em 180ms; foco visível usa contorno de 2px da cor de chuva com offset de 3px.
- **Ghost:** atualizar usa botão circular transparente de 2.4rem, com ícone SVG de 1.25rem.

### Inputs / Fields
- **Style:** busca ocupa a largura disponível, altura 2.75rem, padding horizontal de 1rem, fundo de controle e borda de 1px.
- **Focus:** contorno visível de 2px na cor de chuva, offset de 3px; hover desloca a borda na direção da cor de chuva.
- **Error / Disabled:** mensagens de erro têm superfície própria; durante carregamento, o campo e as ações preservam o estado do app e os botões podem ser desativados.

### Navigation
- **Style:** cabeçalho horizontal com wordmark à esquerda e alternância de unidades/atualização à direita; controles permanecem compactos em telas estreitas.

### Weather Reading
- **Current conditions:** cidade e localidade precedem condição e temperatura; um SVG meteorológico de traço acompanha a leitura atual. Uma frase determinística resume o dia logo abaixo da temperatura, em texto secundário de até duas linhas.
- **Metrics:** sensação, umidade e vento formam uma grade de três colunas; nascer e pôr do sol ficam em dois grupos com rótulo e horário.
- **Hourly forecast:** faixa horizontal sem cartões, com horário, ícone, temperatura e precipitação relevante; tem foco visível, responde às setas e converte a roda vertical em rolagem horizontal.
- **Forecast row:** data, ícone SVG, condição, chance de chuva e temperaturas máxima/mínima aparecem na mesma linha, separadas por borda superior; a mínima usa cor secundária e um divisor sutil.
- **Sky atmosphere:** a condição do céu no elemento raiz escolhe o gradiente; variantes diurnas são quentes e noturnas frias, com paleta própria em cada tema.

## Do's and Don'ts

### Do:
- **Do** manter temperatura como foco dominante e precipitação facilmente escaneável.
- **Do** usar IBM Plex Sans empacotada, números tabulares e SVGs inline para ícones.
- **Do** conservar a hierarquia em uma coluna no mobile e em duas colunas a partir de 720px.
- **Do** manter o campo atmosférico atrás dos dados e ajustar sua tonalidade ao céu e ao tema.

### Don't:
- **Don't** deixar a atmosfera competir com a temperatura ou com a previsão.
- **Don't** converter a previsão em cartões aninhados; a gramática implementada é uma lista com separadores.
- **Don't** introduzir uma segunda família tipográfica ou depender de fonte remota.
