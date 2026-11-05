import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { classes, useRoom } from "@mrlyprod/ui/kit/room.ts";
import type { Props, Scalar } from "@mrlyprod/ui/shell/design.ts";
import type { Ref } from "react";
import { useId } from "react";

type Control = Props["control"];

const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const inRow = (el: HTMLElement) => Boolean((el.parentNode as Partial<Element> | null)?.classList?.contains("row"));

function Pick({ label, value, onChange, options = [], bind, hold }: Control & { hold?: Ref<HTMLLabelElement> }) {
  return (
    <label ref={hold}>
      <span>{label}</span>
      <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...bind?.()}>
        {options.map(([v, text]) => (
          <option key={v} value={v}>
            {text}
          </option>
        ))}
      </select>
      <span className="drop">
        <Icon name="expand_more" />
      </span>
    </label>
  );
}

function Slider({ label, value, text, onChange, min, max, step = 1, zero, unit, bind }: Control) {
  const word = said(zero, value);
  return (
    <label>
      <span>{label}</span>
      <span className="num">
        {word ?? text ?? value}
        {unit && !word ? ` ${unit}` : ""}
      </span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange?.(+e.target.value)}
        {...bind?.()}
      />
    </label>
  );
}

function Text({ label, value, onChange, wide, zero, bind }: Control) {
  const word = said(zero, value);
  return (
    <label>
      <span>{label}</span>
      {word && <span className="num">{word}</span>}
      <input
        type="text"
        className={wide ? "wide" : undefined}
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        {...bind?.()}
      />
    </label>
  );
}

function Toggle({ label, value, onChange, bind }: Control) {
  const on = Number(value) === 1;
  return (
    <label>
      <span>{label}</span>
      <input
        type="checkbox"
        role="switch"
        checked={on}
        aria-checked={on}
        onChange={(e) => onChange?.(e.target.checked ? 1 : 0)}
        {...bind?.()}
      />
    </label>
  );
}

function Segment(props: Control) {
  const { label, value, onChange, options = [], bind } = props;
  const name = useId();
  const words = typeof label === "string" ? label : "";
  const [at, narrow] = useRoom<HTMLElement>([words, ...options.map(([v, text]) => `${v}:${text}`)].join("\n"), inRow);
  const hold = (el: HTMLElement | null) => {
    at.current = el;
  };
  if (narrow)
    return (
      <Pick
        {...props}
        hold={hold}
        onChange={(picked) => {
          const found = options.find(([v]) => String(v) === picked);
          if (found) onChange?.(found[0]);
        }}
      />
    );
  return (
    <div ref={hold} className="segment" role="radiogroup" aria-label={words} {...bind?.()}>
      <span>{label}</span>
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
            {text}
          </label>
        ))}
      </span>
    </div>
  );
}

function Button({ primary, on, tag, bind, children }: Control) {
  return (
    <button type="button" className={classes(primary ? "primary" : on ? "on" : "", tag)} {...bind?.()}>
      {children}
    </button>
  );
}

const KINDS = {
  slider: Slider,
  number: Slider,
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
