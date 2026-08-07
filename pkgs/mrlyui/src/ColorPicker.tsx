import type { CSSProperties } from "react";
import { cx } from "./lib";
import { POOL, css } from "./colors";
import type { ColorName } from "./colors";

export function ColorPicker({
  value = null,
  onChange,
  auto,
}: {
  value?: ColorName | null;
  onChange: (color: ColorName | null) => void;
  auto?: boolean;
}) {
  return (
    <div className="swatches">
      {auto && (
        <button
          type="button"
          className={cx("swatch", "auto", value === null && "active")}
          aria-pressed={value === null}
          onClick={() => onChange(null)}
        >
          AUTO
        </button>
      )}
      {POOL.map((name) => (
        <button
          key={name}
          type="button"
          className={cx("swatch", value === name && "active")}
          style={{ "--swatch-color": css(name) } as CSSProperties}
          aria-label={name}
          aria-pressed={value === name}
          title={name}
          onClick={() => onChange(name)}
        />
      ))}
    </div>
  );
}
