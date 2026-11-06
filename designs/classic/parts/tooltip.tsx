import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Tooltip({ text, bind, children }: Props["tooltip"]) {
  return (
    <span className="helped" {...bind.root()}>
      {children}
      <span className="help" {...bind.tip()}>
        {text}
      </span>
    </span>
  );
}
