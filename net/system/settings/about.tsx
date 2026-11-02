import type { Site } from "site:routes";
import type { Vars } from "../../shell/parts.tsx";
import { Pane as Frame, Item, Nav, Section } from "./ui.tsx";

export const FIND = [
  "about",
  "version",
  "wasm",
  "licences",
  "licenses",
  "source",
  "follow",
  "social",
  "robots",
  "sitemap",
  "llms",
  "copyright",
];

/* MARK */

const MARK = "##### #.#.# #.#.# #.#.# #.#.#".split(" ");

function Mark() {
  return (
    <span className="about-icon" aria-hidden="true">
      <svg
        className="glyphs"
        viewBox={`0 0 ${MARK[0].length} ${MARK.length}`}
        style={{ "--rows": MARK.length, "--cols": MARK[0].length } as Vars}
        aria-hidden="true"
      >
        {MARK.flatMap((line, y) => [...line].flatMap((c, x) => (c === "#" ? [[x, y]] : []))).map(([x, y]) => (
          <rect key={`${x}:${y}`} x={x} y={y} width="1" height="1" />
        ))}
      </svg>
    </span>
  );
}

/* PANE */

export default function Pane({ site }: { site: Site }) {
  const version = site.version ?? "";
  const units = site.units ?? [];
  const licences = site.licences ?? [];
  const socials = site.socials ?? [];
  const slug = site.git?.slug ?? "";
  const root = site.tree?.find((one) => one.name === "Root")?.nodes ?? [];
  const year = new Date().getFullYear();
  const since = site.since ?? year;
  return (
    <Frame title="About">
      <div className="about-head">
        <Mark />
        <strong className="about-name">{site.title}</strong>
        {version && <span className="dim">Version {version}</span>}
        {site.tagline && <span className="about-tag dim">{site.tagline}</span>}
      </div>
      <Section title="Software">
        {version && (
          <Item label="Version">
            <span className="num">{version}</span>
          </Item>
        )}
        {units.length > 0 && (
          <Item label="Wasm units">
            <span className="about-units">{units.join(", ")}</span>
          </Item>
        )}
        <Nav label="Source" href="/git/" />
        {slug && <Nav label="GitHub" hint={slug} href={`https://github.com/${slug}`} />}
      </Section>
      {licences.length > 0 && (
        <Section title="Licences">
          {licences.map((one) => (
            <Nav key={one.href} label={one.name} href={one.href} />
          ))}
        </Section>
      )}
      <Section title="Follow Mrly">
        {socials.map((one) => (
          <Nav key={one.href} label={one.name} href={one.href} />
        ))}
        {site.contact && <Nav label={site.contact} href={`mailto:${site.contact}`} />}
      </Section>
      {root.length > 0 && (
        <Section title="Root">
          {root.map((one) => (
            <Nav key={one.href} label={one.name} href={one.href} />
          ))}
        </Section>
      )}
      <p className="fine about-fine">
        Copyright © {site.company || site.title} {since < year ? `${since}-${year}` : year}. All rights reserved.
      </p>
    </Frame>
  );
}
