import { stamp } from "../../net/sdk/clock.ts";
import type { Props } from "../../net/shell/design.ts";
import { href } from "../shared/kit.ts";
import { Glyph } from "./glyph.tsx";

/* BAR */

const two = (n: number) => String(n).padStart(2, "0");

const DAY = { weekday: "short", day: "numeric", month: "short" } as const;

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  const now = clock?.now;
  return (
    <header className="bar">
      <nav className="trio" data-lens="bar" aria-label="System">
        <a className="glyph launch" href={href(bind.launch)} {...bind.launch()} aria-label="Launchpad">
          <Glyph art="launch" />
        </a>
        <a className="mark" href={href(bind.mark)} {...bind.mark()} aria-label={`${site.name} home`}>
          <img className="logo" src={site.logo} width="20" height="20" alt="" />
        </a>
        {site.cart && (
          <a
            className="glyph"
            href={href(bind.cart)}
            {...bind.cart()}
            aria-label={cart ? `Cart, ${cart} item${cart === 1 ? "" : "s"}` : "Cart"}
          >
            <Glyph art="cart" lit={cart} />
          </a>
        )}
        <span className="app">{app}</span>
        <span className="menus" role="menubar" aria-label="App" {...bind.menus()} />
      </nav>
      <div className="tray" data-lens="bar">
        <time
          className="date"
          dateTime={now && `${now.getFullYear()}-${two(now.getMonth() + 1)}-${two(now.getDate())}`}
        >
          {now?.toLocaleDateString(undefined, DAY)}
        </time>
        <time className="clock" dateTime={now && `${two(now.getHours())}:${two(now.getMinutes())}`}>
          {clock && stamp(clock.now, String(clock.hours ?? ""))}
        </time>
        <a className="glyph find" href={href(bind.find)} {...bind.find()} aria-label="Search">
          <Glyph art="find" />
        </a>
      </div>
    </header>
  );
}
