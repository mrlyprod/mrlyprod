import { rows, site } from "site:routes";
import { useEffect, useState, useSyncExternalStore } from "react";
import { parts } from "./active.ts";
import { find, launch } from "./catalog.ts";
import { knock } from "./desk.ts";
import { BAR } from "./frame.tsx";
import { host } from "./menus.ts";
import { show } from "./parts.tsx";
import { cycle, KEY, read, root, subscribe } from "./prefs.ts";
import { match } from "./route.ts";
import { WORD } from "./welcome.ts";

type Carted = { qty?: unknown };

const MINUTE = 60000;

const BIND = {
  ...BAR,
  launch: () => ({ ...BAR.launch(), onClick: launch }),
  mark: () => ({ ...BAR.mark(), onClick: knock }),
  find: () => ({ ...BAR.find(), onClick: find }),
  menus: () => ({ ...BAR.menus(), ref: host }),
};

function count() {
  try {
    const items: unknown = JSON.parse(read(KEY.cart) || "[]");
    return Array.isArray(items) ? items.reduce((sum: number, item: Carted) => sum + (Number(item.qty) || 1), 0) : 0;
  } catch {
    return 0;
  }
}

function heard(fn: () => void) {
  const off = subscribe(fn);
  for (const name of ["cart", "storage", "pageshow"]) addEventListener(name, fn);
  return () => {
    off();
    for (const name of ["cart", "storage", "pageshow"]) removeEventListener(name, fn);
  };
}

function useNow() {
  const [now, setNow] = useState(() => new Date());
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | 0 = 0;
    const tick = () => {
      const at = new Date();
      setNow(at);
      clearTimeout(timer);
      timer = setTimeout(tick, MINUTE - (at.getSeconds() * 1000 + at.getMilliseconds()) + 50);
    };
    const back = () => !document.hidden && tick();
    tick();
    document.addEventListener("visibilitychange", back);
    return () => {
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", back);
    };
  }, []);
  return now;
}

const titled = () => (root().dataset.window === "app" ? match(rows, location.pathname).title : null);

function useApp() {
  const [app, setApp] = useState(titled);
  useEffect(() => {
    const wire = () => setApp(titled());
    addEventListener("wire", wire);
    return () => removeEventListener("wire", wire);
  }, []);
  return app;
}

function Bar({ cart }: { cart: boolean }) {
  const now = useNow();
  const app = useApp();
  const hours = useSyncExternalStore(subscribe, cycle);
  const items = useSyncExternalStore(heard, count);
  return (
    <parts.bar
      site={{ name: site.title, logo: site.mark, word: WORD, cart }}
      app={app}
      cart={items}
      clock={{ now, hours }}
      bind={BIND}
    />
  );
}

export function start() {
  const box = document.querySelector<HTMLElement>(".barred");
  if (!box) return;
  show(box, <Bar cart={box.hasAttribute("data-cart")} />);
  dispatchEvent(new CustomEvent("shift", { detail: 0 }));
}
