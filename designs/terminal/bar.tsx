import type { Bind, Props } from "../../net/shell/design.ts";
import { hhmm } from "./clock.ts";

/* BAR */

const href = (bind: Bind) => String(bind().href ?? "");

const path = (app: string | null) => (app ? `~/${app.toLowerCase().replace(/\s+/g, "-")}` : "~");

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="terminal bar">
      <a className="start" href={href(bind.launch)} {...bind.launch()} aria-label="Menu">
        <img src={site.logo} width="14" height="14" alt="" />
        <b>{site.name.toLowerCase()}</b>
      </a>
      <nav className="trio" aria-label="Tasks">
        <span className="prompt" aria-hidden="true">
          :{path(app)}$
        </span>
        <span className="task app" hidden={!app}>
          {app}
        </span>
        <span className="cursor" aria-hidden="true" />
      </nav>
      <div className="tray">
        {site.cart && (
          <a className="cart" href={href(bind.cart)} {...bind.cart()} hidden={cart < 1} aria-label={`Cart, ${cart}`}>
            cart:{cart}
          </a>
        )}
        <a className="glyph find" href={href(bind.find)} {...bind.find()} aria-label="Search">
          [/]
        </a>
        <time className="date" hidden />
        <time className="clock">{hhmm(clock)}</time>
      </div>
    </header>
  );
}
