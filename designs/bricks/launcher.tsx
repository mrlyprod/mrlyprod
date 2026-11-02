import { Icon } from "../../net/sdk/icon.tsx";
import type { Tile as Door, Item, Props } from "../../net/shell/design.ts";
import { BRICKS, none, paint } from "./brick.ts";
import { Menu } from "./menu.tsx";
import { Tile } from "./tile.tsx";

/* LAUNCHER */

type Launcher = Props["launcher"];

const door = (one: Door) => <Tile {...one} named bind={none} />;

function Chips({ chips, chip, onChip, naming, bind }: Launcher) {
  const brick = (on: boolean) => ({ className: "chip brick", style: paint(on ? "blue" : "white"), "aria-pressed": on });
  return (
    <nav className="chips" aria-label="Sections">
      <button type="button" {...brick(chip === "")} onClick={() => onChip("")}>
        All
      </button>
      <div className="names" {...bind.chips()}>
        {chips.map((name) =>
          naming === name ? (
            <input key={name} className="chip tile" {...bind.rename()} />
          ) : (
            <button
              type="button"
              key={name}
              {...brick(chip === name)}
              onClick={() => onChip(name)}
              {...bind.chip(name)()}
            >
              {name}
            </button>
          ),
        )}
      </div>
      <button type="button" className="add brick" style={paint("green")} aria-label="New section" {...bind.add()}>
        +
      </button>
    </nav>
  );
}

function Shelves({ sections }: Launcher) {
  return sections.map(({ name, tiles, bind }) => (
    <section key={name} className="shelf" {...bind.shelf()}>
      <h2 className="head" {...bind.head()}>
        <span>{name}</span>
        <span className="count">{tiles.length}</span>
      </h2>
      <ul className="grid" {...bind.list()}>
        {tiles.map((one) => (
          <li key={one.id} {...bind.tile(one.id)()}>
            {door(one)}
          </li>
        ))}
      </ul>
      {!tiles.length && bind.drop && (
        <p className="empty tile">
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
    return <p className="none tile">Nothing found for {query.trim()}.</p>;
  let n = 0;
  const numbered = groups.map((one) => ({
    ...one,
    tiles: one.tiles.map((tile) => [n++, tile] as const),
    hits: one.hits.map((hit) => [n++, hit] as const),
  }));
  const hit = (i: number) => ({ ...bind.hit(i)(), "data-on": i === at || undefined });
  return (
    <div className="results" {...bind.results()}>
      {numbered
        .filter((one) => one.tiles.length)
        .map((one) => (
          <section key={one.name} className="shelf" aria-label={one.name}>
            <h2 className="head">{one.name}</h2>
            <ul className="grid">
              {one.tiles.map(([i, tile]) => (
                <li key={tile.id} {...hit(i)}>
                  {door(tile)}
                </li>
              ))}
            </ul>
          </section>
        ))}
      {numbered
        .filter((one) => one.hits.length)
        .map((one) => (
          <section key={one.name} className="hits" aria-label={one.name}>
            <h2 className="head">{one.name}</h2>
            <ul>
              {one.hits.map(([i, item]: readonly [number, Item]) => (
                <li key={item.href ?? item.label} {...hit(i)}>
                  <a className="item tile" href={item.href}>
                    {item.label}
                  </a>
                </li>
              ))}
            </ul>
          </section>
        ))}
    </div>
  );
}

export function Launcher(props: Launcher) {
  const { query, onQuery, results, said, more, menu, bind } = props;
  return (
    <div className="launcher-root" {...BRICKS} {...bind.root()}>
      <div className="veil" aria-hidden="true" {...bind.veil()} />
      <section className="launchpad plate" style={paint("white")} aria-label="Launchpad">
        <header className="search">
          <form role="search" className="find tile" onSubmit={(e) => e.preventDefault()}>
            <input
              type="search"
              aria-label="Search apps"
              placeholder="Search apps"
              value={query}
              onChange={(e) => onQuery(e.target.value)}
              {...bind.find()}
            />
            <span className="brick" style={paint("yellow")} aria-hidden="true">
              <Icon name="search" size={24} />
            </span>
          </form>
          <button type="button" className="more brick" style={paint("white")} aria-label="More" {...bind.more.button()}>
            <Icon name="more_horiz" size={24} />
          </button>
          <Menu name="More" items={more} mac={false} bind={{ list: bind.more.list, item: bind.more.item }} />
        </header>
        {!results && <Chips {...props} />}
        <div className="body" {...bind.body()}>
          {results ? <Results {...props} /> : <Shelves {...props} />}
        </div>
        {menu && <Menu name={menu.label} items={menu.items} mac={false} bind={menu.bind} />}
      </section>
      {said && (
        <p className="toast brick" style={paint("yellow")} role="status" {...bind.said()}>
          {said}
          {bind.undo && (
            <button type="button" className="undo tile" {...bind.undo()}>
              Undo
            </button>
          )}
        </p>
      )}
    </div>
  );
}
