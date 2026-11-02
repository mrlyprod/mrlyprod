import type { CSSProperties } from "react";
import { Icon } from "../../net/sdk/icon.tsx";
import type { Props } from "../../net/shell/design.ts";
import { BLOCKS } from "./brick.ts";

/* MENU */

const GAP = 8;

const chord = (keys: string[], mac: boolean) => keys.join(mac ? "" : "+");

export function Menu({ name, items, anchor, mac, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at: CSSProperties = anchor ? { position: "fixed", top: anchor.top, left: anchor.right + GAP } : {};
  return (
    <div
      role="menu"
      aria-label={name}
      {...list}
      className="menu plate"
      {...BLOCKS}
      style={{ ...at, ...(style as CSSProperties) }}
    >
      {items.map((item, i) => {
        if (!item) return <hr key={`after ${items[i - 1]?.label ?? i}`} className="studs" />;
        const body = (
          <>
            <span className="tick">{item.on && <Icon name="check" size={20} />}</span>
            <span className="label">{item.label}</span>
            {item.keys && <kbd>{chord(item.keys, mac)}</kbd>}
          </>
        );
        const own = {
          className: "item tile",
          "data-act": item.act,
          "aria-disabled": item.off || undefined,
          ...bind.item(i)(),
        };
        return item.href ? (
          <a key={item.label} href={item.href} {...own}>
            {body}
          </a>
        ) : (
          <button key={item.label} type="button" {...own}>
            {body}
          </button>
        );
      })}
    </div>
  );
}
