import type {
  ChangeEvent,
  ComponentProps,
  CSSProperties,
  HTMLAttributes,
  MouseEvent,
  ReactNode,
  Ref,
  RefObject,
} from "react";
import { createContext, useContext, useEffect, useId, useLayoutEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";
import { ICONS, VIEWBOX } from "../icons.gen.js";
import type { Fixer } from "./export.ts";
import { save } from "./export.ts";
import type { Option, Scalar } from "./spec.ts";

/* TYPES */

export type IconName = keyof typeof ICONS;

export type Action = readonly [string, () => void];

export type MenuRow = readonly [string, () => void, IconName?];

export type Folds = {
  has: (name: string) => boolean;
  flip: (name: string, on: boolean) => void;
  meet: (name: string, order: number) => void;
  of: (name: string) => number;
  at: (after?: string) => number;
};

export type Styled = Omit<CSSProperties, "order"> & { order?: number };

export type IconProps = { name: IconName; size?: number };

export type RowProps = ComponentProps<"div">;

export type GroupProps = Omit<ComponentProps<"section">, "name" | "style"> & {
  name?: string;
  after?: string;
  style?: Styled;
};

export type FoldProps = Omit<ComponentProps<"details">, "open" | "onToggle" | "name" | "style"> & {
  name: string;
  open?: boolean | null;
  onToggle?: (on: boolean) => void;
  live?: ReactNode;
  after?: string;
  style?: Styled;
};

export type PickProps = Omit<ComponentProps<"select">, "value" | "onChange" | "ref"> & {
  label: ReactNode;
  value: Scalar;
  onChange?: (v: string) => void;
  options: readonly Option[];
  ref?: Ref<HTMLLabelElement>;
};

export type SliderProps = Omit<ComponentProps<"input">, "value" | "onChange" | "min" | "max" | "step"> & {
  label: ReactNode;
  value: Scalar;
  onChange: (v: number) => void;
  min?: number;
  max?: number;
  step?: number;
  zero?: string;
  unit?: string;
  show?: (v: Scalar) => Scalar;
  commit?: boolean;
};

export type TextProps = Omit<ComponentProps<"input">, "value" | "onChange"> & {
  label: ReactNode;
  value: Scalar;
  onChange: (v: string) => void;
  wide?: boolean;
  zero?: string;
  commit?: boolean;
};

export type ToggleProps = Omit<ComponentProps<"input">, "value" | "onChange"> & {
  label: ReactNode;
  value: Scalar;
  onChange: (v: 0 | 1) => void;
};

export type SegmentProps<V extends Scalar> = Omit<HTMLAttributes<HTMLElement>, "onChange"> & {
  label: string;
  value: V;
  onChange: (v: V) => void;
  options: readonly (V | [V, Scalar])[];
};

export type BtnProps = ComponentProps<"button"> & { primary?: boolean; on?: boolean };

export type ModalProps = ComponentProps<"dialog"> & { title: string };

export type MenuProps = BtnProps & { rows: readonly MenuRow[] };

export type ExportProps = {
  canvas: HTMLCanvasElement | null | (() => HTMLCanvasElement | null);
  name: string;
  draw?: () => void;
  fix?: Fixer;
  more?: readonly Action[];
  title?: string;
};

type Valued = { value: string };

type Loose = { detail: number; currentTarget: { blur: () => void } };

export const pair = <V extends Scalar>(option: V | [V, Scalar]): [V, Scalar] =>
  Array.isArray(option) ? option : [option, option];

export const number = (input: Valued) => +input.value;

export const string = (input: Valued) => input.value;

export const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

const classes = (...names: (string | undefined)[]) => names.filter(Boolean).join(" ") || undefined;

const loose = (e: Loose) => e.detail > 0 && e.currentTarget.blur();

function useDraft(
  value: Scalar,
  onChange: (v: string) => void,
  commit: boolean | undefined,
  read: (input: Valued) => string,
): [RefObject<HTMLInputElement | null>, Scalar, (e: ChangeEvent<HTMLInputElement>) => void] {
  const [draft, setDraft] = useState<string | null>(null);
  const at = useRef<HTMLInputElement>(null);
  const latest = useRef(onChange);
  latest.current = onChange;
  useEffect(() => {
    const input = at.current;
    if (!commit || !input) return undefined;
    const done = () => {
      setDraft(null);
      latest.current(read(input));
    };
    input.addEventListener("change", done);
    return () => input.removeEventListener("change", done);
  }, [commit, read]);
  const type = (e: ChangeEvent<HTMLInputElement>) => (commit ? setDraft(read(e.target)) : onChange(read(e.target)));
  return [at, draft ?? value, type];
}

export function Icon({ name, size = 20 }: IconProps) {
  return (
    <svg viewBox={VIEWBOX} width={size} height={size} fill="currentColor" aria-hidden="true">
      <path d={ICONS[name]} />
    </svg>
  );
}

export function Row({ children, ...rest }: RowProps) {
  return (
    <div className="row" {...rest}>
      {children}
    </div>
  );
}

export const Accordion = createContext<Folds | null>(null);

const placed = (shared: Folds | null, after: string | undefined, style: Styled | undefined) =>
  shared ? { order: shared.at(after), ...style } : style;

export function Group({ name, hidden, after, style, children, ...rest }: GroupProps) {
  const shared = useContext(Accordion);
  return (
    <section aria-label={name || undefined} hidden={hidden} style={placed(shared, after, style)} {...rest}>
      {name && <h3>{name}</h3>}
      <Row>{children}</Row>
    </section>
  );
}

function Joined({ name, after, style, onToggle, ...rest }: Omit<FoldProps, "open">) {
  const shared = useContext(Accordion);
  const spot = placed(shared, after, style);
  useLayoutEffect(() => {
    shared?.meet(name, spot!.order!);
  });
  if (!shared) return <Fold name={name} open={null} style={style} onToggle={onToggle} {...rest} />;
  const flip = (on: boolean) => {
    shared.flip(name, on);
    onToggle?.(on);
  };
  return <Fold name={name} open={shared.has(name)} style={spot} onToggle={flip} {...rest} />;
}

export function Fold({ name, open, onToggle, live, after, children, ...rest }: FoldProps) {
  if (open === undefined)
    return (
      <Joined name={name} after={after} live={live} onToggle={onToggle} {...rest}>
        {children}
      </Joined>
    );
  return (
    <details className="group" open={open ?? undefined} onToggle={(e) => onToggle?.(e.currentTarget.open)} {...rest}>
      <summary onClick={loose}>
        <h3>{name}</h3>
        <span className="live">{live ?? ""}</span>
        <Icon name="expand_more" />
      </summary>
      <Row>{children}</Row>
    </details>
  );
}

export function Pick({ label, value, onChange, options, ref, ...rest }: PickProps) {
  return (
    <label ref={ref}>
      <span>{label}</span>
      <select value={value} onChange={onChange && ((e) => onChange(e.target.value))} {...rest}>
        {options.map(pair).map(([v, text]) => (
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

export function Slider({ label, value, onChange, min, max, step = 1, zero, unit, show, commit, ...rest }: SliderProps) {
  const word = said(zero, value);
  return (
    <label>
      <span>{label}</span>
      <span className="num">
        {word ?? (show ? show(value) : value)}
        {unit && !word ? ` ${unit}` : ""}
      </span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(number(e.target))}
        {...rest}
      />
    </label>
  );
}

export function Text({ label, value, onChange, wide, zero, commit, ...rest }: TextProps) {
  const [at, now, type] = useDraft(value, onChange, commit, string);
  const word = said(zero, now);
  return (
    <label>
      <span>{label}</span>
      {word && <span className="num">{word}</span>}
      <input ref={at} type="text" className={wide ? "wide" : undefined} value={now} onChange={type} {...rest} />
    </label>
  );
}

export function Toggle({ label, value, onChange, ...rest }: ToggleProps) {
  const on = Number(value) === 1;
  return (
    <label>
      <span>{label}</span>
      <input
        type="checkbox"
        role="switch"
        checked={on}
        aria-checked={on}
        onChange={(e) => onChange(e.target.checked ? 1 : 0)}
        {...rest}
      />
    </label>
  );
}

const natural = (el: HTMLElement) => {
  const was = el.style.width;
  el.style.width = "max-content";
  const need = el.getBoundingClientRect().width;
  el.style.width = was;
  return need;
};

export function useRoom<T extends HTMLElement>(key: string, anywhere = false): [RefObject<T | null>, boolean] {
  const at = useRef<T>(null);
  const need = useRef({ key, width: 0 });
  const [narrow, setNarrow] = useState(false);
  useLayoutEffect(() => {
    const el = at.current;
    if (
      !el ||
      (!anywhere && !(el.parentNode as Partial<Element> | null)?.classList?.contains("row")) ||
      typeof ResizeObserver !== "function"
    )
      return undefined;
    const check = () => {
      if (!narrow) need.current = { key, width: natural(el) };
      setNarrow(need.current.key === key && need.current.width > el.getBoundingClientRect().width);
    };
    check();
    const eye = new ResizeObserver(() => flushSync(check));
    eye.observe(el);
    return () => eye.disconnect();
  }, [narrow, key]);
  return [at, narrow];
}

export function Segment<V extends Scalar>({ label, value, onChange, options, ...rest }: SegmentProps<V>) {
  const name = useId();
  const pairs = options.map((option) => pair(option));
  const [at, narrow] = useRoom<HTMLElement>([label, ...pairs.map(([v, text]) => `${v}:${text}`)].join("\n"));
  const hold = (el: HTMLElement | null) => {
    at.current = el;
  };
  if (narrow)
    return (
      <Pick
        ref={hold}
        label={label}
        value={value}
        options={pairs}
        onChange={(picked) => {
          const found = pairs.find(([v]) => String(v) === picked);
          if (found) onChange(found[0]);
        }}
        {...rest}
      />
    );
  return (
    <div ref={hold} className="segment" role="radiogroup" aria-label={label} {...rest}>
      <span>{label}</span>
      <span className="options">
        {pairs.map(([v, text]) => (
          <label key={v}>
            <input
              type="radio"
              name={name}
              value={v}
              checked={String(v) === String(value)}
              onChange={() => onChange(v)}
            />
            {text}
          </label>
        ))}
      </span>
    </div>
  );
}

export function Btn({ primary, on, className, onClick, children, ...rest }: BtnProps) {
  const look = classes(primary ? "primary" : on ? "on" : "", className);
  const click = (e: MouseEvent<HTMLButtonElement>) => {
    loose(e);
    onClick?.(e);
  };
  return (
    <button type="button" className={look} onClick={click} {...rest}>
      {children}
    </button>
  );
}

export function Modal({ title, className, children, ...rest }: ModalProps) {
  return (
    <dialog className={classes("modal", className)} aria-label={title} {...rest}>
      <form method="dialog">
        <h2>{title}</h2>
        <button type="submit" className="icon" aria-label="Close">
          <Icon name="close" />
        </button>
      </form>
      {children}
    </dialog>
  );
}

export function Menu({ rows, children, ...rest }: MenuProps) {
  const id = `menu${useId().replace(/[^\w-]/g, "")}`;
  const hide = () => document.getElementById(id)?.hidePopover?.();
  return (
    <>
      <Btn popoverTarget={id} style={{ anchorName: `--${id}` }} {...rest}>
        {children}
      </Btn>
      <div id={id} popover="auto" className="pop" style={{ positionAnchor: `--${id}` }}>
        {rows.map(([text, fn, icon]) => (
          <button
            type="button"
            key={text}
            onClick={() => {
              hide();
              fn();
            }}
          >
            {icon && <Icon name={icon} />}
            {text}
          </button>
        ))}
      </div>
    </>
  );
}

const FORMATS: [string, string][] = [
  ["png", "PNG"],
  ["webp", "WebP"],
];

const SCALES: [number, string][] = [
  [1, "1x"],
  [2, "2x"],
  [4, "4x"],
];

export function Export({ canvas, name, draw, fix, more = [], title = "Export" }: ExportProps) {
  const at = useRef<HTMLDialogElement>(null);
  const [kind, setKind] = useState("png");
  const [scale, setScale] = useState(1);
  const close = () => at.current?.close();
  const take = () => {
    close();
    const from = typeof canvas === "function" ? canvas() : canvas;
    if (from) save(from, { name, kind, draw, scale: fix ? scale : 1, fix });
  };
  return (
    <Modal ref={at} className="export" title={title}>
      <div className="rows">
        <Segment label="Format" value={kind} options={FORMATS} onChange={setKind} />
        {fix && <Segment label="Size" value={scale} options={SCALES} onChange={setScale} />}
      </div>
      <Btn primary onClick={take}>
        Save {FORMATS.find(([one]) => one === kind)![1]}
      </Btn>
      {more.length > 0 && (
        <div className="also">
          {more.map(([text, fn]) => (
            <button
              type="button"
              key={text}
              onClick={() => {
                close();
                fn();
              }}
            >
              {text}
            </button>
          ))}
        </div>
      )}
    </Modal>
  );
}
