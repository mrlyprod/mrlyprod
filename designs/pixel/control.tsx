import type { Props, Scalar } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties, ReactNode } from "react";
import { useId } from "react";
import { cx } from "./kit.ts";
import { Sprite } from "./sprite.tsx";

type Control = Props["control"];

export const CELLS = 12;

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const clamp = (v: number, min = -Infinity, max = Infinity) => Math.min(max, Math.max(min, v));

const named = (label: ReactNode) => (typeof label === "string" ? label : undefined);

const shown = ({ value, text, zero, unit }: Control) => {
  const word = said(zero, value);
  return word ?? `${text ?? value}${unit ? ` ${unit}` : ""}`;
};

export const chomp = (at: number) => Math.round(clamp(at, 0, 1) * (CELLS - 1));

function Slider(props: Control) {
  const { label, value, onChange, onCommit, min = 0, max = 100, step = 1, bind } = props;
  const at = max > min ? (Number(value) - min) / (max - min) : 0;
  const head = chomp(at);
  return (
    <label className="pixel control slider" data-kind="slider">
      <span className="label">{label}</span>
      <span className="track" style={{ "--at": at } as CSSProperties}>
        <span className="cells" aria-hidden="true">
          {Array.from({ length: CELLS }, (_, i) => (
            <i key={i} className={i < head ? "eaten" : i === head ? "head" : "dot"}>
              {i === head && <Sprite art="pac" size={18} />}
            </i>
          ))}
        </span>
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
      <output className="num">{shown(props)}</output>
    </label>
  );
}

function Spin({ label, value, onChange, onCommit, min, max, step = 1, bind }: Control) {
  const nudge = (by: number) => {
    onChange?.(clamp(Number(value) + by * step, min, max));
    onCommit?.();
  };
  return (
    <label className="pixel control number" data-kind="number">
      <span className="label">{label}</span>
      <span className="spin">
        <button type="button" className="step" tabIndex={-1} aria-label="Less" onClick={() => nudge(-1)}>
          <Sprite art="minus" size={12} />
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
        <button type="button" className="step" tabIndex={-1} aria-label="More" onClick={() => nudge(1)}>
          <Sprite art="add" size={12} />
        </button>
      </span>
    </label>
  );
}

function Text({ label, value, onChange, onCommit, wide, zero, bind }: Control) {
  const word = said(zero, value);
  return (
    <label className={cx("pixel", "control", "text", wide && "wide")} data-kind="text">
      <span className="label">{label}</span>
      <span className="entry">
        <span className="prompt" aria-hidden="true">
          &gt;
        </span>
        <input
          type="text"
          className="field"
          value={value}
          placeholder={word ?? undefined}
          spellCheck={false}
          onChange={(e) => onChange?.(e.target.value)}
          onBlur={onCommit}
          {...bind?.()}
        />
      </span>
    </label>
  );
}

function Toggle({ label, value, onChange, onCommit, bind }: Control) {
  const on = Number(value) === 1;
  return (
    <label className="pixel control toggle" data-kind="toggle">
      <span className="label">{label}</span>
      <span className="switch" data-on={on}>
        <input
          type="checkbox"
          role="switch"
          checked={on}
          aria-checked={on}
          onChange={(e) => {
            onChange?.(e.target.checked ? 1 : 0);
            onCommit?.();
          }}
          {...bind?.()}
        />
        <span className="slot" aria-hidden="true">
          <span className="block" />
        </span>
        <span className="word" aria-hidden="true">
          {on ? "ON" : "OFF"}
        </span>
      </span>
    </label>
  );
}

function Pick({ label, value, onChange, options = [], bind }: Control) {
  const id = `px${useId().replace(/[^\w]/g, "")}`;
  const at = Math.max(
    0,
    options.findIndex(([v]) => String(v) === String(value)),
  );
  const go = (by: number) => {
    if (!options.length) return;
    onChange?.(options[(at + by + options.length) % options.length]![0]);
  };
  const choose = (v: Scalar) => {
    onChange?.(v);
    document.getElementById(id)?.hidePopover?.();
  };
  return (
    <div className="pixel control pick" data-kind="pick">
      <span className="label" id={`${id}-name`}>
        {label}
      </span>
      <span className="chooser" style={{ anchorName: `--${id}` } as CSSProperties}>
        <button type="button" className="step" aria-label="Previous" tabIndex={-1} onClick={() => go(-1)}>
          <Sprite art="left" size={12} />
        </button>
        <button
          type="button"
          className="value"
          popoverTarget={id}
          aria-haspopup="listbox"
          aria-labelledby={`${id}-name ${id}-value`}
          onKeyDown={(e) => {
            if (e.key === "ArrowLeft") go(-1);
            if (e.key === "ArrowRight") go(1);
          }}
          {...bind?.()}
        >
          <span id={`${id}-value`}>{options[at]?.[1] ?? String(value)}</span>
        </button>
        <button type="button" className="step" aria-label="Next" tabIndex={-1} onClick={() => go(1)}>
          <Sprite art="right" size={12} />
        </button>
      </span>
      <div
        id={id}
        popover="auto"
        role="listbox"
        aria-labelledby={`${id}-name`}
        className="pixel list"
        style={{ positionAnchor: `--${id}` } as CSSProperties}
      >
        {options.map(([v, text], i) => (
          <button
            type="button"
            key={String(v)}
            role="option"
            className="item"
            aria-selected={i === at}
            onClick={() => choose(v)}
          >
            <span className="pointer" aria-hidden="true">
              <Sprite art="pac" size={12} />
            </span>
            <span className="label">{text}</span>
          </button>
        ))}
      </div>
    </div>
  );
}

function Segment({ label, value, onChange, options = [], bind }: Control) {
  const name = useId();
  return (
    <div
      className={cx("pixel", "control", "segment", options.length > 4 && "stack")}
      data-kind="segment"
      role="radiogroup"
      aria-label={named(label)}
      {...bind?.()}
    >
      <span className="label">{label}</span>
      <span className="options">
        {options.map(([v, text]) => {
          const on = String(v) === String(value);
          return (
            <label key={String(v)} className={on ? "on" : undefined}>
              <input type="radio" name={name} value={v} checked={on} onChange={() => onChange?.(v)} />
              <span>{text}</span>
            </label>
          );
        })}
      </span>
    </div>
  );
}

function Button({ primary, on, tag, bind, children, label }: Control) {
  return (
    <button
      type="button"
      className={cx("pixel", "control", "button", on && "pressed", primary && "primary", tag)}
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
