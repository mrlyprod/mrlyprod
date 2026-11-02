import type { ReactNode } from "react";
import { label } from "../../net/sdk/menus.ts";
import { classes } from "../../net/sdk/room.ts";
import type { Item, Props } from "../../net/shell/design.ts";
import { Glyph } from "./glyphs.tsx";

/* CHORD */

function chord(keys: string[], mac: boolean): ReactNode[] {
  return keys.flatMap((key, n) => {
    const one =
      mac && key === "mod" ? (
        <Glyph key={key} art="cmd" />
      ) : mac && key === "shift" ? (
        <Glyph key={key} art="shift" />
      ) : key === "mod" ? (
        "Ctrl"
      ) : key === "shift" ? (
        "Shift"
      ) : (
        label(key)
      );
    return n && !mac ? ["+", one] : [one];
  });
}

const role = (item: Item) => (item.radio ? "menuitemradio" : item.on !== undefined ? "menuitemcheckbox" : "menuitem");

/* MENU */

export function Menu({ name, items, anchor, mac, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at = anchor ? { left: Math.round(anchor.left), top: Math.round(anchor.bottom) } : {};
  return (
    <div
      className="classic menu raised"
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
            <span className="gutter">{item.on && <Glyph art={item.radio ? "dot" : "tick"} />}</span>
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
