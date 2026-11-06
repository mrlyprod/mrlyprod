import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Tabs({ items, bind, children }: Props["tabs"]) {
  return (
    <div className="tabset">
      <div className="strip" {...bind.list()}>
        {items.map((one) => (
          <button type="button" key={one.id} {...bind.tab(one.id)()}>
            {one.icon && <Icon name={one.icon} size={16} />}
            <span>{one.label}</span>
          </button>
        ))}
      </div>
      <div {...bind.panel()}>{children}</div>
    </div>
  );
}
