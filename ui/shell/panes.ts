export type Pane = { id: string; title: string; art: string; more: string; href: string };

export const PANES: Pane[] = [
  {
    id: "appearance",
    title: "Appearance",
    art: ".###. ##..# ##..# ##..# .###.",
    more: "design theme dark light mode font text size tint accent colour color icon icons style tinted clock time hours 12-hour 24-hour",
  },
  {
    id: "desktop",
    title: "Desktop & Dock",
    art: "##### #...# ##### ..#.. .###.",
    more: "desktop picture wallpaper dock magnify widgets launchpad",
  },
  {
    id: "screensaver",
    title: "Screen Saver",
    art: ".##.. #.... #.... #...# .###.",
    more: "screensaver saver idle lock clock shuffle preview",
  },
  {
    id: "accessibility",
    title: "Accessibility",
    art: "..#.. ##### ..#.. .#.#. #...#",
    more: "motion animation contrast transparency larger targets",
  },
  {
    id: "privacy",
    title: "Privacy & Data",
    art: ".###. .#.#. ##### ##.## #####",
    more: "privacy data storage export import reset erase cookies tracking",
  },
  {
    id: "about",
    title: "About",
    art: "..#.. ..... .##.. ..#.. .###.",
    more: "version licences licenses source social follow robots sitemap llms",
  },
].map((one) => ({ ...one, href: `/settings/${one.id}/` }));
