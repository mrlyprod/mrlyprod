import type { ReactNode } from "react";
import { useId } from "react";
import { Icon } from "../../lib/app/controls.tsx";
import type { Vars } from "../../shell/parts.tsx";

/* TYPES */

export type Choice = [string, string];

export type Swatch = [string, string, string];

export type Tile = { value: string; text: string; art: ReactNode };

type WordsProps = { label?: ReactNode; hint?: string; id: string; of?: string };

type LineProps = WordsProps & { children?: ReactNode };

type PaneProps = { title: string; lead?: string; children?: ReactNode };

type SectionProps = { title?: string; foot?: string; children?: ReactNode };

type ItemProps = { label: ReactNode; hint?: string; children?: ReactNode };

type NavProps = { label: string; hint?: string; href: string };

type ActionProps = {
  label?: string;
  hint?: string;
  onClick: () => void;
  danger?: boolean;
  disabled?: boolean;
  children?: ReactNode;
};

type SwitchProps = {
  label: string;
  hint?: string;
  value: boolean;
  onChange: (on: boolean) => void;
  disabled?: boolean;
};

type ChoicesProps = {
  label: string;
  hint?: string;
  value: string;
  options: Choice[];
  onChange: (value: string) => void;
  disabled?: boolean;
};

type TilesProps = {
  label: string;
  value: string;
  options: Tile[];
  onChange: (value: string) => void;
  disabled?: boolean;
};

type SwatchesProps = {
  label: string;
  hint?: string;
  value: string;
  options: Swatch[];
  onChange: (value: string) => void;
  disabled?: boolean;
};

/* WORDS */

const told = (...ids: (string | undefined)[]) => ids.filter(Boolean).join(" ") || undefined;

function Words({ label, hint, id, of }: WordsProps) {
  if (!label && !hint) return null;
  const Name = of ? "label" : "span";
  return (
    <span className="words">
      {label && (
        <Name className="name" id={`${id}-name`} htmlFor={of}>
          {label}
        </Name>
      )}
      {hint && (
        <span className="hint" id={`${id}-hint`}>
          {hint}
        </span>
      )}
    </span>
  );
}

function Line({ label, hint, id, of, children }: LineProps) {
  return (
    <div className="item">
      <Words label={label} hint={hint} id={id} of={of} />
      {children}
    </div>
  );
}

/* PANE */

export function Pane({ title, lead, children }: PaneProps) {
  return (
    <div className="form">
      <h1>{title}</h1>
      {lead && <p className="intro">{lead}</p>}
      {children}
    </div>
  );
}

export function Section({ title, foot, children }: SectionProps) {
  const id = useId();
  return (
    <section className="block" aria-labelledby={title ? id : undefined}>
      {title && <h2 id={id}>{title}</h2>}
      <div className="box">{children}</div>
      {foot && <p className="foot">{foot}</p>}
    </section>
  );
}

export const Note = ({ children }: { children?: ReactNode }) => <p className="note">{children}</p>;

/* ROWS */

export function Item({ label, hint, children }: ItemProps) {
  const id = useId();
  return (
    <Line label={label} hint={hint} id={id}>
      {children !== undefined && children !== null && <span className="end">{children}</span>}
    </Line>
  );
}

export function Nav({ label, hint, href }: NavProps) {
  const id = useId();
  return (
    <a className="item nav" href={href}>
      <Words label={label} hint={hint} id={id} />
      <Icon name="chevron_right" size={16} />
    </a>
  );
}

export function Action({ label, hint, onClick, danger, disabled, children }: ActionProps) {
  const id = useId();
  return (
    <Line label={label} hint={hint} id={id}>
      <span className="end">
        <button
          type="button"
          className={danger ? "push danger" : "push"}
          disabled={disabled}
          aria-describedby={told(label && `${id}-name`, hint && `${id}-hint`)}
          onClick={onClick}
        >
          {children}
        </button>
      </span>
    </Line>
  );
}

/* CONTROLS */

export function Switch({ label, hint, value, onChange, disabled }: SwitchProps) {
  const id = useId();
  return (
    <Line label={label} hint={hint} id={id} of={id}>
      <span className="end">
        <input
          id={id}
          type="checkbox"
          role="switch"
          checked={Boolean(value)}
          aria-checked={Boolean(value)}
          disabled={disabled}
          aria-describedby={told(hint && `${id}-hint`)}
          onChange={(e) => onChange(e.target.checked)}
        />
      </span>
    </Line>
  );
}

export function Select({ label, hint, value, options, onChange, disabled }: ChoicesProps) {
  const id = useId();
  return (
    <Line label={label} hint={hint} id={id} of={id}>
      <span className="end menu">
        <select
          id={id}
          value={value}
          disabled={disabled}
          aria-describedby={told(hint && `${id}-hint`)}
          onChange={(e) => onChange(e.target.value)}
        >
          {options.map(([v, text]) => (
            <option key={v} value={v}>
              {text}
            </option>
          ))}
        </select>
        <Icon name="expand_more" size={16} />
      </span>
    </Line>
  );
}

export function Segment({ label, hint, value, options, onChange, disabled }: ChoicesProps) {
  const id = useId();
  return (
    <Line label={label} hint={hint} id={id}>
      <span
        className="end seg"
        role="radiogroup"
        aria-labelledby={told(label && `${id}-name`)}
        aria-describedby={told(hint && `${id}-hint`)}
      >
        {options.map(([v, text]) => (
          <label key={v}>
            <input
              type="radio"
              name={id}
              value={v}
              checked={String(v) === String(value)}
              disabled={disabled}
              onChange={() => onChange(v)}
            />
            <span>{text}</span>
          </label>
        ))}
      </span>
    </Line>
  );
}

export function Tiles({ label, value, options, onChange, disabled }: TilesProps) {
  const id = useId();
  return (
    <fieldset className="choices" disabled={disabled}>
      {label && <legend className="visually-hidden">{label}</legend>}
      {options.map((one) => (
        <label key={one.value} className="choice">
          <input
            type="radio"
            name={id}
            value={one.value}
            checked={one.value === value}
            onChange={() => onChange(one.value)}
          />
          <span className="art">{one.art}</span>
          <span className="text">{one.text}</span>
        </label>
      ))}
    </fieldset>
  );
}

export function Swatches({ label, hint, value, options, onChange, disabled }: SwatchesProps) {
  const id = useId();
  const chosen = options.find(([v]) => v === value);
  return (
    <Line label={label} hint={hint} id={id}>
      <span className="end swatches">
        <span
          className="dots"
          role="radiogroup"
          aria-labelledby={told(label && `${id}-name`)}
          aria-describedby={told(hint && `${id}-hint`)}
        >
          {options.map(([v, text, color]) => (
            <input
              key={v}
              type="radio"
              name={id}
              value={v}
              className={v ? "dot" : "dot auto"}
              style={color ? ({ "--dot": color } as Vars) : undefined}
              title={text}
              aria-label={text}
              checked={v === value}
              disabled={disabled}
              onChange={() => onChange(v)}
            />
          ))}
        </span>
        <span className="chosen" aria-hidden="true">
          {chosen ? chosen[1] : ""}
        </span>
      </span>
    </Line>
  );
}
