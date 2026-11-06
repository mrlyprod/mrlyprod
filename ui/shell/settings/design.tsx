import type { Site } from "site:routes";
import { site as SITE } from "site:routes";
import { useState } from "react";
import { design } from "../active.ts";
import { held, seal } from "../password.ts";
import { KEY, write } from "../prefs.ts";
import type { Choice } from "./ui.tsx";
import { Section, Segment, Select } from "./ui.tsx";

const MOST = 4;

export const FIND = SITE.designs.length > 1 ? ["Design", ...SITE.designs.map((one) => one.title)] : [];

function restart(next: string, lead: string) {
  write(KEY.design, next === lead ? "" : next);
  write(KEY.welcome, "");
  if (held()) seal();
  window.addEventListener("pageshow", (e) => {
    if (e.persisted) location.reload();
  });
  location.assign("/");
}

export default function Designs({ site }: { site: Site }) {
  const [picked, setPicked] = useState(design.id);
  if (site.designs.length < 2) return null;
  const options = site.designs.map((one): Choice => [one.id, one.title]);
  const Pick = options.length <= MOST ? Segment : Select;
  const pick = (next: string) => {
    if (next === picked) return;
    setPicked(next);
    restart(next, site.designs[0].id);
  };
  return (
    <div data-pref="design" style={{ display: "contents" }}>
      <Section>
        <Pick
          label="Design"
          hint={`${site.title} restarts in the design you pick.`}
          value={picked}
          options={options}
          onChange={pick}
        />
      </Section>
    </div>
  );
}
