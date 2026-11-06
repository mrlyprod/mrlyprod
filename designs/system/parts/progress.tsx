import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Progress({ value, max, label }: Props["progress"]) {
  if (value === null)
    return (
      <span className="loading cell" role="progressbar" aria-label={label}>
        <span className="dots" aria-hidden="true">
          <i />
          <i />
          <i />
        </span>
        {label}
      </span>
    );
  return (
    <label className="loadbar">
      <span className="cell">{label}</span>
      <progress value={value} max={max} />
    </label>
  );
}
