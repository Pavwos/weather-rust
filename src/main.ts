import {
  checkRainAlerts,
  fetchWeather,
  getConfig,
  saveConfig,
  setDefaultCity,
  toApiError,
} from "./api";
import { renderApp } from "./render";
import { createInitialState, type AppState, type Units } from "./state";
import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/plugin-notification";

function getAppRoot(): HTMLElement {
  const element = document.querySelector<HTMLElement>("#app");
  if (!element) {
    throw new Error("Elemento principal do Clima não foi encontrado.");
  }
  return element;
}

const root = getAppRoot();
const state: AppState = createInitialState();
const refreshIntervalMs = 15 * 60 * 1000;

function updateView(): void {
  renderApp(root, state);
}

async function initialize(): Promise<void> {
  try {
    state.config = await getConfig();
    state.units = state.config.units;
    const savedCity = state.config.last_city?.trim() || state.config.default_city?.trim() || "";
    state.cityQuery = savedCity;
    updateView();
    if (savedCity) {
      await loadWeather(savedCity);
    }
  } catch (error) {
    state.phase = "error";
    state.error = toApiError(error);
    updateView();
  }
}

async function loadWeather(city: string): Promise<void> {
  const requestedCity = city.trim();
  if (!requestedCity) {
    state.error = {
      kind: "unknown",
      message: "Digite o nome de uma cidade para buscar a previsão.",
    };
    state.phase = "error";
    updateView();
    return;
  }

  state.cityQuery = requestedCity;
  state.phase = "loading";
  state.error = null;
  state.notice = null;
  updateView();

  try {
    const weather = await fetchWeather(requestedCity, state.units);
    state.weather = weather;
    state.phase = "loaded";
    state.config = {
      ...state.config,
      last_city: requestedCity,
      units: weather.units,
    };
    state.error = null;
    updateView();

    let configSaved = false;
    try {
      await saveConfig(state.config);
      configSaved = true;
    } catch (error) {
      state.error = toApiError(error);
      updateView();
    }
    if (configSaved && state.config.rain_alerts) {
      try {
        await checkRainAlerts();
      } catch {
        state.notice = "Não foi possível verificar a chuva agora. O aviso continua ativo.";
        updateView();
      }
    }
  } catch (error) {
    state.phase = "error";
    state.error = toApiError(error);
    if (state.weather) {
      state.units = state.weather.units;
      state.config = { ...state.config, units: state.weather.units };
    }
    updateView();
  }
}

async function changeRainAlerts(enabled: boolean, input: HTMLInputElement): Promise<void> {
  if (state.savingRainAlerts || enabled === state.config.rain_alerts) {
    return;
  }
  const restoreFocus = document.activeElement === input;
  state.savingRainAlerts = true;
  state.error = null;
  state.notice = null;
  input.disabled = true;

  try {
    if (enabled) {
      const granted =
        (await isPermissionGranted()) || (await requestPermission()) === "granted";
      if (!granted) {
        state.notice =
          "A permissão para notificações não foi concedida. O aviso de chuva permaneceu desligado.";
        return;
      }
    }

    const config = { ...state.config, rain_alerts: enabled };
    await saveConfig(config);
    state.config = config;
    state.notice = enabled ? "Avisos de chuva ativados." : "Avisos de chuva desativados.";
    if (enabled && config.last_city) {
      try {
        await checkRainAlerts();
      } catch {
        state.notice = "Avisos ativados, mas não foi possível verificar a chuva agora.";
      }
    }
  } catch (error) {
    state.error = toApiError(error);
  } finally {
    state.savingRainAlerts = false;
    updateView();
    if (restoreFocus) {
      root.querySelector<HTMLInputElement>('[data-action="rain-alerts"]')?.focus();
    }
  }
}

async function changeUnits(units: Units): Promise<void> {
  if (state.units === units) {
    return;
  }
  state.units = units;
  state.config = { ...state.config, units };
  if (state.weather) {
    await loadWeather(state.cityQuery || state.weather.location.name);
    return;
  }

  state.error = null;
  try {
    await saveConfig(state.config);
  } catch (error) {
    state.error = toApiError(error);
  }
  updateView();
}

async function saveDefaultCity(): Promise<void> {
  const city = state.weather?.location.name;
  if (!city) {
    return;
  }
  try {
    await setDefaultCity(city);
    state.config = { ...state.config, default_city: city };
    state.notice = `${city} salva como cidade padrão.`;
    state.error = null;
  } catch (error) {
    state.error = toApiError(error);
  }
  updateView();
}

function refreshIfStale(): void {
  if (!state.weather || state.phase === "loading") {
    return;
  }
  const fetchedAt = Date.parse(state.weather.fetchedAt);
  if (Number.isFinite(fetchedAt) && Date.now() - fetchedAt > refreshIntervalMs) {
    void loadWeather(state.cityQuery || state.weather.location.name);
  }
}

root.addEventListener("input", (event) => {
  const target = event.target;
  if (target instanceof HTMLInputElement && target.name === "city") {
    state.cityQuery = target.value;
  }
});

root.addEventListener("submit", (event) => {
  const form = event.target;
  if (!(form instanceof HTMLFormElement) || form.id !== "search-form") {
    return;
  }
  event.preventDefault();
  const input = form.elements.namedItem("city");
  if (input instanceof HTMLInputElement) {
    void loadWeather(input.value);
  }
});

root.addEventListener("change", (event) => {
  const target = event.target;
  if (
    target instanceof HTMLInputElement &&
    target.dataset.action === "rain-alerts"
  ) {
    void changeRainAlerts(target.checked, target);
  }
});

root.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof Element)) {
    return;
  }
  const button = target.closest<HTMLButtonElement>("[data-action]");
  if (!button || button.disabled) {
    return;
  }

  switch (button.dataset.action) {
    case "refresh":
      if (state.weather) {
        void loadWeather(state.cityQuery || state.weather.location.name);
      }
      break;
    case "units": {
      const units = button.dataset.units;
      if (units === "metric" || units === "imperial") {
        void changeUnits(units);
      }
      break;
    }
    case "save-city":
      void saveDefaultCity();
      break;
  }
});

root.addEventListener("keydown", (event) => {
  if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") {
    return;
  }
  const target = event.target;
  if (!(target instanceof HTMLElement)) {
    return;
  }
  const list = target.closest<HTMLElement>(".hourly-list");
  if (!list) {
    return;
  }

  const itemWidth = list.querySelector<HTMLElement>(".hourly-item")?.offsetWidth ?? 96;
  event.preventDefault();
  list.scrollBy({
    left: event.key === "ArrowRight" ? itemWidth : -itemWidth,
    behavior: "smooth",
  });
});

root.addEventListener(
  "wheel",
  (event) => {
    if (event.deltaY === 0 || Math.abs(event.deltaY) <= Math.abs(event.deltaX)) {
      return;
    }
    const target = event.target;
    if (!(target instanceof Element)) {
      return;
    }
    const list = target.closest<HTMLElement>(".hourly-list");
    if (!list) {
      return;
    }

    event.preventDefault();
    list.scrollBy({ left: event.deltaY });
  },
  { passive: false },
);

window.addEventListener("focus", refreshIfStale);
window.setInterval(refreshIfStale, refreshIntervalMs);

updateView();
void initialize();
