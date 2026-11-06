import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { drag } from "../own/drag.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Alert({ title, text, actions, bind }: Props["alert"]) {
  return (
    <dialog className="dialog raised message" {...bind.root()}>
      <form className="title" method="dialog" onPointerDown={drag}>
        <span className="name" {...(text ? bind.title() : {})}>
          {text ? title : document.title}
        </span>
        <span className="actions">
          <button type="submit" className="raised knob" aria-label="Close">
            <Glyph art="close" />
          </button>
        </span>
      </form>
      <div className="body">
        <Icon name={actions.some((one) => one.danger) ? "warn" : "info"} size={32} />
        <p {...(text ? bind.text() : bind.title())}>{text || title}</p>
      </div>
      <div className="acts">
        {actions.map((one) => (
          <button type="button" key={one.label} className="raised button" {...one.bind()}>
            {one.label}
          </button>
        ))}
      </div>
    </dialog>
  );
}
