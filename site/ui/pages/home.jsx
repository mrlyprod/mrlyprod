import { doorway, row as line, tiled } from '../heroes.js';
import { filled, hide, show } from '../parts.jsx';

let made = null;

function art(site, rows) {
  if (made) return made;
  const doors = filled(site, rows);
  for (const hero of site.heroes) if (!doors.some((door) => door.name === hero.name)) throw new Error(`home: site.json names the hero ${hero.name}, and the tree has no such door`);
  const rest = doors.filter((door) => door.href !== '/' && !site.heroes.some((hero) => hero.name === door.name));
  const half = (door, avoid) => {
    const one = site.tiles.find((tile) => tile.name === door.name);
    if (!one) throw new Error(`home: the tree's door ${door.name} has no row in the tiles of site.json, so it has no hues`);
    return tiled(door.name, door.href ?? `${site.menu}#${door.name.toLowerCase()}`, one.hues, avoid);
  };
  made = { heroes: line(site.heroes, doorway), shelf: line(rest, half) };
  return made;
}

export function render(row, site, host, { rows }) {
  const { heroes, shelf } = art(site, rows);
  show(
    host,
    <>
      <h1 className="visually-hidden">{site.title}</h1>
      <section className="heroes" aria-label="Doors" dangerouslySetInnerHTML={{ __html: heroes }} />
      <section className="shelf" aria-label="More doors" dangerouslySetInnerHTML={{ __html: shelf }} />
    </>,
  );
}

export const unmount = hide;
