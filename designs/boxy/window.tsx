import type { Props } from "../../net/shell/design.ts";
import { href } from "../shared/kit.ts";
import { BOXY, hash, tint } from "./cell.ts";

/* WINDOW */

export function Window({ kind, title, zoomed, tools, drawer, bind, children, side }: Props["window"]) {
  const key = title.toLowerCase();
  const hue = kind === "wide" && key ? hash(key) : "gray";
  return (
    <section
      className="window"
      {...BOXY}
      data-kind={kind}
      data-zoomed={zoomed}
      data-hue={hue}
      style={tint(hue)}
      aria-label="Page"
    >
      <div className="title">
        <a className="close cell" href={href(bind.close)} {...bind.close()} aria-label="Close">
          x
        </a>
        <span className="name cell" aria-hidden="true" {...bind.title()}>
          {title}
        </span>
        <button type="button" className="zoom cell" aria-label="Zoom" aria-pressed={zoomed} {...bind.zoom()}>
          {zoomed ? "-" : "+"}
        </button>
        <button type="button" className="tools cell" {...bind.tools()} aria-label="Page tools" hidden={!tools}>
          =
        </button>
      </div>
      <div className="panes" {...bind.scroll()}>
        <main {...bind.main()}>{children}</main>
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden={!drawer}>
          {side}
        </aside>
        <div className="scrim" {...bind.scrim()} />
      </div>
    </section>
  );
}
