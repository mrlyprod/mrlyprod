import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Tooltip({ text, bind, children }: Props["tooltip"]) {
  return (
    <span className="pin" {...bind.root()}>
      {children}
      <span className="cue" {...bind.tip()}>
        {text}
      </span>
    </span>
  );
}
