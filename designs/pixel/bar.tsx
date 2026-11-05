import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { caps, day, href, time } from "./kit.ts";
import { Mosaic, Sprite } from "./sprite.tsx";

export function Bar({ site, app, cart, clock, bind }: Props["bar"]) {
  return (
    <header className="pixel bar">
      <Mosaic />
      <a className="start" href={href(bind.launch)} {...bind.launch()} aria-label="Launchpad">
        <Sprite art="pac" size={16} />
        <span className="word">START</span>
      </a>
      <a className="mark" href={href(bind.mark)} {...bind.mark()} aria-label={`${site.name} home`}>
        <img src={site.logo} width="18" height="18" alt="" />
        <span className="word">{caps(site.name)}</span>
      </a>
      {app && (
        <span className="app">
          <Sprite art="right" size={12} />
          {caps(app)}
        </span>
      )}
      <nav className="menus" role="menubar" aria-label="App" {...bind.menus()} />
      <div className="tray">
        {site.cart && (
          <a className="cart" href={href(bind.cart)} {...bind.cart()} hidden={cart < 1} aria-label={`Cart, ${cart}`}>
            <span className="label">CREDIT</span>
            <b>{cart}</b>
          </a>
        )}
        <a className="find" href={href(bind.find)} {...bind.find()} aria-label="Search">
          <Sprite art="find" size={16} />
        </a>
        <time className="date">{clock ? day(clock) : ""}</time>
        <time className="clock">{clock ? time(clock) : ""}</time>
      </div>
    </header>
  );
}
