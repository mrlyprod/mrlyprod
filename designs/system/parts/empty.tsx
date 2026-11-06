import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Word } from "../own/word.tsx";

export function Empty({ icon, title, text, action }: Props["empty"]) {
  return (
    <div className="nil">
      {icon && (
        <span className="mark">
          <Icon name={icon} size={42} />
        </span>
      )}
      <h3>
        <Word text={title} />
        <span className="sr">{title}</span>
      </h3>
      {text && <p>{text}</p>}
      {action && (
        <button type="button" className="cell" {...action.bind()}>
          {action.label}
        </button>
      )}
    </div>
  );
}
