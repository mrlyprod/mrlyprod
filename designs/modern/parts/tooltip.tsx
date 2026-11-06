import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Tooltip({ text, bind, children }: Props["tooltip"]) {
  return (
    <span className="tipped" {...bind.root()}>
      {children}
      <span className="tip" {...bind.tip()}>
        {text}
      </span>
    </span>
  );
}
