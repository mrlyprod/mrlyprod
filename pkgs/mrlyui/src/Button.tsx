import type { ReactNode } from "react";
import { cx } from "./lib";

export function Button({
  children,
  href,
  onClick,
  active,
  disabled,
  primary,
  wide,
  className,
}: {
  children: ReactNode;
  href?: string;
  onClick?: () => void;
  active?: boolean;
  disabled?: boolean;
  primary?: boolean;
  wide?: boolean;
  className?: string;
}) {
  const skin = cx("button", active && "active", primary && "primary", wide && "wide", className);
  if (href !== undefined) {
    return (
      <a
        href={href}
        className={cx(skin, disabled && "disabled")}
        aria-disabled={disabled}
        onClick={(event) => {
          if (disabled) {
            event.preventDefault();
            return;
          }
          onClick?.();
        }}
      >
        {children}
      </a>
    );
  }
  return (
    <button
      type="button"
      className={skin}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
