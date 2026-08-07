import type { ReactNode } from "react";
import { cx } from "./lib";

export function Button({
  children,
  onClick,
  active,
  disabled,
  primary,
  wide,
  className,
}: {
  children: ReactNode;
  onClick?: () => void;
  active?: boolean;
  disabled?: boolean;
  primary?: boolean;
  wide?: boolean;
  className?: string;
}) {
  return (
    <button
      type="button"
      className={cx("button", active && "active", primary && "primary", wide && "wide", className)}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
