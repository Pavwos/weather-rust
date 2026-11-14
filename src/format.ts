import type { DayForecast, Units } from "./state";

const integerFormatter = new Intl.NumberFormat("pt-BR", {
  maximumFractionDigits: 0,
});

export function formatTemperature(value: number): string {
  return integerFormatter.format(Math.round(value));
}

export function formatWind(value: number, units: Units): string {
  return `${integerFormatter.format(Math.round(value))} ${
    units === "imperial" ? "mph" : "km/h"
  }`;
}

export function formatPercent(value: number): string {
  return `${integerFormatter.format(value)}%`;
}

export function formatForecastDay(date: string): {
  weekday: string;
  date: string;
} {
  const parsedDate = new Date(`${date}T12:00:00`);
  return {
    weekday: new Intl.DateTimeFormat("pt-BR", { weekday: "short" })
      .format(parsedDate)
      .replace(/\.$/, ""),
    date: new Intl.DateTimeFormat("pt-BR", {
      day: "2-digit",
      month: "2-digit",
    })
      .format(parsedDate),
  };
}

export function formatTime(value: string): string {
  const match = value.match(/T(\d{2}:\d{2})/);
  return match?.[1] ?? "—";
}

export function formatUpdatedAt(value: string): string {
  return new Intl.DateTimeFormat("pt-BR", {
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(value));
}

export function locationLabel(location: {
  admin1: string | null;
  country: string | null;
}): string {
  return [location.admin1, location.country]
    .filter((part, index, parts): part is string =>
      Boolean(part) && parts.indexOf(part) === index,
    )
    .join(" · ");
}

export function firstDay(days: DayForecast[]): DayForecast | undefined {
  return days[0];
}
