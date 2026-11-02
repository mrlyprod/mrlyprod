import type { Bind, Props } from "../../shell/design.ts";
import { Glyph } from "./glyph.tsx";
import { Fold } from "./lock.tsx";

/* BAR */

const href = (bind: Bind) => String(bind().href ?? "");

export function Bar({ site, app, bind }: Props["bar"]) {
  return (
    <header className="bar">
      <nav className="trio" data-lens="bar" aria-label="System">
        <a className="glyph launch" href={href(bind.launch)} {...bind.launch()} aria-label="Launchpad">
          <Glyph art="launch" />
        </a>
        <a className="mark" href={href(bind.mark)} {...bind.mark()} aria-label={`${site.name} home`}>
          <Fold word={site.word} />
        </a>
        <a className="glyph" href={href(bind.cart)} {...bind.cart()} aria-label="Cart">
          <Glyph art="cart" />
        </a>
        <span className="app">{app}</span>
        <span className="menus" role="menubar" aria-label="App" />
      </nav>
      <div className="tray" data-lens="bar">
        <time className="date" />
        <time className="clock" />
        <a className="glyph find" href={href(bind.find)} {...bind.find()} aria-label="Search">
          <Glyph art="find" />
        </a>
      </div>
    </header>
  );
}
