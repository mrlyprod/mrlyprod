import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { cx } from "./kit.ts";
import { Sprite } from "./sprite.tsx";

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={cx("pixel", "dialog", light ? "light" : "frame", tag)} aria-label={title} {...bind()}>
      {!light && (
        <form className="title" method="dialog">
          <span className="name">{title}</span>
          <button type="submit" className="knob close" aria-label="Close">
            <Sprite art="close" size={12} />
          </button>
        </form>
      )}
      <div className="body">{children}</div>
    </dialog>
  );
}
