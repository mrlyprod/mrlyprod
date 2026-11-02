import type { Props } from "../../shell/design.ts";
import { MRLY } from "./cell.ts";

/* THUMB */

const SIDE = 112;

type Paint = (pen: CanvasRenderingContext2D, side: number) => void;

const draw = (paint: Paint) => (canvas: HTMLCanvasElement | null) => {
  const pen = canvas?.getContext("2d");
  if (pen) paint(pen, SIDE);
};

/* LIBRARY */

export function Library({ label, chips, chip, onChip, shelves }: Props["library"]) {
  return (
    <nav className="library" {...MRLY} aria-label={label}>
      {chips.length > 0 && (
        <div className="chips" role="tablist" aria-label={label}>
          {chips.map(([id, name]) => (
            <button
              type="button"
              role="tab"
              key={id}
              className="chip"
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
          <h3 className="head">{name}</h3>
          <div className="items">
            {items.map(({ label: one, sub, on, href, paint, bind }) => {
              const body = (
                <>
                  {grid && (
                    <span className="art cell">{paint && <canvas width={SIDE} height={SIDE} ref={draw(paint)} />}</span>
                  )}
                  <span className="name cell">
                    {one}
                    {sub && <small>{sub}</small>}
                  </span>
                </>
              );
              const own = { className: "item", "aria-current": on ? ("true" as const) : undefined, ...bind() };
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
