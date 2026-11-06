import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Toast({ text, kind, icon, action }: Props["toast"]) {
  return (
    <div className="toast" data-kind={kind}>
      <Icon name={icon} size={18} />
      <span>{text}</span>
      {action && (
        <button type="button" {...action.bind()}>
          {action.label}
        </button>
      )}
    </div>
  );
}
