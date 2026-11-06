import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Search({ label, value, placeholder = "Search", onChange, bind }: Props["search"]) {
  return (
    <div className="sunken lookup" role="search">
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
        <button type="button" className="raised knob" {...bind.clear()}>
          <Glyph art="close" />
        </button>
      )}
    </div>
  );
}
