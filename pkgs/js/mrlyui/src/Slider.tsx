import { cx } from "./lib";

export function Slider({
  min,
  max,
  step = 1,
  value,
  onChange,
  onCommit,
  format,
}: {
  min: number;
  max: number;
  step?: number;
  value: number;
  onChange: (value: number) => void;
  onCommit?: (value: number) => void;
  format?: (value: number) => string;
}) {
  return (
    <div className="slider">
      <input
        className="slider-track"
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(event) => onChange(Number(event.target.value))}
        onPointerUp={(event) => onCommit?.(Number(event.currentTarget.value))}
        onPointerCancel={(event) => onCommit?.(Number(event.currentTarget.value))}
        onKeyUp={(event) => onCommit?.(Number(event.currentTarget.value))}
      />
      <span className="slider-value">{format ? format(value) : value}</span>
    </div>
  );
}

export function StepSlider<T extends string>({
  steps,
  value,
  onChange,
}: {
  steps: { label: string; value: T }[];
  value: T;
  onChange: (value: T) => void;
}) {
  const index = Math.max(0, steps.findIndex((step) => step.value === value));
  const pick = (next: number) => {
    const step = steps[next];
    if (step) onChange(step.value);
  };
  return (
    <div className="step-slider">
      <input
        className="slider-track"
        type="range"
        min={0}
        max={Math.max(0, steps.length - 1)}
        step={1}
        value={index}
        aria-valuetext={steps[index]?.label}
        onChange={(event) => pick(Number(event.target.value))}
      />
      <div className="step-ticks">
        {steps.map((step, i) => (
          <button
            key={step.value}
            type="button"
            className={cx("step-tick", i === index && "active")}
            onClick={() => onChange(step.value)}
          >
            {step.label}
          </button>
        ))}
      </div>
    </div>
  );
}
