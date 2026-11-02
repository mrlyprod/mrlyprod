import type { Props } from "../../net/shell/design.ts";

/* GLYPHS */

const TOOLS =
  "M11.5 16v-8L7.5 12l4 4ZM5 21q-0.825 0-1.4125-0.5875T3 19v-14q0-0.825 0.5875-1.4125T5 3h14q0.825 0 1.4125 0.5875T21 5v14q0 0.825-0.5875 1.4125T19 21H5Zm11-2h3v-14H16v14Zm-2 0v-14H5v14h9Zm2 0h3-3Z";

/* WINDOW */

export function Window({ title, zoomed, tools, drawer, bind, children, side }: Props["window"]) {
  return (
    <section className="window" aria-label="Page">
      {"\n"}
      <div className="title">
        <a className="close" href={String(bind.close().href ?? "")} {...bind.close()} aria-label="Close" />
        <button type="button" className="zoom" aria-label="Zoom" aria-pressed={zoomed} {...bind.zoom()} />
        <span className="name" aria-hidden="true">
          {title}
        </span>
        <span className="actions">
          <button type="button" className="icon" {...bind.tools()} aria-label="Page tools" hidden={!tools}>
            <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true">
              <path d={TOOLS} />
            </svg>
          </button>
        </span>
      </div>
      {"\n"}
      <div className="panes" {...bind.scroll()}>
        {"\n"}
        <main {...bind.main()}>{children}</main>
        {"\n"}
        <aside className="pane right" {...bind.pane()} aria-label="Page tools" hidden={!drawer}>
          {side}
        </aside>
        {"\n"}
        <div className="scrim" {...bind.scrim()} />
        {"\n"}
      </div>
      {"\n"}
    </section>
  );
}
