import { label } from "@mrlyprod/ui/kit/menus.ts";
import type { Item, Props } from "@mrlyprod/ui/shell/design.ts";
import { cx } from "./kit.ts";
import { Sprite } from "./sprite.tsx";

export const chord = (keys: string[], mac: boolean) =>
  keys
    .map((key) => (key === "mod" ? (mac ? "\u2318" : "CTRL") : key === "shift" ? "\u21e7" : label(key)))
    .join(mac ? "" : "+");

const role = (item: Item) => (item.radio ? "menuitemradio" : item.on !== undefined ? "menuitemcheckbox" : "menuitem");

export function Menu({ name, items, anchor, mac, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at = anchor ? { left: Math.round(anchor.left), top: Math.round(anchor.bottom) } : {};
  return (
    <div className="pixel menu" role="menu" aria-label={name} {...list} style={{ ...at, ...(style as object) }}>
      {items.map((item, i) =>
        item ? (
          <button
            type="button"
            key={item.label}
            className={cx("item", item.off && "off")}
            role={role(item)}
            {...(item.on === undefined ? {} : { "aria-checked": item.on })}
            aria-disabled={item.off || undefined}
            data-act={item.act}
            {...bind.item(i)()}
          >
            <span className="pointer" aria-hidden="true">
              <Sprite art="pac" size={12} />
            </span>
            <span className="tick" aria-hidden="true">
              {item.on && <Sprite art={item.radio ? "pellet" : "check"} size={12} />}
            </span>
            <span className="label">{item.label}</span>
            {item.keys?.length ? <span className="chord">{chord(item.keys, mac)}</span> : null}
          </button>
        ) : (
          <hr key={`after ${items[i - 1]?.label ?? ""}`} />
        ),
      )}
    </div>
  );
}
