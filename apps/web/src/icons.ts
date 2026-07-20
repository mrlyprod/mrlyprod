export const ICONS = {
  check: "check",
  close: "close",
  down: "keyboard_arrow_down",
  download: "download",
  grid: "grid_view",
  left: "chevron_left",
  list: "list",
  pause: "pause",
  play: "play_arrow",
  reset: "refresh",
  right: "chevron_right",
  search: "search",
  settings: "settings",
  up: "keyboard_arrow_up",
} as const

export function icon(name: string): string {
  return (ICONS as Record<string, string>)[name] ?? name
}
