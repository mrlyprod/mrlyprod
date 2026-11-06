import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Calendar({ label, text, month, week, days, bind }: Props["calendar"]) {
  return (
    <span className="control datepick">
      <span className="label">{label}</span>
      <button type="button" className="sunken field when" {...bind.field()}>
        <span className={text ? undefined : "unset"}>{text || "(None)"}</span>
        <span className="raised arrow" aria-hidden="true">
          <Glyph art="down" />
        </span>
      </button>
      <span className="monthcal" {...bind.pop()}>
        <span className="head">
          <button type="button" className="raised" {...bind.prev()}>
            <Glyph art="left" />
          </button>
          <b>{month}</b>
          <button type="button" className="raised" {...bind.next()}>
            <Glyph art="right" />
          </button>
        </span>
        <span className="week" aria-hidden="true">
          {week.map((one) => (
            <abbr key={one}>{one}</abbr>
          ))}
        </span>
        <span className="days" {...bind.grid()}>
          {days.map((one) => (
            <button type="button" key={one.iso} {...one.bind()}>
              {one.n}
            </button>
          ))}
        </span>
      </span>
    </span>
  );
}
