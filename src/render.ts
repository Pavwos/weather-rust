import { actionIcon, weatherIcon } from "./icons";
import {
  firstDay,
  formatForecastDay,
  formatHour,
  formatPercent,
  formatTemperature,
  formatTime,
  formatUpdatedAt,
  formatWind,
  locationLabel,
} from "./format";
import type { AppState, Units, WeatherData } from "./state";

export function renderApp(root: HTMLElement, state: AppState): void {
  const weather = state.weather;
  const sky = weather?.current.icon ?? "cloudy";
  const displayedUnits =
    state.phase === "loading" && weather ? weather.units : state.units;
  root.innerHTML = `
    <div class="app-shell" data-sky="${escapeHtml(sky)}">
      <header class="topbar">
        <div class="wordmark">Clima</div>
        <div class="toolbar" aria-label="Ações do clima">
          <div class="unit-switch" role="group" aria-label="Unidade de temperatura">
            <button type="button" class="unit-button ${displayedUnits === "metric" ? "is-active" : ""}" data-action="units" data-units="metric" aria-pressed="${displayedUnits === "metric"}" ${state.phase === "loading" ? "disabled" : ""}>°C</button>
            <button type="button" class="unit-button ${displayedUnits === "imperial" ? "is-active" : ""}" data-action="units" data-units="imperial" aria-pressed="${displayedUnits === "imperial"}" ${state.phase === "loading" ? "disabled" : ""}>°F</button>
          </div>
          <button type="button" class="icon-button" data-action="refresh" aria-label="Atualizar clima" ${state.phase === "loading" ? "disabled" : ""}>
            ${actionIcon("refresh")}
          </button>
        </div>
      </header>

      <form class="search-form" id="search-form">
        <label class="sr-only" for="city-search">Buscar cidade</label>
        <input id="city-search" name="city" type="search" autocomplete="off" placeholder="Buscar cidade" value="${escapeHtml(state.cityQuery)}" />
        <button class="search-button" type="submit" ${state.phase === "loading" ? "disabled" : ""}>
          ${state.phase === "loading" ? '<span class="spinner" aria-hidden="true"></span>' : ""}
          <span>${state.phase === "loading" ? "Buscando" : "Buscar"}</span>
        </button>
      </form>

      <div class="live-message" aria-live="polite" aria-atomic="true">
        ${state.error ? `<p class="message message-error" role="alert">${escapeHtml(state.error.message)}</p>` : ""}
        ${state.notice ? `<p class="message message-notice" role="status">${escapeHtml(state.notice)}</p>` : ""}
        ${state.phase === "loading" ? '<p class="loading-copy" role="status">Atualizando a previsão…</p>' : ""}
      </div>

      ${
        weather
          ? renderWeather(weather, state)
          : renderWelcome(state)
      }
    </div>
  `;
}

function renderWeather(weather: WeatherData, state: AppState): string {
  const current = weather.current;
  const today = firstDay(weather.daily);
  const location = locationLabel({
    admin1:
      weather.location.admin1 === weather.location.name
        ? null
        : weather.location.admin1,
    country: weather.location.country,
  });
  const updatedAt = formatUpdatedAt(weather.fetchedAt);
  const isStale = state.phase === "error";

  return `
    <main class="weather-view">
      <section class="current-weather" aria-labelledby="current-title">
        <div class="current-heading">
          <div>
            <h1 id="current-title">${escapeHtml(weather.location.name)}</h1>
            <p class="location">${escapeHtml(location)}</p>
          </div>
          <button class="save-city-button" type="button" data-action="save-city" aria-label="Salvar ${escapeHtml(weather.location.name)} como cidade padrão">
            ${actionIcon("bookmark")}
            <span>Salvar padrão</span>
          </button>
        </div>

        <div class="condition-row">
          <div class="temperature-block">
            <p class="condition">${escapeHtml(current.description)}</p>
            <p class="temperature">${formatTemperature(current.temperature)}<span>${weather.units === "imperial" ? "°F" : "°C"}</span></p>
          </div>
          ${weatherIcon(current.icon, "current-icon")}
        </div>

        <dl class="weather-metrics">
          <div>
            <dt>Sensação</dt>
            <dd>${formatTemperature(current.apparentTemperature)}°</dd>
          </div>
          <div>
            <dt>Umidade</dt>
            <dd>${formatPercent(current.humidity)}</dd>
          </div>
          <div>
            <dt>Vento</dt>
            <dd>${formatWind(current.windSpeed, weather.units)}</dd>
          </div>
        </dl>

        <div class="sun-times" aria-label="Horários do sol">
          <p><span>Nascer do sol</span><strong>${today ? formatTime(today.sunrise) : "—"}</strong></p>
          <p><span>Pôr do sol</span><strong>${today ? formatTime(today.sunset) : "—"}</strong></p>
        </div>

        <p class="updated-at" aria-live="polite">
          ${isStale ? "Últimos dados" : "Atualizado"} às ${updatedAt}
        </p>
      </section>

      <div class="forecast-column">
        <section class="hourly-forecast" aria-labelledby="hourly-title">
          <div class="section-heading">
            <h2 id="hourly-title">Próximas horas</h2>
          </div>
          <ul class="hourly-list" tabindex="0" aria-label="Previsão por hora para ${escapeHtml(weather.location.name)}, próximas 24 horas">
            ${weather.hourly.map((hour, index) => renderHour(hour, index, weather.units)).join("")}
          </ul>
        </section>

        <section class="forecast" aria-labelledby="forecast-title">
          <div class="section-heading">
            <h2 id="forecast-title">Próximos dias</h2>
          </div>
          <ul class="forecast-list">
            ${weather.daily.map((day) => renderDay(day, weather.units)).join("")}
          </ul>
        </section>
      </div>
    </main>
  `;
}

