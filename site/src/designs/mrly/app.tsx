import type { CSSProperties } from "react";
import type { Props, Side } from "../../shell/design.ts";
import { MRLY } from "./cell.ts";

/* APP */

const KEYS: Record<Side, string> = { left: "[", right: "]" };

export function App({ title, slots, open, widths, float, bind, children }: Props["app"]) {
  const shown = (side: Side) => Boolean(slots[side]) && open[side];
  const style = {
    "--left": shown("left") ? `${widths.left}px` : "0px",
    "--right": shown("right") ? `${widths.right}px` : "0px",
  } as CSSProperties;
  const flip = (side: Side) =>
    slots[side] && (
      <button type="button" className="cell side" {...bind.opener(side)()}>
        {KEYS[side]}
      </button>
    );
  return (
    <section
      className="app"
      {...MRLY}
      aria-label={title || undefined}
      {...bind.root()}
      style={style}
      data-float={float ? "" : undefined}
    >
      {slots.left && <aside className="pane left" {...bind.pane("left")()} {...slots.left()} hidden={!open.left} />}
      {slots.left && <div className="grip left" {...bind.grip("left")()} hidden={!open.left} />}
      <div className="middle">
        <div className="stage cell" {...slots.stage?.()}>
          {children}
        </div>
      </div>
      <div className="strip">
        {flip("left")}
        {slots.actions && <div className="actions" {...slots.actions()} />}
        {slots.status && <div className="status cell" {...slots.status()} />}
        {slots.hints && <div className="hints cell" {...slots.hints()} />}
        {flip("right")}
      </div>
      {slots.right && <div className="grip right" {...bind.grip("right")()} hidden={!open.right} />}
      {slots.right && (
        <aside className="pane right" {...bind.pane("right")()} hidden={!open.right}>
          <section className="controls" aria-label="Controls" {...slots.right()} />
          {slots.tail && <div className="controls tail" {...slots.tail()} />}
        </aside>
      )}
    </section>
  );
}
