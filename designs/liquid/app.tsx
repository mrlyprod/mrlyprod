import { sourced, tap } from "@mrlyprod/ui/kit/feed.ts";
import type { IconName } from "@mrlyprod/ui/kit/icon.tsx";
import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties, Ref } from "react";
import { useEffect, useMemo, useRef, useState } from "react";

type Vars = CSSProperties & Record<`--${string}`, string>;

const ICONS: Record<Side, [IconName, IconName]> = {
  left: ["left_panel_open", "left_panel_close"],
  right: ["right_panel_open", "right_panel_close"],
};

const state = (on: boolean | undefined, open: boolean) => (on ? (open ? "open" : "shut") : undefined);

function joined<T>(mine: { current: T | null }, theirs: unknown) {
  return (el: T | null) => {
    mine.current = el;
    if (typeof theirs === "function") theirs(el);
    else if (theirs && typeof theirs === "object") (theirs as { current: T | null }).current = el;
  };
}

export function App({ title, slots, open, widths, float, bind, children }: Props["app"]) {
  const box = useRef<HTMLElement>(null);
  const [fed, setFed] = useState(false);
  useEffect(() => {
    const look = () => setFed(sourced().some((one) => box.current?.contains(one)));
    look();
    return tap(look);
  }, []);
  const { ref: theirs, ...root } = bind.root();
  const ref = useMemo(() => joined(box, theirs), [theirs]) as Ref<HTMLElement>;
  const glass = (name: string, kind: string) =>
    fed ? { className: name, "data-lens": kind } : { className: `${name} frost` };
  const sides = (["left", "right"] as Side[]).filter((side) => slots[side]);
  const style: Vars = {
    "--left": `${widths.left}px`,
    "--right": `${widths.right}px`,
  };
  const opener = (side: Side) => (
    <button type="button" className="icon side" {...bind.opener(side)()}>
      <Icon name={ICONS[side][open[side] ? 1 : 0]} />
    </button>
  );
  return (
    <section
      className="frame"
      aria-label={title || undefined}
      {...root}
      ref={ref}
      style={style}
      data-left={state(!!slots.left, open.left)}
      data-right={state(!!slots.right, open.right)}
      data-float={float ? "" : undefined}
    >
      <div className="canvas" {...slots.stage?.()}>
        {children}
      </div>
      {slots.left && (
        <aside {...glass("pane left", "panel")} {...bind.pane("left")()}>
          <div className="body" {...slots.left()} />
          <div className="grip" {...bind.grip("left")()} />
        </aside>
      )}
      {slots.right && (
        <aside {...glass("pane right", "panel")} {...bind.pane("right")()}>
          <div className="body">
            <section className="controls" aria-label="Controls" {...slots.right()} />
            <div className="controls tail" {...slots.tail?.()} />
          </div>
          <div className="grip" {...bind.grip("right")()} />
        </aside>
      )}
      {(slots.actions || (float && sides.length > 0)) && (
        <div {...glass("actions", "pill")}>
          {float && slots.left && opener("left")}
          <div className="buttons" {...slots.actions?.()} />
          {float && slots.right && opener("right")}
        </div>
      )}
      {(slots.status || slots.hints) && (
        <div {...glass("status", "pill")}>
          <div className="facts" {...slots.status?.()} />
          <div className="hints" {...slots.hints?.()} />
        </div>
      )}
    </section>
  );
}
