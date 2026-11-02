import { classes } from "../../net/sdk/room.ts";
import type { Props, Side } from "../../net/shell/design.ts";

/* APP */

const none = () => ({});

export function App({ title, slots, open, widths, float, onFlip, bind, children }: Props["app"]) {
  const slot = (name: keyof typeof slots) => (slots[name] ?? none)();
  const grip = (side: Side) => (
    <span className="grip" hidden={!open[side] || float} onDoubleClick={() => onFlip(side)} {...bind.grip(side)()} />
  );
  const pane = (side: Side) =>
    slots[side] && (
      <div
        className={classes("column", side, float ? "frame" : undefined)}
        hidden={!open[side]}
        style={{ width: widths[side] }}
        {...bind.pane(side)()}
      >
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
      <button type="button" className="terminal control button" aria-pressed={open[side]} {...bind.opener(side)()}>
        {side === "left" ? "Library" : "Tools"}
      </button>
    );
  return (
    <section
      className={classes("terminal", "app", float ? "float" : undefined)}
      aria-label={title || undefined}
      {...bind.root()}
    >
      <div className="wells">
        {pane("left")}
        {slots.left && grip("left")}
        <div className="column stage">
          <div className="well" {...slot("stage")}>
            {children}
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
