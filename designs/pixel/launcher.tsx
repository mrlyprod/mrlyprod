import type { Tile as Door, Props } from "@mrlyprod/ui/shell/design.ts";
import { cx, none } from "./kit.ts";
import { Menu } from "./menu.tsx";
import { Sprite } from "./sprite.tsx";
import { Tile } from "./tile.tsx";

type Launcher = Props["launcher"];

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
        ALL
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
        <Sprite art="add" size={12} />
      </button>
    </nav>
  );
}

function Sections({ sections }: Launcher) {
  return sections.map(({ name, tiles, bind }, n) => (
    <div key={name} className="section" data-stage={(n % 4) + 1} {...bind.shelf()}>
      <div className="head">
        <span className="label" {...bind.head()}>
          {name}
        </span>
        <span className="count">{String(tiles.length).padStart(2, "0")}</span>
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
              REMOVE {name}
            </button>
          ) : (
            "EMPTY"
          )}
        </p>
      )}
    </div>
  ));
}

function Results({ results, at, bind }: Launcher) {
  const groups = results ?? [];
  if (groups.every((one) => !one.tiles.length && !one.hits.length)) return <p className="empty">NO MATCH</p>;
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
              <span className="count">{String(one.rows.length).padStart(2, "0")}</span>
            </div>
            <ul>
              {one.rows.map(([i, row, key]) => (
                <li key={key} className={cx(i === at && "on")} {...bind.hit(i)()}>
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
    <div className="pixel launcher launchpad" {...bind.root()}>
      <div className="veil" aria-hidden="true" {...bind.veil()} />
      <section className="panel frame" aria-label="Menu">
        <div className="title">
          <span className="name">SELECT GAME</span>
          <span className="word">{word.toUpperCase()}</span>
        </div>
        <form role="search" className="find" onSubmit={(e) => e.preventDefault()}>
          <Sprite art="find" size={16} />
          <input
            type="search"
            className="field"
            value={query}
            placeholder="TYPE TO FIND"
            spellCheck={false}
            onChange={(e) => onQuery(e.target.value)}
            {...bind.find()}
          />
          <button type="button" className="knob more" aria-label="More" {...bind.more.button()}>
            <Sprite art="more" size={16} />
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
              UNDO
            </button>
          )}
        </p>
      )}
      {menu && (
        <div className="pixel menu context" role="menu" aria-label={menu.label} {...menu.bind.list()}>
          {menu.items.map((item, i) => (
            <button type="button" key={item.label} className="item" role="menuitem" {...menu.bind.item(i)()}>
              <span className="pointer" aria-hidden="true">
                <Sprite art="pac" size={12} />
              </span>
              <span className="tick" />
              <span className="label">{item.label}</span>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
