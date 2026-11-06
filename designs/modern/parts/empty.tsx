import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Empty({ icon, title, text, action }: Props["empty"]) {
  return (
    <div className="void">
      {icon && (
        <span className="halo">
          <Icon name={icon} size={30} />
        </span>
      )}
      <h3>{title}</h3>
      {text && <p>{text}</p>}
      {action && (
        <button type="button" className="plain" {...action.bind()}>
          {action.label}
        </button>
      )}
    </div>
  );
}
