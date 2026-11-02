import type { CSSProperties } from "react";
import { Icon } from "../../net/sdk/icon.tsx";
import type { Props } from "../../net/shell/design.ts";
import { href } from "../shared/kit.ts";
import { BRICKS, paint } from "./brick.ts";
import { STUD } from "./measure.ts";

/* WINDOW */

const snap = (width: number) => Math.max(STUD, Math.round(width / STUD) * STUD);

export function Window({ kind, title, zoomed, tools, drawer, width, bind, children, side }: Props["window"]) {
  const style = { ...paint("yellow"), "--width": `${snap(width)}px` } as CSSProperties;
  return (
    <section className="window plate" {...BRICKS} data-kind={kind} data-zoomed={zoomed} style={style} aria-label="Page">
      <div className="lip studs" aria-hidden="true" />
      <div className="title">
        <a className="close brick" style={paint("red")} href={href(bind.close)} {...bind.close()} aria-label="Close">
          <Icon name="close" size={24} />
        </a>
        <span className="name tile" aria-hidden="true" {...bind.title()}>
          {title}
        </span>
        <button
          type="button"
          className="zoom brick"
          style={paint("white")}
          aria-label="Zoom"
          aria-pressed={zoomed}
          {...bind.zoom()}
        >
          <Icon name="fullscreen_exit" size={24} />
        </button>
        <button
          type="button"
          className="tools brick"
          style={paint("blue")}
          {...bind.tools()}
          aria-label="Page tools"
          hidden={!tools}
        >
          <Icon name="tune" size={24} />
        </button>
      </div>
      <div className="panes" {...bind.scroll()}>
        <main className="tile" {...bind.main()}>
          {children}
        </main>
        <aside
          className="pane right plate"
          style={paint("blue")}
          {...bind.pane()}
          aria-label="Page tools"
          hidden={!drawer}
        >
          {side}
        </aside>
        <div className="scrim" {...bind.scrim()} />
      </div>
      <div className="edge left" {...bind.edge("left")()} />
      <div className="edge right" {...bind.edge("right")()} />
    </section>
  );
}
