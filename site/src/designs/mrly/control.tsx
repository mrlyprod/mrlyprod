import type { ReactNode } from "react";
import { useId } from "react";
import { classes, useRoom } from "../../lib/app/room.ts";
import type { Props, Scalar } from "../../shell/design.ts";
import { MRLY } from "./cell.ts";

/* CONTROL */

type Control = Props["control"];

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const shown = ({ value, text, zero, unit }: Control) => {
  const word = said(zero, value);
  return word ?? `${text ?? value}${unit ? ` ${unit}` : ""}`;
};

function Row({ kind, label, children }: { kind: string; label: ReactNode; children: ReactNode }) {
  return (
    <div className="control" {...MRLY} data-kind={kind}>
      <span className="label cell">{label}</span>
      {children}
    </div>
  );
}

function Slider(props: Control) {
  const { label, value, onChange, onCommit, min, max, step = 1, bind } = props;
  return (
    <Row kind="slider" label={label}>
      <input
        className="track cell"
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        aria-label={typeof label === "string" ? label : undefined}
        onChange={(e) => onChange?.(+e.target.value)}
        onPointerUp={onCommit}
        onKeyUp={onCommit}
        {...bind?.()}
      />
      <output className="value cell">{shown(props)}</output>
    </Row>
  );
}

function Number_(props: Control) {
  const { label, value, onChange, onCommit, min = -Infinity, max = Infinity, step = 1, bind } = props;
  const n = Number(value);
  const go = (dir: number) => {
    const next = Math.min(max, Math.max(min, n + dir * step));
    if (next === n) return;
    onChange?.(next);
    onCommit?.();
  };
  return (
    <Row kind="number" label={label}>
      <button type="button" className="step cell" data-limit={n <= min} aria-label="Less" onClick={() => go(-1)}>
        -
      </button>
      <output className="value cell" {...bind?.()}>
        {shown(props)}
      </output>
      <button type="button" className="step cell" data-limit={n >= max} aria-label="More" onClick={() => go(1)}>
        +
      </button>
    </Row>
  );
}

function Toggle({ label, value, onChange, onCommit, bind }: Control) {
  const on = Number(value) === 1;
  return (
    <Row kind="toggle" label={label}>
      <button
        type="button"
        role="switch"
        className="switch cell"
        aria-checked={on}
        onClick={() => {
          onChange?.(on ? 0 : 1);
          onCommit?.();
        }}
        {...bind?.()}
      >
        {on ? "ON" : "OFF"}
      </button>
    </Row>
  );
}

function Pick({ label, value, onChange, options = [], bind }: Control) {
  return (
    <Row kind="pick" label={label}>
      <span className="pick cell">
        <select value={value} onChange={(e) => onChange?.(e.target.value)} {...bind?.()}>
          {options.map(([v, text]) => (
            <option key={v} value={v}>
              {text}
            </option>
          ))}
        </select>
        <span className="drop" aria-hidden="true">
          v
        </span>
      </span>
    </Row>
  );
}

function Segment(props: Control) {
  const { label, value, onChange, options = [], bind } = props;
  const name = useId();
  const words = typeof label === "string" ? label : "";
  const [at, narrow] = useRoom<HTMLDivElement>([words, ...options.map(([v, text]) => `${v}:${text}`)].join("\n"));
  if (narrow)
    return (
      <div ref={at}>
        <Pick
          {...props}
          onChange={(picked) => {
            const found = options.find(([v]) => String(v) === String(picked));
            if (found) onChange?.(found[0]);
          }}
        />
      </div>
    );
  return (
    <div ref={at}>
      <Row kind="segment" label={label}>
        <span className="options" role="radiogroup" aria-label={words} {...bind?.()}>
          {options.map(([v, text]) => (
            <label key={v} className="cell" data-on={String(v) === String(value)}>
              <input
                type="radio"
                name={name}
                value={v}
                checked={String(v) === String(value)}
                onChange={() => onChange?.(v)}
              />
              {text}
            </label>
          ))}
        </span>
      </Row>
    </div>
  );
}

function Text({ label, value, onChange, wide, bind }: Control) {
  return (
    <Row kind="text" label={label}>
      <input
        type="text"
        className={classes("text cell", wide ? "wide" : undefined)}
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        {...bind?.()}
      />
    </Row>
  );
}

function Button({ label, primary, on, tag, bind, children }: Control) {
  return (
    <button
      type="button"
      {...MRLY}
      className={classes("button cell", primary ? "primary" : on ? "on" : undefined, tag)}
      {...bind?.()}
    >
      {children ?? label}
    </button>
  );
}

const KINDS = {
  slider: Slider,
  number: Number_,
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
