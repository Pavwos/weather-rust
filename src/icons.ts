const iconPaths: Record<string, string> = {
  "clear-day":
    '<circle cx="32" cy="32" r="9"/><path d="M32 5v8m0 38v8M5 32h8m38 0h8M13 13l6 6m26 26 6 6m0-38-6 6m-26 26-6 6"/>',
  "clear-night":
    '<path d="M42 43a18 18 0 0 1-21-26 19 19 0 1 0 26 25c-1.6.7-3.3 1-5 1Z"/><path d="m45 11 1.5 4.5L51 17l-4.5 1.5L45 23l-1.5-4.5L39 17l4.5-1.5L45 11Z"/>',
  "partly-cloudy-day":
    '<circle cx="24" cy="25" r="8"/><path d="M24 7v5m0 26v5M6 25h5m26 0h5M11 12l4 4m18 18 4 4m0-26-4 4m-18 18-4 4"/><path d="M19 48h29a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-6 20Z"/>',
  "partly-cloudy-night":
    '<path d="M26 12a14 14 0 0 0 14 18 15 15 0 0 1-21 7 15 15 0 0 1 7-25Z"/><path d="M18 50h29a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-6 20Z"/>',
  cloudy:
    '<path d="M17 48h31a10 10 0 0 0 0-20 13 13 0 0 0-25-3 11 11 0 0 0-6 23Z"/><path d="M12 37a9 9 0 0 1 2-17 13 13 0 0 1 23-5"/>',
  fog:
    '<path d="M17 37h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="M11 45h28m-21 7h27"/>',
  drizzle:
    '<path d="M17 38h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="m22 45-2 5m13-5-2 5m13-5-2 5"/>',
  rain:
    '<path d="M17 36h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="m21 43-3 8m16-8-3 8m16-8-3 8"/>',
  "heavy-rain":
    '<path d="M17 34h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="m18 41-4 10m13-10-4 10m13-10-4 10m13-10-4 10"/>',
  sleet:
    '<path d="M17 36h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="m23 43-3 6m13-6-3 6m12-6-3 6"/><path d="M20 53h.1m13 0h.1m13 0h.1"/>',
  snow:
    '<path d="M17 34h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="M22 42v12m-5-9 10 6m0-6-10 6m19-9v12m-5-9 10 6m0-6-10 6"/>',
  thunderstorm:
    '<path d="M17 34h30a9 9 0 0 0 0-18 12 12 0 0 0-23-2 10 10 0 0 0-7 20Z"/><path d="m33 37-8 12h8l-3 9 12-15h-8l4-6"/>',
};

export function weatherIcon(icon: string, className = ""): string {
  const paths = iconPaths[icon] ?? iconPaths.cloudy;
  return `<svg class="weather-icon ${className}" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">${paths}</svg>`;
}

export function actionIcon(name: "refresh" | "bookmark"): string {
  const paths = {
    refresh:
      '<path d="M20 11a13 13 0 0 1 22-4l3 3"/><path d="M45 5v5h-5"/><path d="M44 21a13 13 0 0 1-22 4l-3-3"/><path d="M19 27v-5h5"/>',
    bookmark:
      '<path d="M20 10h24v34l-12-7-12 7V10Z"/>',
  } satisfies Record<string, string>;
  return `<svg class="action-icon" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">${paths[name]}</svg>`;
}
