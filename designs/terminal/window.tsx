import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";

/* WINDOW */

export function Window(props: Props["window"]) {
  const { kind, title, zoomed, tools, drawer, width, menus = [], bind, children, side } = props;
  return (
    <section
      className={classes("terminal", "window", "frame", kind, zoomed ? "zoomed" : undefined)}
      style={width ? ({ "--width": `${width}px` } as object) : undefined}
      aria-label="Page"
    >
      <span className="edge left" {...bind.edge("left")()} />
      <span className="edge right" {...bind.edge("right")()} />
      <div className="title">
        <a className="knob close" href={String(bind.close().href ?? "")} {...bind.close()} aria-label="Close">
          [x]
        </a>
        <span className="name">{title}</span>
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
      <nav className="trio menubar" aria-label="App">
        <span className="menus" role="menubar">
          {menus.map((one) => (
            <button type="button" key={one.name} {...one.bind()}>
              {one.name}
            </button>
          ))}
        </span>
      </nav>
    </section>
  );
}
