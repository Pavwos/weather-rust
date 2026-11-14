import { fetchWeather, getConfig, saveConfig, setDefaultCity, toApiError } from "./api";
import { renderApp } from "./render";
import { createInitialState, type AppState, type Units } from "./state";

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

    try {
      await saveConfig(state.config);
    } catch (error) {
      state.error = toApiError(error);
      updateView();
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

window.addEventListener("focus", refreshIfStale);
window.setInterval(refreshIfStale, refreshIntervalMs);

updateView();
void initialize();
