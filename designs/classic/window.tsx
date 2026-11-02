import { classes } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { href } from "../shared/kit.ts";
import { Glyph } from "./glyphs.tsx";

/* WINDOW */

export function Window(props: Props["window"]) {
  const { kind, title, zoomed, tools, drawer, width, bind, children, side } = props;
  return (
    <section
      className={classes("classic", "window", "raised", kind, zoomed ? "zoomed" : undefined)}
      style={width ? ({ "--width": `${width}px` } as object) : undefined}
      aria-label="Page"
    >
      <span className="edge left" {...bind.edge("left")()} />
      <span className="edge right" {...bind.edge("right")()} />
      <div className="title">
        <Glyph art="page" size={14} />
        <span className="name" {...bind.title()}>
          {title}
        </span>
        <span className="actions">
          <button type="button" className="raised knob" {...bind.tools()} aria-label="Page tools" hidden={!tools}>
            <Glyph art="tools" />
          </button>
          <button type="button" className="raised knob zoom" aria-label="Zoom" aria-pressed={zoomed} {...bind.zoom()}>
            <Glyph art={zoomed ? "restore" : "zoom"} />
          </button>
          <a className="raised knob close" href={href(bind.close)} {...bind.close()} aria-label="Close">
            <Glyph art="close" />
          </a>
        </span>
      </div>
      <nav className="menubar" aria-label="App">
        <span className="menus" role="menubar" {...bind.menus()} />
      </nav>
      <div className="panes" {...bind.scroll()}>
        <main className="sunken well" {...bind.main()}>
          {children}
        </main>
        <aside className="raised pane right" {...bind.pane()} aria-label="Page tools" hidden={!drawer}>
          <div className="title">
            <span className="name">Tools</span>
          </div>
          {side}
        </aside>
        <div className="scrim" {...bind.scrim()} />
      </div>
    </section>
  );
}
