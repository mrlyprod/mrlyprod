import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Tile as Door, Item, Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyph.tsx";
import { Control } from "./control.tsx";
import { Menu } from "./menu.tsx";
import { Tile } from "./tile.tsx";

type Launcher = Props["launcher"];

const none = () => ({});

const door = (one: Door) => <Tile {...one} named bind={none} />;

function Chips({ chips, chip, onChip, naming, bind }: Launcher) {
  return (
    <nav className="chips" aria-label="Sections">
      <button type="button" className="pill" aria-pressed={chip === ""} onClick={() => onChip("")}>
        All
      </button>
      <div className="names" {...bind.chips()}>
        {chips.map((name) =>
          naming === name ? (
            <input key={name} className="pill" {...bind.rename()} />
          ) : (
            <button
              type="button"
              key={name}
              className="pill"
              aria-pressed={chip === name}
              onClick={() => onChip(name)}
              {...bind.chip(name)()}
            >
              {name}
            </button>
          ),
        )}
      </div>
      <button type="button" className="pill add" aria-label="New section" title="New section" {...bind.add()}>
        <Glyph art="plus" />
      </button>
    </nav>
  );
}

function Shelves({ sections }: Launcher) {
  return sections.map(({ name, tiles, bind }) => (
    <section key={name} className="shelf" {...bind.shelf()}>
      <h2 {...bind.head()}>{name}</h2>
      <ul className="apps" {...bind.list()}>
        {tiles.map((one) => (
          <li key={one.id} {...bind.tile(one.id)()}>
            {door(one)}
          </li>
        ))}
      </ul>
      {!tiles.length && bind.drop && (
        <p className="empty">
          Drag apps here, or{" "}
          <button type="button" className="link" {...bind.drop()}>
            remove this section
          </button>
          .
        </p>
      )}
    </section>
  ));
}

function Results({ query, results, at, bind }: Launcher) {
  const groups = results ?? [];
  if (groups.every((one) => !one.tiles.length && !one.hits.length))
    return <p className="none">Nothing found for {query.trim()}.</p>;
  let n = 0;
  const numbered = groups.map((one) => ({
    ...one,
    tiles: one.tiles.map((tile) => [n++, tile] as const),
    hits: one.hits.map((hit) => [n++, hit] as const),
  }));
  const hit = (i: number) => ({ ...bind.hit(i)(), className: i === at ? "on" : undefined });
  return (
    <div className="results" {...bind.results()}>
      {numbered
        .filter((one) => one.tiles.length)
        .map((one) => (
          <section key={one.name} className="shelf" aria-label={one.name}>
            <h2>{one.name}</h2>
            <ul className="apps">
              {one.tiles.map(([i, tile]) => (
                <li key={tile.id} {...hit(i)}>
                  {door(tile)}
                </li>
              ))}
            </ul>
          </section>
        ))}
      <div className="lists">
        {numbered
          .filter((one) => one.hits.length)
          .map((one) => (
            <section key={one.name} aria-label={one.name}>
              <h2>{one.name}</h2>
              <ul className="hits">
                {one.hits.map(([i, item]: readonly [number, Item]) => (
                  <li key={item.href} {...hit(i)}>
                    <a href={item.href}>{item.label}</a>
                  </li>
                ))}
              </ul>
            </section>
          ))}
      </div>
    </div>
  );
}

export function Launcher(props: Launcher) {
  const { query, onQuery, results, said, more, menu, bind } = props;
  return (
    <div className="launchpad" {...bind.root()}>
      <div className="veil" aria-hidden="true" {...bind.veil()} />
      <section className="pad" data-lens="launchpad" aria-label="Launchpad">
        <header>
          <Glyph art="apps" />
          {!results && <h1>Apps</h1>}
          <form role="search" className="find" onSubmit={(e) => e.preventDefault()}>
            <Glyph art="find" />
            <input type="search" value={query} onChange={(e) => onQuery(e.target.value)} {...bind.find()} />
          </form>
          <Control kind="button" label="" value="" tag="icon more" bind={bind.more.button}>
            <Icon name="more" />
          </Control>
          <Menu name="More" items={more} mac={false} bind={{ list: bind.more.list, item: bind.more.item }} />
        </header>
        {said && (
          <p className="said" role="status" {...bind.said()}>
            {said}
            {bind.undo && (
              <button type="button" className="link" {...bind.undo()}>
                Undo
              </button>
            )}
          </p>
        )}
        {!results && <Chips {...props} />}
        <div className="body" {...bind.body()}>
          {results ? <Results {...props} /> : <Shelves {...props} />}
        </div>
        {menu && (
          <div className="pop menu" data-lens="menu" role="group" aria-label={menu.label} {...menu.bind.list()}>
            {menu.items.map((item, i) => (
              <button type="button" key={item.label} {...menu.bind.item(i)()}>
                {item.label}
              </button>
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
