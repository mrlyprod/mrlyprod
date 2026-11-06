import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Empty({ icon, title, text, action }: Props["empty"]) {
  return (
    <div className="blank">
      {icon && <Icon name={icon} size={32} />}
      <h3>{title}</h3>
      {text && <p>{text}</p>}
      {action && (
        <button type="button" className="raised button" {...action.bind()}>
          {action.label}
        </button>
      )}
    </div>
  );
}
