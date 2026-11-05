import type { IconName } from "@mrlyprod/ui/kit/icon.tsx";
import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";

const spot = (anchor: DOMRect) => ({ "--x": `${anchor.left}px`, "--y": `${anchor.bottom}px` }) as CSSProperties;

export function Menu({ items, anchor, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  const at = anchor ? { className: "pop menu", "data-lens": "menu", style: spot(anchor) } : null;
  return (
    <div {...list} className="pop" style={style as CSSProperties} {...at}>
      {items.map((item, i) => {
        if (!item) return <hr key={`after ${items[i - 1]?.label ?? ""}`} />;
        const body = (
          <>
            {item.icon && <Icon name={item.icon as IconName} />}
            {item.label}
          </>
        );
        return item.href ? (
          <a className="button" key={item.label} href={item.href} {...bind.item(i)()}>
            {body}
          </a>
        ) : (
          <button type="button" key={item.label} {...bind.item(i)()}>
            {body}
          </button>
        );
      })}
    </div>
  );
}
