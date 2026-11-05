import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "./glyphs.tsx";

const none = () => ({});

export function App({ title, slots, open, widths, float, onFlip, bind, children }: Props["app"]) {
  const slot = (name: keyof typeof slots) => (slots[name] ?? none)();
  const grip = (side: Side) => (
    <span
      className="raised grip"
      hidden={!open[side] || float}
      onDoubleClick={() => onFlip(side)}
      {...bind.grip(side)()}
    />
  );
  const pane = (side: Side) =>
    slots[side] && (
      <div
        className={classes("column", side, float ? "raised" : undefined)}
        hidden={!open[side]}
        style={{ width: widths[side] }}
        {...bind.pane(side)()}
      >
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
    float &&
    slots[side] && (
      <button type="button" className="control button raised" aria-pressed={open[side]} {...bind.opener(side)()}>
        {side === "left" ? "Library" : "Tools"}
      </button>
    );
  return (
    <section
      className={classes("classic", "app", float ? "float" : undefined)}
      aria-label={title || undefined}
      {...bind.root()}
    >
      <div className="wells">
        {pane("left")}
        {slots.left && grip("left")}
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
        {slots.right && grip("right")}
        {pane("right")}
      </div>
      <footer className="status">
        <span className="sunken cell" {...slot("status")} />
        <span className="sunken cell hints" {...slot("hints")} />
        <span className="sizer" aria-hidden="true">
          <Glyph art="grip" size={12} />
        </span>
      </footer>
    </section>
  );
}
