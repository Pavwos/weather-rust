import { invoke } from "@tauri-apps/api/core";
import type { Config, Units, WeatherData } from "./state";

export type ApiErrorKind = "not_found" | "network" | "timeout" | "unknown";

export interface ApiError {
  kind: ApiErrorKind;
  message: string;
}

export async function fetchWeather(
  city: string,
  units: Units,
): Promise<WeatherData> {
  return invoke<WeatherData>("fetch_weather", { city, units });
}

export async function getConfig(): Promise<Config> {
  const config = await invoke<Config>("get_config");
  return {
    ...config,
    units: config.units === "imperial" ? "imperial" : "metric",
  };
}

export function saveConfig(config: Config): Promise<void> {
  return invoke<void>("save_config", { config });
}

export function setDefaultCity(city: string): Promise<void> {
  return invoke<void>("set_default_city", { city });
}

export function toApiError(error: unknown): ApiError {
  if (typeof error === "object" && error !== null && "message" in error) {
    const message = error.message;
    const kind = "kind" in error ? error.kind : undefined;
    if (typeof message === "string") {
      return {
        kind: isApiErrorKind(kind) ? kind : "unknown",
        message,
      };
    }
  }

  if (typeof error === "string") {
    return { kind: "unknown", message: error };
  }

  return {
    kind: "unknown",
    message: "Não foi possível concluir a operação. Tente novamente.",
  };
}

function isApiErrorKind(value: unknown): value is ApiErrorKind {
  return (
    value === "not_found" ||
    value === "network" ||
    value === "timeout" ||
    value === "unknown"
  );
}
