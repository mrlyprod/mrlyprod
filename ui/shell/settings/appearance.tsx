import type { Site } from "site:routes";
import type { ReactNode } from "react";
import { useSyncExternalStore } from "react";
import { HUES, tints } from "../../kit/theme/hues.ts";
import { design, parts } from "../active.ts";
import { AppIcon } from "../appicon.tsx";
import type { Knob } from "../design.ts";
import { dial, KEY, knob, look, root } from "../prefs.ts";
import type { Swatch } from "./ui.tsx";
import { Section, Select } from "./ui.tsx";
import { usePref } from "./use.ts";

const OWN = design.knobs?.own ?? [];

export const FIND = OWN.flatMap((one) => [one.name, ...one.options.map(([, label]) => label)]);

const DARK = "(prefers-color-scheme: dark)";

const media = () => (typeof matchMedia === "function" ? matchMedia(DARK) : null);

function watch(fn: () => void) {
  const query = media();
  const seen = new MutationObserver(fn);
  seen.observe(root(), { attributes: true, attributeFilter: ["data-theme"] });
  query?.addEventListener("change", fn);
  return () => {
    seen.disconnect();
    query?.removeEventListener("change", fn);
  };
}

const dark = () => {
  const set = root().dataset.theme;
  return set ? set === "dark" : Boolean(media()?.matches);
};

const useDark = () => useSyncExternalStore(watch, dark, () => false);

const Still = parts.preview;

const Desk = ({ tone }: { tone: "light" | "dark" }) => (
  <span className="theme-art" data-tone={tone}>
    {Still ? (
      <Still title={design.title} theme={tone} />
    ) : (
      <span className="theme-win">
        <i />
      </span>
    )}
  </span>
);

export const swatches = (site: Site): Swatch[] => {
  const auto = HUES.includes(site?.tint) ? `var(--${site.tint})` : "var(--blue)";
  return tints("Auto").map(([value, name]) => [value, name, value ? `var(--${value})` : auto]);
};

const SHOWN = ["settings", "git", "screensavers"];

const Icons = ({ value }: { value: string }) => (
  <span className="icon-art" data-icons={value}>
    {SHOWN.map((id) => (
      <AppIcon key={id} id={id} />
    ))}
  </span>
);

function Dial({ one }: { one: Knob }) {
  const value = usePref(dial(one.id));
  return (
    <Select
      label={one.name}
      hint={one.hint}
      value={value}
      options={one.options}
      onChange={(next) => knob(one.id, next)}
    />
  );
}

function IconArt({ value }: { value: string }) {
  const always = usePref(KEY.always) === "1";
  const night = useDark();
  return <Icons value={look(value, always, night)} />;
}

export const ART: Partial<Record<string, (value: string) => ReactNode>> = {
  theme: (value) =>
    value ? (
      <Desk tone={value as "light" | "dark"} />
    ) : (
      <span className="theme-split">
        <Desk tone="light" />
        <Desk tone="dark" />
      </span>
    ),
  icons: (value) => <IconArt value={value} />,
};

export function Knobs() {
  if (!OWN.length) return null;
  return (
    <Section title={design.title}>
      {OWN.map((one) => (
        <Dial key={one.id} one={one} />
      ))}
    </Section>
  );
}
