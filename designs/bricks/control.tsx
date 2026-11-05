import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { classes, useRoom } from "@mrlyprod/ui/kit/room.ts";
import type { Props, Scalar } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties, ReactNode } from "react";
import { useId } from "react";
import { BRICKS, paint } from "./brick.ts";

type Control = Props["control"];

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const shown = ({ value, text, zero, unit }: Control) =>
  said(zero, value) ?? `${text ?? value}${unit ? ` ${unit}` : ""}`;

const named = (label: ReactNode) => (typeof label === "string" ? label : undefined);

function Row({ kind, label, children }: { kind: string; label: ReactNode; children: ReactNode }) {
  return (
    <div className="control" {...BRICKS} data-kind={kind}>
      <span className="label tile">{label}</span>
      {children}
    </div>
  );
}

function Slider(props: Control) {
  const { label, value, onChange, onCommit, min = 0, max = 100, step = 1, bind } = props;
  const at = max > min ? (Number(value) - min) / (max - min) : 0;
  return (
    <Row kind="slider" label={label}>
      <span className="track plate" style={{ ...paint("black"), "--at": at } as CSSProperties}>
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          aria-label={named(label)}
          onChange={(e) => onChange?.(+e.target.value)}
          onPointerUp={onCommit}
          onKeyUp={onCommit}
          {...bind?.()}
        />
      </span>
      <output className="value tile">{shown(props)}</output>
    </Row>
  );
}

function Count(props: Control) {
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
      <button
        type="button"
        className="step brick"
        style={paint("red")}
        data-limit={n <= min}
        aria-label="Less"
        onClick={() => go(-1)}
      >
        -
      </button>
      <output className="value tile" {...bind?.()}>
        {shown(props)}
      </output>
      <button
        type="button"
        className="step brick"
        style={paint("green")}
        data-limit={n >= max}
        aria-label="More"
        onClick={() => go(1)}
      >
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
        className="slot plate"
        style={paint("black")}
        aria-checked={on}
        aria-label={named(label)}
        onClick={() => {
          onChange?.(on ? 0 : 1);
          onCommit?.();
        }}
        {...bind?.()}
      >
        <span className="knob brick" style={paint(on ? "green" : "white")} />
        <span className="word">{on ? "On" : "Off"}</span>
      </button>
    </Row>
  );
}

function Pick({ label, value, onChange, options = [], bind }: Control) {
  return (
    <Row kind="pick" label={label}>
      <span className="pick tile">
        <select value={value} aria-label={named(label)} onChange={(e) => onChange?.(e.target.value)} {...bind?.()}>
          {options.map(([v, text]) => (
            <option key={v} value={v}>
              {text}
            </option>
          ))}
        </select>
        <span className="drop brick" style={paint("white")} aria-hidden="true">
          <Icon name="expand_more" size={24} />
        </span>
      </span>
    </Row>
  );
}

function Segment(props: Control) {
  const { label, value, onChange, options = [], bind } = props;
  const name = useId();
  const words = named(label) ?? "";
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
          {options.map(([v, text]) => {
            const on = String(v) === String(value);
            return (
              <label key={v} className="brick" style={paint(on ? "blue" : "white")} data-on={on}>
                <input type="radio" name={name} value={v} checked={on} onChange={() => onChange?.(v)} />
                {text}
              </label>
            );
          })}
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
        className={classes("text tile", wide ? "wide" : undefined)}
        value={value}
        aria-label={named(label)}
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
      {...BRICKS}
      className={classes("button brick", primary ? "primary" : undefined, tag)}
      style={paint(primary ? "red" : on ? "blue" : "white")}
      aria-pressed={on}
      {...bind?.()}
    >
      {children ?? label}
    </button>
  );
}

const KINDS = {
  slider: Slider,
  number: Count,
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
