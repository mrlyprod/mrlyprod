import type { Props } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";

const chord = (keys: string[], mac: boolean) => keys.join(mac ? "" : "+");

export function Menu({ name, items, anchor, mac, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at: CSSProperties = anchor ? { position: "fixed", top: anchor.bottom + 5, left: anchor.left } : {};
  return (
    <div role="menu" aria-label={name} {...list} className="menu" style={{ ...at, ...(style as CSSProperties) }}>
      {items.map((item, i) => {
        if (!item) return <hr key={`after ${items[i - 1]?.label ?? i}`} />;
        const body = (
          <>
            <span className="tick">{item.on ? "*" : ""}</span>
            <span className="label">{item.label}</span>
            {item.keys && <kbd>{chord(item.keys, mac)}</kbd>}
          </>
        );
        const own = {
          className: "item",
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
