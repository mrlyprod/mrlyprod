import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { cx, href } from "./kit.ts";
import { Sprite } from "./sprite.tsx";

export function Window(props: Props["window"]) {
  const { kind, title, zoomed, tools, drawer, width, bind, children, side } = props;
  return (
    <section
      className={cx("pixel", "window", "frame", kind, zoomed && "zoomed")}
      style={width ? ({ "--width": `${width}px` } as object) : undefined}
      aria-label="Page"
    >
      <span className="edge left" {...bind.edge("left")()} />
      <span className="edge right" {...bind.edge("right")()} />
      <div className="title">
        <a className="knob close" href={href(bind.close)} {...bind.close()} aria-label="Close">
          <Sprite art="close" size={12} />
        </a>
        <span className="name" {...bind.title()}>
          {title}
        </span>
        <span className="actions">
          <button type="button" className="knob" {...bind.tools()} aria-label="Page tools" hidden={!tools}>
            <Sprite art="tools" size={12} />
          </button>
          <button type="button" className="knob zoom" aria-label="Zoom" aria-pressed={zoomed} {...bind.zoom()}>
            <Sprite art="zoom" size={12} />
          </button>
        </span>
      </div>
      <div className="panes" {...bind.scroll()}>
        <main className="well" {...bind.main()}>
          {children}
        </main>
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden={!drawer}>
          <div className="head">TOOLS</div>
          {side}
        </aside>
        <div className="scrim" {...bind.scrim()} />
      </div>
    </section>
  );
}
