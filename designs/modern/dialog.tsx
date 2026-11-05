import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Dialog({ title, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("modal", tag)} aria-label={title} {...bind()}>
      <form method="dialog">
        <h2>{title}</h2>
        <button type="submit" className="icon" aria-label="Close">
          <Icon name="close" />
        </button>
      </form>
      {children}
    </dialog>
  );
}
