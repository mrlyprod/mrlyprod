import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Toast({ text, kind, icon, action }: Props["toast"]) {
  return (
    <div className="chit" data-kind={kind}>
      <span className="mark cell">
        <Icon name={icon} size={14} />
      </span>
      <span className="text cell">{text}</span>
      {action && (
        <button type="button" className="cell" {...action.bind()}>
          {action.label}
        </button>
      )}
    </div>
  );
}
