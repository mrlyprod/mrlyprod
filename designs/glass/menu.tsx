import type { IconName } from "../../net/sdk/icon.tsx";
import { Icon } from "../../net/sdk/icon.tsx";
import type { Props } from "../../net/shell/design.ts";

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
