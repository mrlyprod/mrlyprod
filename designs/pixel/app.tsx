import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";
import { cx, none } from "./kit.ts";

export function App({ title, slots, open, widths, float, onFlip, bind, children }: Props["app"]) {
  const slot = (name: keyof typeof slots) => (slots[name] ?? none)();
  const grip = (side: Side) => (
    <span className="grip" hidden={!open[side] || float} onDoubleClick={() => onFlip(side)} {...bind.grip(side)()} />
  );
  const pane = (side: Side) =>
    slots[side] && (
      <div
        className={cx("column", side, float && "frame")}
        hidden={!open[side]}
        style={{ width: widths[side] }}
        {...bind.pane(side)()}
      >
        <div className="head">{side === "left" ? "LIBRARY" : "KNOBS"}</div>
        {side === "right" ? (
          <div className="well">
            <div className="controls" {...slot("right")} />
            {slots.tail && <div className="controls tail" {...slot("tail")} />}
          </div>
        ) : (
          <div className="well" {...slot(side)} />
        )}
      </div>
    );
  const opener = (side: Side) =>
    float &&
    slots[side] && (
      <button type="button" className="pixel control button" aria-pressed={open[side]} {...bind.opener(side)()}>
        {side === "left" ? "LIBRARY" : "KNOBS"}
      </button>
    );
  return (
    <section className={cx("pixel", "app", float && "float")} aria-label={title || undefined} {...bind.root()}>
      <div className="wells">
        {pane("left")}
        {slots.left && grip("left")}
        <div className="column stage">
          <div className="screen">
            <div className="well" {...slot("stage")}>
              {children}
            </div>
          </div>
          <div className="actions">
            {opener("left")}
            <div className="buttons" {...slot("actions")} />
            {opener("right")}
          </div>
        </div>
        {slots.right && grip("right")}
        {pane("right")}
      </div>
      <footer className="status">
        <span className="cell" {...slot("status")} />
        <span className="cell hints" {...slot("hints")} />
      </footer>
    </section>
  );
}
