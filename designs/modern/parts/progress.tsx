import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Progress({ value, max, label }: Props["progress"]) {
  if (value === null)
    return (
      <span className="busy" role="progressbar" aria-label={label}>
        <span className="spinner" />
        {label}
      </span>
    );
  return (
    <label className="gauge">
      <span>{label}</span>
      <progress value={value} max={max} />
    </label>
  );
}
