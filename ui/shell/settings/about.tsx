import type { Site } from "site:routes";
import { useEffect, useState } from "react";
import { current, start } from "../world.ts";
import { Item, Nav, Section } from "./ui.tsx";

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
  "world",
  "seed",
  "rules",
];

export default function Pane({ site }: { site: Site }) {
  const [store, setStore] = useState(current);
  useEffect(() => {
    let on = true;
    start().then((one) => on && setStore(one));
    return () => {
      on = false;
    };
  }, []);
  const world = store?.world() ?? null;
  const version = site.version ?? "";
  const units = site.units ?? [];
  const licences = site.licences ?? [];
  const socials = site.socials ?? [];
  const slug = site.git?.slug ?? "";
  const root = site.tree?.find((one) => one.name === "Root")?.nodes ?? [];
  const year = new Date().getFullYear();
  const since = site.since ?? year;
  return (
    <>
      <div className="about-head">
        <img className="about-icon" src={site.mark} alt="" />
        <strong className="about-name">{site.title}</strong>
        {version && <span className="dim">Version {version}</span>}
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
      {world && (
        <Section title="World" foot="A fresh seed on every load; a rule is pinned or drawn from the seed at boot.">
          <Item label="Seed">
            <span className="num">{world.seed}</span>
          </Item>
          {Object.entries(world.rules).map(([rule, value]) => (
            <Item key={rule} label={rule}>
              <span className="num">{JSON.stringify(value)}</span>
            </Item>
          ))}
        </Section>
      )}
      {licences.length > 0 && (
        <Section title="Licences">
          {licences.map((one) => (
            <Nav key={one.href} label={one.name} href={one.href} />
          ))}
        </Section>
      )}
      <Section title={`Follow ${site.title}`}>
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
    </>
  );
}
