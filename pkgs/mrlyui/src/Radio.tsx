import { useRef } from "react";
import type { KeyboardEvent } from "react";
import { cx } from "./lib";

export function Radio<T extends string>({
  options,
  value,
  onChange,
  disabled,
}: {
  options: { label: string; value: T }[];
  value: T;
  onChange: (value: T) => void;
  disabled?: boolean;
}) {
  const refs = useRef<Map<T, HTMLButtonElement>>(new Map());
  const move = (delta: number) => {
    if (options.length === 0) return;
    const index = Math.max(0, options.findIndex((option) => option.value === value));
    const next = options[(index + delta + options.length) % options.length];
    if (!next) return;
    onChange(next.value);
    refs.current.get(next.value)?.focus();
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      move(1);
    }
    if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      move(-1);
    }
  };
  return (
    <div className={cx("radio", disabled && "disabled")} role="radiogroup" onKeyDown={onKeyDown}>
      {options.map((option) => (
        <button
          key={option.value}
          ref={(node) => {
            if (node) {
              refs.current.set(option.value, node);
            } else {
              refs.current.delete(option.value);
            }
          }}
          type="button"
          role="radio"
          aria-checked={option.value === value}
          tabIndex={option.value === value ? 0 : -1}
          className={cx("radio-item", option.value === value && "active")}
          onClick={() => onChange(option.value)}
          disabled={disabled}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}
