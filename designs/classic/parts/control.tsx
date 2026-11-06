import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props, Scalar } from "@mrlyprod/ui/shell/design.ts";
import { useId } from "react";
import { Glyph } from "../own/glyphs.tsx";

type Control = Props["control"];

const FEW = 20;

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const clamp = (v: number, min = -Infinity, max = Infinity) => Math.min(max, Math.max(min, v));

function Slider({ label, value, text, onChange, onCommit, min = 0, max = 100, step = 1, zero, unit, bind }: Control) {
  const word = said(zero, value);
  const steps = Math.round((max - min) / step);
  return (
    <label className="control slider">
      <span className="label">{label}</span>
      <span className="track">
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
        {steps > 0 && steps < FEW && (
          <span className="ticks" aria-hidden="true">
            {Array.from({ length: steps + 1 }, (_, n) => min + n * step).map((at) => (
              <i key={at} />
            ))}
          </span>
        )}
      </span>
      <span className="sunken field num">
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
    <label className="control number">
      <span className="label">{label}</span>
      <span className="sunken field spin">
        <input
          type="text"
          inputMode="decimal"
          value={value}
          onChange={(e) => onChange?.(e.target.value === "" ? "" : clamp(+e.target.value, min, max))}
          onBlur={onCommit}
          {...bind?.()}
        />
        <span className="arrows">
          <button type="button" className="raised" tabIndex={-1} aria-label="More" onClick={() => nudge(1)}>
            <Glyph art="up" />
          </button>
          <button type="button" className="raised" tabIndex={-1} aria-label="Less" onClick={() => nudge(-1)}>
            <Glyph art="down" />
          </button>
        </span>
      </span>
    </label>
  );
}

function Text({ label, value, onChange, onCommit, wide, zero, bind }: Control) {
  const word = said(zero, value);
  return (
    <label className={classes("control", "text", wide ? "wide" : undefined)}>
      <span className="label">{label}</span>
      <input
        type="text"
        className="sunken field"
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
    <label className="control toggle">
      <span className="sunken check">
        <input
          type="checkbox"
          role="switch"
          checked={on}
          aria-checked={on}
          onChange={(e) => onChange?.(e.target.checked ? 1 : 0)}
          {...bind?.()}
        />
        <Glyph art="tick" />
      </span>
      <span className="label">{label}</span>
    </label>
  );
}

function Pick({ label, value, onChange, options = [], bind }: Control) {
  return (
    <label className="control pick">
      <span className="label">{label}</span>
      <span className="sunken field drop">
        <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...bind?.()}>
          {options.map(([v, text]) => (
            <option key={v} value={v}>
              {text}
            </option>
          ))}
        </select>
        <span className="raised arrow" aria-hidden="true">
          <Glyph art="down" />
        </span>
      </span>
    </label>
  );
}

function Segment({ label, value, onChange, options = [], bind }: Control) {
  const name = useId();
  return (
    <div
      className={classes("control", "segment", options.length > 4 ? "stack" : undefined)}
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
              className="sunken"
              name={name}
              value={v}
              checked={String(v) === String(value)}
              onChange={() => onChange?.(v)}
            />
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
      className={classes("control", "button", on ? "pressed" : "raised", primary ? "primary" : undefined, tag)}
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