function renderHour(
  hour: WeatherData["hourly"][number],
  index: number,
  units: Units,
): string {
  const time = index === 0 ? "Agora" : formatHour(hour.time);
  const unit = units === "imperial" ? "Fahrenheit" : "Celsius";
  const rainLabel =
    hour.precipitationProbability >= 20
      ? `chance de chuva ${formatPercent(hour.precipitationProbability)}`
      : "sem chance significativa de chuva";
  const accessibleLabel = `${time}, ${formatTemperature(hour.temperature)} graus ${unit}, ${rainLabel}`;

  return `
    <li class="hourly-item" aria-label="${escapeHtml(accessibleLabel)}">
      <span class="hourly-time">${escapeHtml(time)}</span>
      ${weatherIcon(hour.icon, "hourly-icon")}
      <span class="hourly-temperature">${formatTemperature(hour.temperature)}°</span>
      ${
        hour.precipitationProbability >= 20
          ? `<span class="hourly-rain">${formatPercent(hour.precipitationProbability)}</span>`
          : '<span class="hourly-rain" aria-hidden="true"></span>'
      }
    </li>
  `;
}

function renderDay(day: WeatherData["daily"][number], units: Units): string {
  const date = formatForecastDay(day.date);
  const unit = units === "imperial" ? "F" : "C";
  return `
    <li class="forecast-day">
      <div class="forecast-date">
        <strong>${escapeHtml(date.weekday)}</strong>
        <span>${escapeHtml(date.date)}</span>
      </div>
      ${weatherIcon(day.icon, "forecast-icon")}
      <div class="forecast-condition">
        <span class="forecast-description">${escapeHtml(day.description)}</span>
        <span class="rain-chance">${formatPercent(day.precipitationProbability)} chuva</span>
      </div>
      <div class="forecast-temperatures" role="group" aria-label="Máxima ${formatTemperature(day.tempMax)} graus ${unit}, mínima ${formatTemperature(day.tempMin)} graus ${unit}">
        <strong title="Máxima">${formatTemperature(day.tempMax)}°</strong>
        <span class="forecast-temperature-divider" aria-hidden="true"></span>
        <span title="Mínima">${formatTemperature(day.tempMin)}°</span>
      </div>
    </li>
  `;
}

function renderWelcome(state: AppState): string {
  return `
    <main class="welcome-view">
      <div class="welcome-mark" aria-hidden="true">
        ${weatherIcon("partly-cloudy-day")}
      </div>
      <h1>O céu da sua cidade, de relance.</h1>
      <p>Busque uma cidade para ver a temperatura e a previsão dos próximos dias.</p>
      ${
        state.phase === "loading"
          ? '<p class="welcome-loading" role="status">Preparando a previsão…</p>'
          : ""
      }
    </main>
  `;
}

function escapeHtml(value: string): string {
  return value.replace(/[&<>"']/g, (character) => {
    const entities: Record<string, string> = {
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      '"': "&quot;",
      "'": "&#39;",
    };
    return entities[character] ?? character;
  });
}
