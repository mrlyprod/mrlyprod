import { classes } from "@mrlyprod/ui/kit/room.ts";
import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Window(props: Props["window"]) {
  const { kind, title, zoomed, tools, drawer, width, bind, children, side } = props;
  return (
    <section
      className={classes("terminal", "window", "frame", kind, zoomed ? "zoomed" : undefined)}
      style={width ? ({ "--width": `${width}px` } as object) : undefined}
      aria-label="Page"
    >
      <span className="edge left" {...bind.edge("left")()} />
      <span className="edge right" {...bind.edge("right")()} />
      <div className="title">
        <a className="knob close" href={href(bind.close)} {...bind.close()} aria-label="Close">
          [x]
        </a>
        <span className="name" {...bind.title()}>
          {title}
        </span>
        <span className="actions">
          <button type="button" className="knob" {...bind.tools()} aria-label="Page tools" hidden={!tools}>
            [=]
          </button>
          <button type="button" className="knob zoom" aria-label="Zoom" aria-pressed={zoomed} {...bind.zoom()}>
            {zoomed ? "[v]" : "[^]"}
          </button>
        </span>
      </div>
      <div className="panes" {...bind.scroll()}>
        <main className="well" {...bind.main()}>
          {children}
        </main>
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden={!drawer}>
          <div className="head">tools</div>
          {side}
        </aside>
        <div className="scrim" {...bind.scrim()} />
      </div>
      <nav className="menubar" aria-label="App">
        <span className="menus" role="menubar" {...bind.menus()} />
      </nav>
    </section>
  );
}
