import type { Item, Props } from "../../net/shell/design.ts";
import { BOXY, none } from "./cell.ts";
import { Menu } from "./menu.tsx";
import { Tile } from "./tile.tsx";

/* LAUNCHER */

type Launcher = Props["launcher"];

function Hit({ item }: { item: Item }) {
  const body = (
    <>
      <span className="tick">{item.on ? "*" : ""}</span>
      <span className="label">{item.label}</span>
      {item.keys && <kbd>{item.keys.join("+")}</kbd>}
    </>
  );
  return item.href ? (
    <a className="item" href={item.href} data-act={item.act}>
      {body}
    </a>
  ) : (
    <button type="button" className="item" data-act={item.act}>
      {body}
    </button>
  );
}

function Chips({ chips, chip, onChip, naming, bind }: Launcher) {
  return (
    <nav className="chips" aria-label="Sections">
      <button type="button" className="chip" aria-pressed={chip === ""} onClick={() => onChip("")}>
        All
      </button>
      <div className="names" {...bind.chips()}>
        {chips.map((name) =>
          naming === name ? (
            <input key={name} className="chip" {...bind.rename()} />
          ) : (
            <button
              type="button"
              key={name}
              className="chip"
              aria-pressed={chip === name}
              onClick={() => onChip(name)}
              {...bind.chip(name)()}
            >
              {name}
            </button>
          ),
        )}
      </div>
      <button type="button" className="chip add" aria-label="New section" title="New section" {...bind.add()}>
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
            <Tile {...one} named bind={none} />
          </li>
        ))}
      </ul>
      {!tiles.length && bind.drop && (
        <p className="empty">
          drag apps here, or{" "}
          <button type="button" className="link" {...bind.drop()}>
            remove this section
          </button>
        </p>
      )}
    </section>
  ));
}

function Results({ query, results, at, bind }: Launcher) {
  const groups = results ?? [];
  if (groups.every((one) => !one.tiles.length && !one.hits.length))
    return <p className="none cell">nothing found for {query.trim()}</p>;
  let n = 0;
  const numbered = groups.map((one) => ({
    ...one,
    tiles: one.tiles.map((tile) => [n++, tile] as const),
    hits: one.hits.map((hit) => [n++, hit] as const),
  }));
  const hit = (i: number) => ({ ...bind.hit(i)(), "aria-selected": i === at });
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
                  <Tile {...tile} named bind={none} />
                </li>
              ))}
            </ul>
          </section>
        ))}
      {numbered
        .filter((one) => one.hits.length)
        .map((one) => (
          <section key={one.name} className="hits menu" aria-label={one.name}>
            <h2 className="head">{one.name}</h2>
            <ul>
              {one.hits.map(([i, item]) => (
                <li key={item.href ?? item.label} {...hit(i)}>
                  <Hit item={item} />
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
    <div className="launchpad" {...BOXY} {...bind.root()}>
      <div className="veil" aria-hidden="true" {...bind.veil()} />
      <section className="pad" aria-label="Apps">
        <form role="search" className="search" onSubmit={(e) => e.preventDefault()}>
          <label className="find">
            <span className="sr">Search apps</span>
            <input type="search" value={query} onChange={(e) => onQuery(e.target.value)} {...bind.find()} />
          </label>
          <button type="button" className="more cell" {...bind.more.button()}>
            ...
          </button>
          <Menu name="More" items={more} mac={false} bind={{ list: bind.more.list, item: bind.more.item }} />
        </form>
        {!results && <Chips {...props} />}
        <div className="body" {...bind.body()}>
          {results ? <Results {...props} /> : <Shelves {...props} />}
        </div>
        {said && (
          <output className="toast" aria-live="polite" {...bind.said()}>
            {said}
            {bind.undo && (
              <button type="button" className="link" {...bind.undo()}>
                undo
              </button>
            )}
          </output>
        )}
        {menu && (
          <div className="menu pop" role="group" aria-label={menu.label} {...menu.bind.list()}>
            {menu.items.map((item, i) => (
              <button type="button" key={item.label} className="item" {...menu.bind.item(i)()}>
                <span className="tick" />
                <span className="label">{item.label}</span>
              </button>
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
