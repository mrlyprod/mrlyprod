import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BRICKS, day, paint, time } from "./brick.ts";

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="bar tower" {...BRICKS} data-app={app ?? undefined}>
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
        <span className="menus" role="menubar" aria-label="App" aria-orientation="vertical" {...bind.menus()} />
        <a
          className="glyph find brick"
          style={paint("yellow")}
          href={href(bind.find)}
          {...bind.find()}
          aria-label="Search"
        >
          <Icon name="search" size={24} />
        </a>
        {site.cart && (
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
        )}
      </nav>
      <div className="tray brick" style={paint("white")}>
        <time className="date">{clock && day(clock)}</time>
        <time className="clock">{clock && time(clock)}</time>
      </div>
    </header>
  );
}
