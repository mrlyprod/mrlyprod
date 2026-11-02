import { useId } from "react";
import { classes } from "../../net/sdk/room.ts";
import type { Props, Scalar } from "../../net/shell/design.ts";
import { gauge } from "./banner.ts";

/* CONTROL */

type Control = Props["control"];

const CELLS = 16;

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const clamp = (v: number, min = -Infinity, max = Infinity) => Math.min(max, Math.max(min, v));

function Slider({ label, value, text, onChange, onCommit, min = 0, max = 100, step = 1, zero, unit, bind }: Control) {
  const word = said(zero, value);
  const at = max > min ? (Number(value) - min) / (max - min) : 0;
  return (
    <label className="terminal control slider">
      <span className="label">{label}</span>
      <span className="track">
        <span className="gauge" aria-hidden="true">
          {gauge(at, CELLS)}
        </span>
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange?.(+e.target.value)}
          onPointerUp={onCommit}
          {...bind?.()}
        />
      </span>
      <span className="num">
        {word ?? text ?? value}
        {unit && !word ? ` ${unit}` : ""}
      </span>
    </label>
  );
}

function Spin({ label, value, onChange, onCommit, min, max, step = 1, bind }: Control) {
  const nudge = (by: number) => {
    onChange?.(clamp(Number(value) + by * step, min, max));
    onCommit?.();
  };
  return (
    <label className="terminal control number">
      <span className="label">{label}</span>
      <span className="spin">
        <button type="button" className="knob" tabIndex={-1} aria-label="Less" onClick={() => nudge(-1)}>
          [-]
        </button>
        <input
          type="text"
          inputMode="decimal"
          className="field"
          value={value}
          onChange={(e) => onChange?.(e.target.value === "" ? "" : clamp(+e.target.value, min, max))}
          onBlur={onCommit}
          {...bind?.()}
        />
        <button type="button" className="knob" tabIndex={-1} aria-label="More" onClick={() => nudge(1)}>
          [+]
        </button>
      </span>
    </label>
  );
}

function Text({ label, value, onChange, onCommit, wide, zero, bind }: Control) {
  const word = said(zero, value);
  return (
    <label className={classes("terminal", "control", "text", wide ? "wide" : undefined)}>
      <span className="label">{label}</span>
      <input
        type="text"
        className="field"
        value={value}
        placeholder={word ?? undefined}
        onChange={(e) => onChange?.(e.target.value)}
        onBlur={onCommit}
        {...bind?.()}
      />
    </label>
  );
}

function Toggle({ label, value, onChange, bind }: Control) {
  const on = Number(value) === 1;
  return (
    <label className="terminal control toggle">
      <span className="check">
        <input
          type="checkbox"
          role="switch"
          checked={on}
          aria-checked={on}
          onChange={(e) => onChange?.(e.target.checked ? 1 : 0)}
          {...bind?.()}
        />
        <span className="box" aria-hidden="true" />
      </span>
      <span className="label">{label}</span>
    </label>
  );
}

function Pick({ label, value, onChange, options = [], bind }: Control) {
  return (
    <label className="terminal control pick">
      <span className="label">{label}</span>
      <span className="drop">
        <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...bind?.()}>
          {options.map(([v, text]) => (
            <option key={v} value={v}>
              {text}
            </option>
          ))}
        </select>
        <span className="arrow" aria-hidden="true">
          v
        </span>
      </span>
    </label>
  );
}

function Segment({ label, value, onChange, options = [], bind }: Control) {
  const name = useId();
  return (
    <div
      className={classes("terminal", "control", "segment", options.length > 4 ? "stack" : undefined)}
      role="radiogroup"
      aria-label={typeof label === "string" ? label : undefined}
      {...bind?.()}
    >
      <span className="label">{label}</span>
      <span className="options">
        {options.map(([v, text]) => (
          <label key={v}>
            <input
              type="radio"
              name={name}
              value={v}
              checked={String(v) === String(value)}
              onChange={() => onChange?.(v)}
            />
            <span className="dot" aria-hidden="true" />
            <span>{text}</span>
          </label>
        ))}
      </span>
    </div>
  );
}

function Button({ primary, on, tag, bind, children, label }: Control) {
  return (
    <button
      type="button"
      className={classes(
        "terminal",
        "control",
        "button",
        on ? "pressed" : undefined,
        primary ? "primary" : undefined,
        tag,
      )}
      aria-pressed={on}
      {...bind?.()}
    >
      {children ?? label}
    </button>
  );
}

const KINDS = {
  slider: Slider,
  number: Spin,
  toggle: Toggle,
  pick: Pick,
  segment: Segment,
  text: Text,
  button: Button,
};

export function Control(props: Control) {
  const Kind = KINDS[props.kind];
  return <Kind {...props} />;
}
