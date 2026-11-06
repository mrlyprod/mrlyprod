import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props, Side } from "@mrlyprod/ui/shell/design.ts";

const state = (on: boolean | undefined, open: boolean) => (on ? (open ? "open" : "shut") : undefined);

export function App({ title, slots, open, bind, children }: Props["app"]) {
  const glass = (name: string, kind: string) => ({ className: name, "data-lens": kind });
  const opener = (side: Side) =>
    slots[side] && (
      <button type="button" className="icon side" {...bind.opener(side)()}>
        <Icon name={`pane-${side}`} />
      </button>
    );
  return (
    <section
      className="frame"
      aria-label={title || undefined}
      data-left={state(!!slots.left, open.left)}
      data-right={state(!!slots.right, open.right)}
    >
      <div className="canvas" {...slots.stage?.()}>
        {children}
      </div>
      {slots.left && (
        <aside {...glass("pane left", "panel")} {...bind.pane("left")()}>
          <div className="body" {...slots.left()} />
        </aside>
      )}
      {slots.right && (
        <aside {...glass("pane right", "panel")} {...bind.pane("right")()}>
          <div className="body">
            <section className="controls" aria-label="Controls" {...slots.right()} />
            <div className="controls tail" {...slots.tail?.()} />
          </div>
        </aside>
      )}
      {(slots.actions || slots.left || slots.right) && (
        <div {...glass("actions", "pill")}>
          {opener("left")}
          <div className="buttons" {...slots.actions?.()} />
          {opener("right")}
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
