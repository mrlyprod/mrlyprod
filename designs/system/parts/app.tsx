import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";

const KEYS: Record<Side, string> = { left: "[", right: "]" };

export function App({ title, slots, open, bind, children }: Props["app"]) {
  const flip = (side: Side) =>
    slots[side] && (
      <button type="button" className="cell side" {...bind.opener(side)()}>
        {KEYS[side]}
      </button>
    );
  return (
    <section className="app" aria-label={title || undefined}>
      {slots.left && <aside className="pane left" {...bind.pane("left")()} {...slots.left()} hidden={!open.left} />}
      <div className="middle">
        <div className="board cell" {...slots.stage?.()}>
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
      {slots.right && (
        <aside className="pane right" {...bind.pane("right")()} hidden={!open.right}>
          <section className="controls" aria-label="Controls" {...slots.right()} />
          {slots.tail && <div className="controls tail" {...slots.tail()} />}
        </aside>
      )}
    </section>
  );
}
