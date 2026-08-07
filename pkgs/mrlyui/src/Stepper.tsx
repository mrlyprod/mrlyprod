import { Symbol } from "./Glyphs";

export function Stepper({
  value,
  onChange,
  min = -Infinity,
  max = Infinity,
  step = 1,
  format,
}: {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  step?: number;
  format?: (value: number) => string;
}) {
  const set = (next: number) => onChange(Math.min(max, Math.max(min, next)));
  return (
    <div className="stepper">
      <button
        type="button"
        className="stepper-button"
        aria-label="Decrease"
        disabled={value <= min}
        onClick={() => set(value - step)}
      >
        <Symbol name="remove" />
      </button>
      <span className="stepper-value">{format ? format(value) : value}</span>
      <button
        type="button"
        className="stepper-button"
        aria-label="Increase"
        disabled={value >= max}
        onClick={() => set(value + step)}
      >
        <Symbol name="add" />
      </button>
    </div>
  );
}
