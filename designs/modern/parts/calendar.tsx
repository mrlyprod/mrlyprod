import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Calendar({ label, text, month, week, days, bind }: Props["calendar"]) {
  return (
    <span className="datefield">
      <span>{label}</span>
      <button type="button" className={text ? undefined : "unset"} {...bind.field()}>
        {text || "Choose"}
      </button>
      <span className="month" {...bind.pop()}>
        <span className="head">
          <b>{month}</b>
          <button type="button" className="icon" {...bind.prev()}>
            <Icon name="left" size={18} />
          </button>
          <button type="button" className="icon" {...bind.next()}>
            <Icon name="right" size={18} />
          </button>
        </span>
        <span className="week" aria-hidden="true">
          {week.map((one) => (
            <abbr key={one}>{one}</abbr>
          ))}
        </span>
        <span className="days" {...bind.grid()}>
          {days.map((one) => (
            <button type="button" key={one.iso} className={one.out ? "out" : undefined} {...one.bind()}>
              {one.n}
            </button>
          ))}
        </span>
      </span>
    </span>
  );
}
