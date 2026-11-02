import { parts } from "./active.ts";
import { measure } from "./measure.ts";

/* BINDS */

const none = () => ({});

const BAR = {
  launch: () => ({ href: "/menu/" }),
  mark: () => ({ href: "/" }),
  cart: () => ({ href: "/cart/", "data-cart": "true" }),
  find: () => ({ href: "/menu/" }),
};

const WINDOW = {
  close: () => ({ href: "/" }),
  zoom: none,
  tools: () => ({
    "data-pane": "right",
    "aria-controls": "right",
    "aria-expanded": "false",
    "data-few": "3",
  }),
  edge: () => none,
  scroll: () => ({ id: "scroll" }),
  main: () => ({ id: "main", tabIndex: -1 }),
  pane: () => ({ id: "right" }),
  scrim: none,
};

/* FRAME */

export function Frame({ title, mark, word, cart }: { title: string; mark: string; word: string; cart?: boolean }) {
  return (
    <>
      <a className="skip" href="#main">
        Skip to content
      </a>
      {"\n"}
      <parts.bar
        site={{ name: title, logo: mark, word, cart }}
        app={null}
        menus={[]}
        cart={0}
        clock={{ now: new Date(0), hours: null }}
        bind={BAR}
      />
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
