import type { Bind, Props } from "../../net/shell/design.ts";
import { hhmm } from "./clock.ts";
import { Glyph } from "./glyphs.tsx";

/* BAR */

const href = (bind: Bind) => String(bind().href ?? "");

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="classic bar">
      <a className="raised start" href={href(bind.launch)} {...bind.launch()} aria-label="Start">
        <img src={site.logo} width="16" height="16" alt="" />
        <b>{site.name}</b>
      </a>
      <nav className="trio" aria-label="Tasks">
        <span className="pressed task app" hidden={!app}>
          {app}
        </span>
      </nav>
      <div className="sunken tray">
        {site.cart && (
          <a className="cart" href={href(bind.cart)} {...bind.cart()} hidden={cart < 1} aria-label={`Cart, ${cart}`}>
            <Glyph art="cart" size={14} />
            {cart}
          </a>
        )}
        <a className="glyph find" href={href(bind.find)} {...bind.find()} aria-label="Search">
          <Glyph art="find" size={14} />
        </a>
        <time className="date" hidden />
        <time className="clock">{hhmm(clock)}</time>
      </div>
    </header>
  );
}
