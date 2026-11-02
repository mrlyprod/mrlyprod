import type { ComponentProps, CSSProperties, HTMLAttributes, MouseEvent, ReactNode, RefObject } from "react";
import { createContext, useContext, useEffect, useId, useLayoutEffect, useRef, useState } from "react";
import { parts } from "../shell/active.ts";
import type { Fixer } from "./export.ts";
import { save } from "./export.ts";
import type { IconName } from "./icon.tsx";
import { loose } from "./room.ts";
import type { Option, Scalar } from "./spec.ts";

export type { IconName, IconProps } from "./icon.tsx";
export { Icon } from "./icon.tsx";
export { useRoom } from "./room.ts";

/* TYPES */

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

export type ModalProps = ComponentProps<"dialog"> & { title: string; light?: boolean };

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

export const pair = <V extends Scalar>(option: V | [V, Scalar]): [V, Scalar] =>
  Array.isArray(option) ? option : [option, option];

export const number = (input: Valued) => +input.value;

export const string = (input: Valued) => input.value;

export const said = (zero: string | undefined, v: Scalar) => (zero && v !== "" && Number(v) === 0 ? zero : null);

function useDraft(
  value: Scalar,
  onChange: (v: string) => void,
  commit: boolean | undefined,
  read: (input: Valued) => string,
): [RefObject<HTMLInputElement | null>, Scalar, (v: string) => void] {
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
  const type = (v: string) => (commit ? setDraft(v) : onChange(v));
  return [at, draft ?? value, type];
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
  const bind = () => ({ hidden, style: placed(shared, after, style), ...rest });
  return (
    <parts.group name={name} bind={bind}>
      {children}
    </parts.group>
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
    <parts.group name={name} open={open} live={live} onToggle={onToggle} bind={() => rest}>
      {children}
    </parts.group>
  );
}

export function Pick({ label, value, onChange, options, ...rest }: PickProps) {
  return (
    <parts.control
      kind="pick"
      label={label}
      value={value}
      options={options.map(pair)}
      onChange={onChange && ((v) => onChange(String(v)))}
      bind={() => rest}
    />
  );
}

export function Slider({ label, value, onChange, min, max, step = 1, zero, unit, show, commit, ...rest }: SliderProps) {
  return (
    <parts.control
      kind="slider"
      label={label}
      value={value}
      text={show ? show(value) : undefined}
      min={min}
      max={max}
      step={step}
      zero={zero}
      unit={unit}
      onChange={(v) => onChange(Number(v))}
      bind={() => rest}
    />
  );
}

export function Text({ label, value, onChange, wide, zero, commit, ...rest }: TextProps) {
  const [at, now, type] = useDraft(value, onChange, commit, string);
  return (
    <parts.control
      kind="text"
      label={label}
      value={now}
      wide={wide}
      zero={zero}
      onChange={(v) => type(String(v))}
      bind={() => ({ ref: at, ...rest })}
    />
  );
}

export function Toggle({ label, value, onChange, ...rest }: ToggleProps) {
  return (
    <parts.control
      kind="toggle"
      label={label}
      value={value}
      onChange={(v) => onChange(Number(v) === 1 ? 1 : 0)}
      bind={() => rest}
    />
  );
}

export function Segment<V extends Scalar>({ label, value, onChange, options, ...rest }: SegmentProps<V>) {
  const pairs = options.map((option) => pair(option));
  return (
    <parts.control
      kind="segment"
      label={label}
      value={value}
      options={pairs}
      onChange={(v) => onChange(v as V)}
      bind={() => rest}
    />
  );
}

export function Btn({ primary, on, className, onClick, children, ...rest }: BtnProps) {
  const click = (e: MouseEvent<HTMLButtonElement>) => {
    loose(e);
    onClick?.(e);
  };
  return (
    <parts.control
      kind="button"
      label=""
      value=""
      primary={primary}
      on={on}
      tag={className}
      bind={() => ({ onClick: click, ...rest })}
    >
      {children}
    </parts.control>
  );
}

export function Modal({ title, light = false, className, children, ...rest }: ModalProps) {
  const bind = () => (light ? { ...rest, "data-light": "true" } : rest);
  return (
    <parts.dialog title={title} light={light} tag={className} bind={bind}>
      {children}
    </parts.dialog>
  );
}

export function useMenu(rows: readonly MenuRow[]) {
  const id = `menu${useId().replace(/[^\w-]/g, "")}`;
  const hide = () => document.getElementById(id)?.hidePopover?.();
  return {
    id,
    items: rows.map(([label, , icon]) => ({ label, icon })),
    button: () => ({ onClick: loose, popoverTarget: id, style: { anchorName: `--${id}` } }),
    list: () => ({ id, popover: "auto", style: { positionAnchor: `--${id}` } }),
    item: (i: number) => () => ({
      onClick: () => {
        hide();
        rows[i]![1]();
      },
    }),
  };
}

export function Menu({ rows, children, ...rest }: MenuProps) {
  const menu = useMenu(rows);
  return (
    <>
      <Btn popoverTarget={menu.id} style={{ anchorName: `--${menu.id}` }} {...rest}>
        {children}
      </Btn>
      <parts.menu name={menu.id} items={menu.items} mac={false} bind={menu} />
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
