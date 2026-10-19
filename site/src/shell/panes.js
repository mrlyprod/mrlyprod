export const PANES = [
  { id: 'appearance', title: 'Appearance', art: '.###. ##..# ##..# ##..# .###.', hue: 'purple', more: 'theme dark light mode font text size tint accent colour color icon icons style tinted' },
  { id: 'glass', title: 'Glass', art: '..#.. .###. #.### ##### .###.', hue: 'cyan', more: 'liquid clear tinted frosted transparency blur' },
  { id: 'desktop', title: 'Desktop & Dock', art: '##### #...# ##### ..#.. .###.', hue: 'indigo', more: 'desktop picture wallpaper dock magnify widgets launchpad window width' },
  { id: 'screensaver', title: 'Screen Saver', art: '.##.. #.... #.... #...# .###.', hue: 'pink', more: 'screensaver saver idle lock clock shuffle preview' },
  { id: 'accessibility', title: 'Accessibility', art: '..#.. ##### ..#.. .#.#. #...#', hue: 'blue', more: 'motion animation contrast transparency larger targets' },
  { id: 'privacy', title: 'Privacy & Data', art: '.###. .#.#. ##### ##.## #####', hue: 'green', more: 'privacy data storage export import reset erase cookies tracking' },
  { id: 'about', title: 'About', art: '..#.. ..... .##.. ..#.. .###.', hue: 'orange', more: 'version licences licenses source social follow robots sitemap llms' },
].map((one) => ({ ...one, href: `/settings/${one.id}/` }));
