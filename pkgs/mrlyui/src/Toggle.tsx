import { cx } from "./lib";

export function Toggle({
  value,
  onChange,
  disabled,
}: {
  value: boolean;
  onChange: (value: boolean) => void;
  disabled?: boolean;
}) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={value}
      className={cx("toggle", value && "on")}
      onClick={() => onChange(!value)}
      disabled={disabled}
    >
      <span className="toggle-knob" />
    </button>
  );
}
