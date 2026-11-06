import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Progress({ value, max, label }: Props["progress"]) {
  if (value === null)
    return (
      <span className="gauge" role="progressbar" aria-label={label}>
        <span>{label}</span>
        <span className="sunken sweep">
          <i />
        </span>
      </span>
    );
  return (
    <label className="gauge">
      <span>{label}</span>
      <progress className="sunken" value={value} max={max} />
    </label>
  );
}
