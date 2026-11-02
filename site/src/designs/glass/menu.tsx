import type { IconName } from "../../lib/app/icon.tsx";
import { Icon } from "../../lib/app/icon.tsx";
import type { Props } from "../../shell/design.ts";

/* MENU */

export function Menu({ items, bind }: Props["menu"]) {
  const { style, ...list } = bind.list();
  return (
    <div {...list} className="pop" style={style as object}>
      {items.map((item, i) =>
        item ? (
          <button type="button" key={item.label} {...bind.item(i)()}>
            {item.icon && <Icon name={item.icon as IconName} />}
            {item.label}
          </button>
        ) : (
          <hr key={`after ${items[i - 1]?.label ?? ""}`} />
        ),
      )}
    </div>
  );
}
