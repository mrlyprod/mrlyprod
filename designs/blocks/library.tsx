import type { Props } from "../../net/shell/design.ts";
import { BLOCKS, paint } from "./brick.ts";

/* THUMB */

const SIDE = 48;

type Pen = (pen: CanvasRenderingContext2D, side: number) => void;

const draw = (pen: Pen) => (canvas: HTMLCanvasElement | null) => {
  const ink = canvas?.getContext("2d");
  if (ink) pen(ink, SIDE);
};

/* LIBRARY */

export function Library({ label, chips, chip, onChip, shelves }: Props["library"]) {
  return (
    <nav className="library" {...BLOCKS} aria-label={label}>
      {chips.length > 0 && (
        <div className="chips" role="tablist" aria-label={label}>
          {chips.map(([id, name]) => (
            <button
              type="button"
              role="tab"
              key={id}
              className="chip brick"
              style={paint(id === chip ? "blue" : "white")}
              aria-selected={id === chip}
              onClick={() => onChip(id)}
            >
              {name}
            </button>
          ))}
        </div>
      )}
      {shelves.map(({ name, grid, items }) => (
        <section key={name} className="shelf" data-grid={grid} aria-label={name}>
          <h3 className="head tile">{name}</h3>
          <div className="items">
            {items.map(({ label: one, sub, on, href, paint: pen, bind }) => {
              const body = (
                <>
                  {grid && (
                    <span className="art tile">{pen && <canvas width={SIDE} height={SIDE} ref={draw(pen)} />}</span>
                  )}
                  <span className="name">
                    {one}
                    {sub && <small>{sub}</small>}
                  </span>
                </>
              );
              const own = {
                className: "item brick",
                style: paint(on ? "blue" : "white"),
                "aria-current": on ? ("true" as const) : undefined,
                ...bind(),
              };
              return href ? (
                <a key={one} href={href} {...own}>
                  {body}
                </a>
              ) : (
                <button key={one} type="button" {...own}>
                  {body}
                </button>
              );
            })}
          </div>
        </section>
      ))}
    </nav>
  );
}
