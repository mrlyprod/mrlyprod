import type { IconName } from "@mrlyprod/ui/kit/icon.tsx";
import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";
import { BRICKS, paint } from "./brick.ts";
import { STUD } from "./measure.ts";

const SIDES: Record<Side, { colour: "red" | "blue"; icons: [IconName, IconName] }> = {
  left: { colour: "red", icons: ["left_panel_open", "left_panel_close"] },
  right: { colour: "blue", icons: ["right_panel_open", "right_panel_close"] },
};

const snap = (px: number) => Math.round(px / STUD) * STUD;

const state = (has: boolean, open: boolean) => (has ? (open ? "open" : "shut") : undefined);

export function App({ title, slots, open, widths, float, bind, children }: Props["app"]) {
  const shown = (side: Side) => Boolean(slots[side]) && open[side] && !float;
  const style = {
    "--left": shown("left") ? `${snap(widths.left)}px` : "0px",
    "--right": shown("right") ? `${snap(widths.right)}px` : "0px",
  } as CSSProperties;
  const opener = (side: Side) =>
    float &&
    slots[side] && (
      <button type="button" className="side brick" style={paint("white")} {...bind.opener(side)()}>
        <Icon name={SIDES[side].icons[open[side] ? 1 : 0]} size={24} />
      </button>
    );
  const pane = (side: Side) =>
    slots[side] && (
      <aside className={`pane ${side} plate`} style={paint(SIDES[side].colour)} {...bind.pane(side)()}>
        {side === "left" ? (
          <div className="body" {...slots.left?.()} />
        ) : (
          <div className="body">
            <section className="controls" aria-label="Controls" {...slots.right?.()} />
            <div className="controls tail" {...slots.tail?.()} />
          </div>
        )}
        <div className="grip" {...bind.grip(side)()} />
      </aside>
    );
  return (
    <section
      className="app"
      {...BRICKS}
      aria-label={title || undefined}
      {...bind.root()}
      style={style}
      data-left={state(Boolean(slots.left), open.left)}
      data-right={state(Boolean(slots.right), open.right)}
      data-float={float || undefined}
    >
      {pane("left")}
      <div className="middle">
        <div className="stage tile" {...slots.stage?.()}>
          {children}
        </div>
        <div className="strip">
          {opener("left")}
          <div className="actions" {...slots.actions?.()} />
          <div className="status tile" {...slots.status?.()} />
          <div className="hints tile" {...slots.hints?.()} />
          {opener("right")}
        </div>
      </div>
      {pane("right")}
    </section>
  );
}
