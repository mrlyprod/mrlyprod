import { useEffect, useState } from "react";
import { glass, KEY } from "../../shell/prefs.ts";
import { Nav, Note, Pane as Page, Section, Tiles } from "./ui.tsx";
import { usePref } from "./use.ts";

export const FIND = ["Style", "Clear", "Tinted", "Frosted", "Off", "Liquid glass", "Transparency", "Blur"];

/* SYSTEM */

const MEDIA = "(prefers-reduced-transparency: reduce)";

const asked = () => typeof matchMedia === "function" && matchMedia(MEDIA).matches;

function useSystem() {
  const [on, set] = useState(asked);
  useEffect(() => {
    if (typeof matchMedia !== "function") return undefined;
    const query = matchMedia(MEDIA);
    const sync = () => set(query.matches);
    sync();
    query.addEventListener("change", sync);
    return () => query.removeEventListener("change", sync);
  }, []);
  return on;
}

/* TILES */

const Art = ({ look }: { look: string }) => (
  <span className="glass-art" data-look={look}>
    <i className="glass-chip" />
  </span>
);

const LOOKS = [
  { value: "clear", text: "Clear", art: <Art look="clear" /> },
  { value: "", text: "Tinted", art: <Art look="tinted" /> },
  { value: "frosted", text: "Frosted", art: <Art look="frosted" /> },
  { value: "off", text: "Off", art: <Art look="off" /> },
];

/* PANE */

export default function Pane() {
  const style = usePref(KEY.glass);
  const system = useSystem();
  const solid = usePref(KEY.clear) === "1" || system;
  return (
    <Page title="Glass">
      <Section title="Style">
        <Tiles label="Style" value={solid ? "off" : style} options={LOOKS} onChange={glass} disabled={solid} />
        {solid ? <Note>Reduce transparency is on, so glass is solid.</Note> : null}
        {solid ? <Nav label="Accessibility" hint="Reduce transparency" href="/settings/accessibility/" /> : null}
      </Section>
      <Section>
        <Note>Glass bends and blurs the picture under the bar, dock, widgets, windows and panels.</Note>
      </Section>
    </Page>
  );
}
