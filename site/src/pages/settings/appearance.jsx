import { useSyncExternalStore } from "react";
import { HUES, tints } from "../../lib/theme/hues.js";
import { AppIcon } from "../../shell/appicon.jsx";
import { face, hours, icons, KEY, look, read, root, text, theme, tint } from "../../shell/prefs.js";
import { Note, Pane as Page, Section, Segment, Select, Swatches, Tiles } from "./ui.jsx";
import { usePref } from "./use.js";

export const FIND = [
  "Appearance",
  "Auto",
  "Light",
  "Dark",
  "Theme",
  "Dark mode",
  "Light mode",
  "Accent colour",
  "Accent color",
  "Tint",
  "Colour",
  "Color",
  "Font",
  "Typeface",
  "Text size",
  "Icon & widget style",
  "Icons",
  "Clear",
  "Tinted",
  "Icons go dark",
  "Clock",
  "Time format",
  "12-hour",
  "24-hour",
];

/* DARK */

const DARK = "(prefers-color-scheme: dark)";

const media = () => (typeof matchMedia === "function" ? matchMedia(DARK) : null);

function watch(fn) {
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

/* APPEARANCE */

const Desk = ({ tone }) => (
  <span className="theme-art" data-tone={tone}>
    <span className="theme-win">
      <i />
    </span>
  </span>
);

const MODES = [
  {
    value: "",
    text: "Auto",
    art: (
      <span className="theme-split">
        <Desk tone="light" />
        <Desk tone="dark" />
      </span>
    ),
  },
  { value: "light", text: "Light", art: <Desk tone="light" /> },
  { value: "dark", text: "Dark", art: <Desk tone="dark" /> },
];

/* ACCENT */

const swatches = (site) => {
  const auto = HUES.includes(site?.tint) ? `var(--${site.tint})` : "var(--blue)";
  return tints("Auto").map(([value, name]) => [value, name, value ? `var(--${value})` : auto]);
};

/* TEXT */

const FACES = [
  ["", "Default"],
  ["sans", "Noto Sans"],
  ["serif", "Noto Serif"],
  ["mono", "Noto Sans Mono"],
  ["mrly", "MrlyFont"],
];

const SIZES = [
  ["small", "Small"],
  ["", "Default"],
  ["large", "Large"],
  ["larger", "Larger"],
];

/* ICONS */

const SHOWN = ["polygons", "spirograph", "three"];

const STYLES = [
  ["", "Default"],
  ["dark", "Dark"],
  ["clear", "Clear"],
  ["tinted", "Tinted"],
];

const WHEN = [
  ["", "Auto"],
  ["always", "Always"],
];

const Icons = ({ value }) => (
  <span className="icon-art" data-icons={value}>
    {SHOWN.map((id) => (
      <AppIcon key={id} id={id} />
    ))}
  </span>
);

const styles = (always, night) =>
  STYLES.map(([value, name]) => ({ value, text: name, art: <Icons value={look(value, always, night)} /> }));

/* CLOCK */

const CYCLES = [
  ["", "Auto"],
  ["12", "12-hour"],
  ["24", "24-hour"],
];

/* PANE */

export default function Pane({ site }) {
  const mode = usePref(KEY.theme);
  const hue = usePref(KEY.tint);
  const font = usePref(KEY.font);
  const size = usePref(KEY.text);
  const style = usePref(KEY.icons);
  const always = usePref(KEY.always) === "1";
  const cycle = usePref(KEY.hours);
  const night = useDark();
  return (
    <Page title="Appearance">
      <Section>
        <Tiles label="Appearance" value={mode} options={MODES} onChange={theme} />
        <Swatches label="Accent colour" value={hue} options={swatches(site)} onChange={tint} />
      </Section>
      <Section>
        <Select label="Font" value={font} options={FACES} onChange={face} />
        <Segment label="Text size" value={size} options={SIZES} onChange={text} />
        <Note>
          <span className="face-sample">The quick brown fox jumps over the lazy dog.</span>
        </Note>
      </Section>
      <Section
        title="Icon & widget style"
        foot="Auto turns Dark, Clear and Tinted icons dark in dark mode. Always keeps them dark."
      >
        <Tiles
          label="Icon & widget style"
          value={style}
          options={styles(always, night)}
          onChange={(next) => icons(next, always)}
        />
        <Segment
          label="Icons go dark"
          value={always ? "always" : ""}
          options={WHEN}
          onChange={(next) => icons(read(KEY.icons), next === "always")}
          disabled={!style}
        />
      </Section>
      <Section title="Clock" foot="Auto follows your browser's language and region.">
        <Segment label="Time format" value={cycle} options={CYCLES} onChange={hours} />
      </Section>
    </Page>
  );
}
