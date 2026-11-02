import type { Props } from "../../shell/design.ts";
import { day, href, MRLY, time } from "./cell.ts";
import { Word } from "./word.tsx";

/* BAR */

export function Bar({ site, app, menus, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="bar" {...MRLY}>
      <a className="mark cell" href={href(bind.mark)} {...bind.mark()} aria-label={`${site.name} home`}>
        <Word text={site.name} />
      </a>
      <nav className="trio" aria-label="System">
        {app && <span className="app cell">{app}</span>}
        <span className="menus" role="menubar" aria-label="App">
          {menus.map(({ name, bind: one }) => (
            <button type="button" className="cell" key={name} {...one()}>
              {name}
            </button>
          ))}
        </span>
        <a className="cart cell" href={href(bind.cart)} {...bind.cart()} aria-label="Cart" hidden={cart <= 0}>
          {cart}
        </a>
      </nav>
      <div className="tray">
        <time className="date cell">{day(clock)}</time>
        <time className="clock cell">{time(clock)}</time>
        <a className="glyph find cell" href={href(bind.find)} {...bind.find()} aria-label="Search">
          find
        </a>
      </div>
    </header>
  );
}
