import { Icon } from "../../lib/app/icon.tsx";
import type { Props } from "../../shell/design.ts";
import { BLOCKS, day, href, paint, time } from "./brick.ts";

/* BAR */

export function Bar({ site, app, menus, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="bar tower" {...BLOCKS} data-app={app ?? undefined}>
      <nav className="trio" aria-label="System">
        <a
          className="mark brick"
          style={paint("black")}
          href={href(bind.mark)}
          {...bind.mark()}
          aria-label={`${site.name} home`}
        >
          <img src={site.logo} alt="" />
        </a>
        <a className="launch brick" style={paint("white")} href={href(bind.launch)} {...bind.launch()}>
          <span className="print">Menu</span>
        </a>
        <span className="menus" role="menubar" aria-label="App" aria-orientation="vertical">
          {menus.map(({ name, bind: one }) => (
            <button type="button" className="brick" style={paint("blue")} key={name} {...one()}>
              <span className="print">{name}</span>
            </button>
          ))}
        </span>
        <a
          className="glyph find brick"
          style={paint("yellow")}
          href={href(bind.find)}
          {...bind.find()}
          aria-label="Search"
        >
          <Icon name="search" size={24} />
        </a>
        <a
          className="cart brick"
          style={paint("red")}
          href={href(bind.cart)}
          {...bind.cart()}
          data-cart={cart > 0}
          aria-label={`Cart, ${cart}`}
        >
          <span className="print">Cart</span>
          <span className="count">{cart}</span>
        </a>
      </nav>
      <div className="tray brick" style={paint("white")}>
        <time className="date">{day(clock)}</time>
        <time className="clock">{time(clock)}</time>
      </div>
    </header>
  );
}
