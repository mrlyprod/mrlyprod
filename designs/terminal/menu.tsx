import { label } from "@mrlyprod/ui/kit/menus.ts";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Item, Props } from "@mrlyprod/ui/shell/design.ts";

export function chord(keys: string[], mac: boolean) {
  return keys.map((key) => (key === "mod" ? (mac ? "Cmd" : "Ctrl") : key === "shift" ? "Shift" : label(key))).join("+");
}

export const mark = (item: Pick<Item, "radio" | "on">) =>
  item.radio ? (item.on ? "(*)" : "( )") : item.on ? "[x]" : "";

const role = (item: Item) => (item.radio ? "menuitemradio" : item.on !== undefined ? "menuitemcheckbox" : "menuitem");

export function Menu({ name, items, anchor, mac, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at = anchor ? { left: Math.round(anchor.left), top: Math.round(anchor.bottom) } : {};
  return (
    <div
      className="terminal menu frame"
      role="menu"
      aria-label={name}
      {...list}
      style={{ ...at, ...(style as object) }}
    >
      {items.map((item, i) =>
        item ? (
          <button
            type="button"
            key={item.label}
            className={classes("item", item.off ? "off" : undefined)}
            role={role(item)}
            {...(item.on === undefined ? {} : { "aria-checked": item.on })}
            aria-disabled={item.off || undefined}
            data-act={item.act}
            {...bind.item(i)()}
          >
            <span className="gutter">{mark(item)}</span>
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
