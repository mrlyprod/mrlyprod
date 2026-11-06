import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Alert({ title, text, actions, bind }: Props["alert"]) {
  return (
    <dialog className="alert" {...bind.root()}>
      <h2 {...bind.title()}>{title}</h2>
      {text && <p {...bind.text()}>{text}</p>}
      <div className="acts">
        {actions.map((one) => (
          <button
            type="button"
            key={one.label}
            className={classes(one.primary ? "primary" : undefined, one.danger ? "danger" : undefined)}
            {...one.bind()}
          >
            {one.label}
          </button>
        ))}
      </div>
    </dialog>
  );
}
