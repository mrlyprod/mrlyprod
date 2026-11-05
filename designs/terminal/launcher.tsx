import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Tile as Door, Props } from "@mrlyprod/ui/shell/design.ts";
import { Menu } from "./menu.tsx";
import { Tile } from "./tile.tsx";

type Launcher = Props["launcher"];

const none = () => ({});

const door = (one: Door) => <Tile {...one} named bind={none} />;

function Chips({ chips, chip, onChip, naming, bind }: Launcher) {
  return (
    <nav className="chips" aria-label="Sections">
      <button
        type="button"
        className={chip === "" ? "on" : undefined}
        aria-pressed={chip === ""}
        onClick={() => onChip("")}
      >
        all
      </button>
      <span className="names" {...bind.chips()}>
        {chips.map((name) =>
          naming === name ? (
            <input key={name} className="field" {...bind.rename()} />
          ) : (
            <button
              type="button"
              key={name}
              className={name === chip ? "on" : undefined}
              aria-pressed={name === chip}
              onClick={() => onChip(name)}
              {...bind.chip(name)()}
            >
              {name}
            </button>
          ),
        )}
      </span>
      <button type="button" className="add" aria-label="New section" title="New section" {...bind.add()}>
        [+]
      </button>
    </nav>
  );
}

function Sections({ sections }: Launcher) {
  return sections.map(({ name, tiles, bind }) => (
    <div key={name} className="section" {...bind.shelf()}>
      <div className="head">
        <span className="label" {...bind.head()}>
          {name}/
        </span>
        <span className="count">{tiles.length}</span>
      </div>
      <ul className="apps" aria-label={name} {...bind.list()}>
        {tiles.map((one) => (
          <li key={one.id} {...bind.tile(one.id)()}>
            {door(one)}
          </li>
        ))}
      </ul>
      {!tiles.length && (
        <p className="empty">
          {bind.drop ? (
            <button type="button" className="link" {...bind.drop()}>
              rmdir {name}
            </button>
          ) : (
            "(empty)"
          )}
        </p>
      )}
    </div>
  ));
}

function Results({ results, at, bind }: Launcher) {
  const groups = results ?? [];
  if (groups.every((one) => !one.tiles.length && !one.hits.length)) return <p className="empty">0 matches</p>;
  let n = 0;
  const numbered = groups.map((one) => ({
    ...one,
    rows: [
      ...one.tiles.map((tile) => [n++, door(tile), tile.id] as const),
      ...one.hits.map(
        (hit) =>
          [
            n++,
            <a key="hit" className="item" href={hit.href}>
              <span className="label">{hit.label}</span>
            </a>,
            hit.href ?? hit.label,
          ] as const,
      ),
    ],
  }));
  return (
    <div className="results" {...bind.results()}>
      {numbered
        .filter((one) => one.rows.length)
        .map((one) => (
          <section key={one.name} aria-label={one.name}>
            <div className="head">
              <span className="label">{one.name}</span>
              <span className="count">{one.rows.length}</span>
            </div>
            <ul>
              {one.rows.map(([i, row, key]) => (
                <li key={key} className={classes(i === at ? "on" : undefined)} {...bind.hit(i)()}>
                  {row}
                </li>
              ))}
            </ul>
          </section>
        ))}
    </div>
  );
}

export function Launcher(props: Launcher) {
  const { word, query, onQuery, results, said, more, menu, bind } = props;
  return (
    <div className="terminal launcher launchpad" {...bind.root()}>
      <div className="veil" aria-hidden="true" {...bind.veil()} />
      <section className="frame panel" aria-label="Menu">
        <div className="title">
          <span className="name">{word.toLowerCase()} menu</span>
        </div>
        <form role="search" className="find" onSubmit={(e) => e.preventDefault()}>
          <span className="sigil" aria-hidden="true">
            &gt;
          </span>
          <input
            type="search"
            className="field"
            value={query}
            placeholder="find"
            onChange={(e) => onQuery(e.target.value)}
            {...bind.find()}
          />
          <button type="button" className="knob more" {...bind.more.button()}>
            [...]
          </button>
        </form>
        <Menu name="More" items={more} mac={false} bind={{ list: bind.more.list, item: bind.more.item }} />
        {!results && <Chips {...props} />}
        <div className="body" {...bind.body()}>
          {results ? <Results {...props} /> : <Sections {...props} />}
        </div>
      </section>
      {said && (
        <p className="said" role="status" {...bind.said()}>
          {said}
          {bind.undo && (
            <button type="button" className="link" {...bind.undo()}>
              [undo]
            </button>
          )}
        </p>
      )}
      {menu && (
        <div className="terminal menu frame context" role="menu" aria-label={menu.label} {...menu.bind.list()}>
          {menu.items.map((item, i) => (
            <button type="button" key={item.label} className="item" role="menuitem" {...menu.bind.item(i)()}>
              <span className="gutter" />
              <span className="label">{item.label}</span>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
