import { useSyncExternalStore } from "react";
import { clear, contrast, KEY, motion, targets } from "../../shell/prefs.ts";
import { Pane as Frame, Section, Switch } from "./ui.tsx";
import { usePref } from "./use.ts";

/* TYPES */

type AskProps = { label: string; hint: string; asked: boolean; on: boolean; onChange: (on: boolean) => void };

export const FIND = [
  "Reduce motion",
  "Animation",
  "Reduce transparency",
  "Glass",
  "Increase contrast",
  "Contrast",
  "Larger targets",
];

/* SYSTEM */

const ASK = {
  motion: "(prefers-reduced-motion: reduce)",
  clear: "(prefers-reduced-transparency: reduce)",
  contrast: "(prefers-contrast: more)",
};

const lists = (): [string, MediaQueryList][] =>
  typeof matchMedia === "function" ? Object.entries(ASK).map(([name, query]) => [name, matchMedia(query)]) : [];

const watch = (fn: () => void) => {
  const all = lists();
  for (const [, list] of all) list.addEventListener("change", fn);
  return () => {
    for (const [, list] of all) list.removeEventListener("change", fn);
  };
};

const peek = () =>
  lists()
    .filter(([, list]) => list.matches)
    .map(([name]) => name)
    .join(" ");

const useAsked = () => useSyncExternalStore(watch, peek, () => "").split(" ");

/* SWITCH */

function Ask({ label, hint, asked, on, onChange }: AskProps) {
  return (
    <Switch
      label={label}
      hint={asked ? "On in your system settings" : hint}
      value={on || asked}
      disabled={asked}
      onChange={onChange}
    />
  );
}

/* PANE */

export default function Pane() {
  const asked = useAsked();
  const still = usePref(KEY.motion) === "reduce";
  const solid = usePref(KEY.clear) === "1";
  const more = usePref(KEY.contrast) === "more";
  const large = usePref(KEY.targets) === "large";
  return (
    <Frame title="Accessibility">
      <Section
        title="Display"
        foot="When your system asks for more contrast, less transparency or less motion, the site follows."
      >
        <Ask
          label="Increase contrast"
          hint="Stronger text, lines and edges"
          asked={asked.includes("contrast")}
          on={more}
          onChange={(on) => contrast(on ? "more" : "")}
        />
        <Ask
          label="Reduce transparency"
          hint="Glass turns solid"
          asked={asked.includes("clear")}
          on={solid}
          onChange={clear}
        />
        <Ask
          label="Reduce motion"
          hint="Animations and transitions stop"
          asked={asked.includes("motion")}
          on={still}
          onChange={(on) => motion(on ? "reduce" : "")}
        />
      </Section>
      <Section title="Controls">
        <Switch
          label="Larger targets"
          hint="Every button and control is at least 44 px"
          value={large}
          onChange={(on) => targets(on ? "large" : "")}
        />
      </Section>
    </Frame>
  );
}
