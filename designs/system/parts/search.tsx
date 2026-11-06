import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Search({ label, value, placeholder = "search", onChange, bind }: Props["search"]) {
  return (
    <div className="hunt" role="search">
      <Icon name="search" size={14} />
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
          <Icon name="close" size={14} />
        </button>
      )}
    </div>
  );
}
