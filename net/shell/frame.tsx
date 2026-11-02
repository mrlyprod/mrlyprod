import { parts } from "./active.ts";
import type { Side } from "./design.ts";
import { measure } from "./measure.ts";

/* BINDS */

export const BAR = {
  launch: () => ({ href: "/menu/" }),
  mark: () => ({ href: "/" }),
  cart: () => ({ href: "/cart/", "data-cart": "true" }),
  find: () => ({ href: "/menu/" }),
  menus: () => ({ "data-menus": "" }),
};

const WINDOW = {
  close: () => ({ href: "/", "data-close": "" }),
  zoom: () => ({ "data-zoom": "" }),
  tools: () => ({
    "data-pane": "right",
    "data-tools": "",
    "aria-controls": "right",
    "aria-expanded": "false",
    "data-few": "3",
  }),
  title: () => ({ "data-title": "" }),
  menus: () => ({ "data-menus": "" }),
  edge: (side: Side) => () => ({ "data-edge": side }),
  scroll: () => ({ id: "scroll" }),
  main: () => ({ id: "main", tabIndex: -1 }),
  pane: () => ({ id: "right" }),
  scrim: () => ({ "data-scrim": "" }),
};

/* FRAME */

export function Frame({ title, mark, word, cart }: { title: string; mark: string; word: string; cart?: boolean }) {
  return (
    <>
      <a className="skip" href="#main">
        Skip to content
      </a>
      {"\n"}
      <div className="barred" data-cart={cart || undefined}>
        <parts.bar site={{ name: title, logo: mark, word, cart }} app={null} cart={0} clock={null} bind={BAR} />
      </div>
      {"\n"}
      {parts.lock && (
        <>
          <div className="welcome" aria-hidden="true" />
          {"\n"}
        </>
      )}
      <parts.window kind="page" title="" zoomed={false} tools={false} drawer={false} width={measure.page} bind={WINDOW}>
        {null}
      </parts.window>
    </>
  );
}
