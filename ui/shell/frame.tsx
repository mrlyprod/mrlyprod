import type { Design } from "./design.ts";

type FrameProps = { design: Design; title: string; mark: string; word: string; cart?: boolean };

export const BAR = {
  launch: () => ({ href: "/menu/" }),
  mark: () => ({ href: "/" }),
  cart: () => ({ href: "/cart/", "data-cart": "true" }),
  find: () => ({ href: "/menu/" }),
  menus: () => ({ "data-menus": "" }),
};

const WINDOW = {
  close: () => ({ href: "/" }),
  title: () => ({ "data-title": "" }),
  menus: () => ({ "data-menus": "" }),
  main: () => ({ id: "main", tabIndex: -1 }),
  pane: () => ({ id: "right" }),
};

export function Frame({ design: { parts }, title, mark, word, cart }: FrameProps) {
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
      <parts.window bind={WINDOW} />
    </>
  );
}
