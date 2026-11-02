import type { Bot } from "../../sdk/system.ts";
import { below } from "../../sdk/system.ts";
import type { Care } from "./app.ts";

/* POLICY */

export const WEEK = 7 * 24 * 60;

export const bot: Bot<Care> = ({ state, me, actors, at, draw }): ReturnType<Bot> => {
  const open = Object.values(state.clinics).filter((one) => one.closed === null);
  const run = open.find((one) => one.by === me);
  if (!open.length || (!run && open.length * 4 < actors.length && !below(draw, 4)))
    return { verb: "open", args: { name: `Clinic ${me.split("-").pop()}` } };
  const hires = run ? actors.filter((id) => !run.practitioners.includes(id)) : [];
  if (run && run.practitioners.length < 3 && hires.length && !below(draw, 5))
    return { verb: "hire", args: { clinic: run.n, person: hires[below(draw, hires.length)]! } };
  const ahead = Object.values(state.visits).filter(
    (one) => one.cancelled === null && one.start > at && one.patient === me,
  );
  if (ahead.length && !below(draw, 8)) return { verb: "cancel", args: { visit: ahead[0]!.n } };
  const choices = open.flatMap((one) => one.practitioners.filter((id) => id !== me).map((id) => [one.n, id] as const));
  if (ahead.length >= 2 || !choices.length || below(draw, 2)) return null;
  const [clinic, doctor] = choices[below(draw, choices.length)]!;
  return { verb: "book", args: { clinic, with: doctor, in: 30 + below(draw, WEEK) } };
};
