import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Alert({ title, text, actions, bind }: Props["alert"]) {
  return (
    <dialog className="halt" {...bind.root()}>
      <h2 className="cell" {...bind.title()}>
        {title}
      </h2>
      {text && (
        <p className="cell" {...bind.text()}>
          {text}
        </p>
      )}
      <div className="acts">
        {actions.map((one) => (
          <button
            type="button"
            key={one.label}
            className={classes("cell", one.primary ? "primary" : undefined, one.danger ? "danger" : undefined)}
            {...one.bind()}
          >
            {one.label}
          </button>
        ))}
      </div>
    </dialog>
  );
}
