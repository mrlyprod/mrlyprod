import type { ReactNode } from "react";
import { cx } from "./lib";
import { Symbol } from "./Glyphs";

export function Checkbox({
  checked,
  onChange,
  label,
  disabled,
}: {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label?: ReactNode;
  disabled?: boolean;
}) {
  return (
    <button
      type="button"
      role="checkbox"
      aria-checked={checked}
      className={cx("checkbox", checked && "checked")}
      onClick={() => onChange(!checked)}
      disabled={disabled}
    >
      <span className="checkbox-box" aria-hidden="true">
        <span className="checkbox-check">
          <Symbol name="check" size="var(--font-size)" />
        </span>
      </span>
      {label && <span>{label}</span>}
    </button>
  );
}
