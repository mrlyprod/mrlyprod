import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Search({ label, value, placeholder = "Search", onChange, bind }: Props["search"]) {
  return (
    <div className="seek" role="search">
      <Icon name="search" size={16} />
      <input
        type="search"
        value={value}
        placeholder={placeholder}
        aria-label={label}
        onChange={(e) => onChange(e.target.value)}
        {...bind.field()}
      />
      {value && (
        <button type="button" className="clear" {...bind.clear()}>
          <Icon name="close" size={12} />
        </button>
      )}
    </div>
  );
}
