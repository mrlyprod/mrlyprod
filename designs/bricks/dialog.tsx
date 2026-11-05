import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BRICKS, paint } from "./brick.ts";

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog
      className={classes("modal plate", tag)}
      {...BRICKS}
      style={paint(light ? "yellow" : "white")}
      data-light={light}
      aria-label={title}
      {...bind()}
    >
      <form method="dialog" className="head plate" style={paint("black")}>
        <h2>{title}</h2>
        <button type="submit" className="close brick" style={paint("red")} aria-label="Close">
          <Icon name="close" size={24} />
        </button>
      </form>
      <div className="body tile">{children}</div>
    </dialog>
  );
}
