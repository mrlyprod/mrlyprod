import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Calendar({ label, text, month, week, days, bind }: Props["calendar"]) {
  return (
    <span className="cal">
      <span className="cell">{label}</span>
      <button type="button" className="cell" {...bind.field()}>
        {text || "choose"}
      </button>
      <span className="calpop" {...bind.pop()}>
        <span className="head">
          <button type="button" className="cell" {...bind.prev()}>
            <Icon name="left" size={14} />
          </button>
          <b className="cell">{month}</b>
          <button type="button" className="cell" {...bind.next()}>
            <Icon name="right" size={14} />
          </button>
        </span>
        <span className="week" aria-hidden="true">
          {week.map((one) => (
            <abbr key={one}>{one}</abbr>
          ))}
        </span>
        <span className="days" {...bind.grid()}>
          {days.map((one) => (
            <button
              type="button"
              key={one.iso}
              className={classes("cell", one.out ? "out" : undefined)}
              {...one.bind()}
            >
              {one.n}
            </button>
          ))}
        </span>
      </span>
    </span>
  );
}
