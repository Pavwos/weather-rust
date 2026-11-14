export type Units = "metric" | "imperial";
export type RequestPhase = "idle" | "loading" | "loaded" | "error";

export interface Config {
  default_city: string | null;
  units: Units;
  last_city: string | null;
}

export interface Location {
  name: string;
  admin1: string | null;
  country: string | null;
  latitude: number;
  longitude: number;
}

export interface CurrentWeather {
  temperature: number;
  apparentTemperature: number;
  humidity: number;
  windSpeed: number;
  weatherCode: number;
  isDay: boolean;
  description: string;
  icon: string;
}

export interface DayForecast {
  date: string;
  tempMax: number;
  tempMin: number;
  precipitationProbability: number;
  weatherCode: number;
  description: string;
  icon: string;
  sunrise: string;
  sunset: string;
}

export interface WeatherData {
  location: Location;
  current: CurrentWeather;
  daily: DayForecast[];
  units: Units;
  fetchedAt: string;
}

export interface AppState {
  phase: RequestPhase;
  cityQuery: string;
  units: Units;
  config: Config;
  weather: WeatherData | null;
  error: { kind: string; message: string } | null;
  notice: string | null;
}

export function createInitialState(): AppState {
  return {
    phase: "idle",
    cityQuery: "",
    units: "metric",
    config: {
      default_city: null,
      units: "metric",
      last_city: null,
    },
    weather: null,
    error: null,
    notice: null,
  };
}
