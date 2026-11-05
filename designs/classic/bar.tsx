import { stamp } from "@mrlyprod/ui/kit/clock.ts";
import { href } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "./glyphs.tsx";

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="classic bar">
      <a className="raised start" href={href(bind.launch)} {...bind.launch()} aria-label="Start">
        <img src={site.logo} width="16" height="16" alt="" />
        <b>{site.name}</b>
      </a>
      <nav className="tasks" aria-label="Tasks">
        {app && <span className="pressed task">{app}</span>}
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
        <time className="clock">{clock && stamp(clock.now, String(clock.hours ?? ""))}</time>
      </div>
    </header>
  );
}
