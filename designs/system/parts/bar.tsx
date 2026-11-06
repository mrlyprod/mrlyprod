import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { day, time } from "../own/cell.ts";
import { Word } from "../own/word.tsx";

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="bar">
      <a className="mark cell" href={href(bind.mark)} {...bind.mark()} aria-label={`${site.name} home`}>
        <Word text={site.name} />
      </a>
      <nav className="trio" aria-label="System">
        {app && <span className="app cell">{app}</span>}
        <span className="menus" role="menubar" aria-label="App" {...bind.menus()} />
        {site.cart && (
          <a className="cart cell" href={href(bind.cart)} {...bind.cart()} aria-label="Cart" hidden={cart <= 0}>
            {cart}
          </a>
        )}
      </nav>
      <div className="tray">
        <time className="date cell">{clock && day(clock)}</time>
        <time className="clock cell">{clock && time(clock)}</time>
        <a className="glyph find cell" href={href(bind.find)} {...bind.find()} aria-label="Search">
          find
        </a>
      </div>
    </header>
  );
}
