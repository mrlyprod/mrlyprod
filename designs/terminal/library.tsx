import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";

/* ITEM */

type Item = Props["library"]["shelves"][number]["items"][number];

function Thumb({ paint, side }: { paint: Item["paint"]; side: number }) {
  if (!paint) return null;
  return (
    <canvas
      width={side}
      height={side}
      aria-hidden="true"
      ref={(el) => {
        const pen = el?.getContext("2d");
        if (pen) paint(pen, side);
      }}
    />
  );
}

function Leaf({ item, grid }: { item: Item; grid: boolean }) {
  const inner = (
    <>
      <Thumb paint={item.paint} side={grid ? 48 : 16} />
      <span className="label">{item.label}</span>
      {item.sub && <span className="sub">{item.sub}</span>}
    </>
  );
  const look = classes("leaf", item.on ? "on" : undefined);
  return item.href ? (
    <a className={look} href={item.href} aria-current={item.on ? "true" : undefined} {...item.bind()}>
      {inner}
    </a>
  ) : (
    <button type="button" className={look} aria-pressed={item.on} {...item.bind()}>
      {inner}
    </button>
  );
}

/* LIBRARY */

export function Library({ label, chips, chip, onChip, shelves }: Props["library"]) {
  return (
    <nav className="terminal library" aria-label={label}>
      {chips.length > 1 && (
        <div className="chips">
          {chips.map(([id, name]) => (
            <button
              type="button"
              key={id}
              className={id === chip ? "on" : undefined}
              aria-pressed={id === chip}
              onClick={() => onChip(id)}
            >
              {name}
            </button>
          ))}
        </div>
      )}
      <ul className="tree">
        {shelves.map((shelf) => (
          <li key={shelf.name}>
            <details open>
              <summary>
                <span className="sign" aria-hidden="true" />
                <span className="name">{shelf.name}/</span>
              </summary>
              <ul className={shelf.grid ? "grid" : "branch"}>
                {shelf.items.map((item) => (
                  <li key={item.label}>
                    <Leaf item={item} grid={shelf.grid} />
                  </li>
                ))}
              </ul>
            </details>
          </li>
        ))}
      </ul>
    </nav>
  );
}
