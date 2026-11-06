import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";

const none = () => ({});

export function App({ title, slots, open, bind, children }: Props["app"]) {
  const slot = (name: keyof typeof slots) => (slots[name] ?? none)();
  const pane = (side: Side) =>
    slots[side] && (
      <div className={classes("column", side)} hidden={!open[side]} {...bind.pane(side)()}>
        {side === "right" ? (
          <div className="sunken well">
            <div className="controls" {...slot("right")} />
            {slots.tail && <div className="controls tail" {...slot("tail")} />}
          </div>
        ) : (
          <div className="sunken well" {...slot(side)} />
        )}
      </div>
    );
  const opener = (side: Side) =>
    slots[side] && (
      <button
        type="button"
        className={classes("control", "button", open[side] ? "pressed" : "raised")}
        {...bind.opener(side)()}
      >
        {side === "left" ? "Library" : "Tools"}
      </button>
    );
  return (
    <section className="app" aria-label={title || undefined}>
      <div className="wells">
        {pane("left")}
        <div className="column stage">
          <div className="sunken well" {...slot("stage")}>
            {children}
          </div>
          <div className="actions">
            {opener("left")}
            <div className="buttons" {...slot("actions")} />
            {opener("right")}
          </div>
        </div>
        {pane("right")}
      </div>
      <footer className="status">
        <span className="sunken cell" {...slot("status")} />
        <span className="sunken cell hints" {...slot("hints")} />
      </footer>
    </section>
  );
}
