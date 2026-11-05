import type { Site } from "site:routes";
import type { DoNode, Obj, PaneNode, PrefNode, ViewNode } from "@mrlyprod/os/kernel/index.ts";
import type { ComponentType } from "react";
import { useState, useSyncExternalStore } from "react";
import { useCall, useWorld } from "../../kit/world.ts";
import { design } from "../active.ts";
import { keyOf, read } from "../prefs.ts";
import { current } from "../world.ts";
import About, { FIND as about } from "./about.tsx";
import { ART, Knobs, FIND as knobs, swatches } from "./appearance.tsx";
import Desktop, { FIND as desktop } from "./desktop.tsx";
import Privacy, { FIND as privacy, Sure } from "./privacy.tsx";
import Shuffle, { FIND as shuffle } from "./screensaver.tsx";
import type { Choice } from "./ui.tsx";
import { Action, Pane as Page, Section, Segment, Select, Swatches, Switch, Tiles } from "./ui.tsx";

const VIEWERS: Partial<Record<string, ComponentType<{ site: Site }>>> = {
  knobs: Knobs,
  desktop: Desktop,
  shuffle: Shuffle,
  privacy: Privacy,
  about: About,
};

const WORDS: Partial<Record<string, string[]>> = { knobs, desktop, shuffle, privacy, about };

const MOST = 4;

const ASKS: Partial<Record<string, string>> = {
  contrast: "(prefers-contrast: more)",
  clear: "(prefers-reduced-transparency: reduce)",
  motion: "(prefers-reduced-motion: reduce)",
};

const NEEDS: Partial<Record<string, boolean>> = { icons: design.knobs?.icons === true };

const useAsked = (key: string) => {
  const query = ASKS[key];
  const list = query && typeof matchMedia === "function" ? matchMedia(query) : null;
  return useSyncExternalStore(
    (fn) => {
      list?.addEventListener("change", fn);
      return () => list?.removeEventListener("change", fn);
    },
    () => Boolean(list?.matches),
    () => false,
  );
};

export function labelsOf(node: ViewNode): string[] {
  if (node.kind === "viewer") return WORDS[node.name] ?? [];
  if (node.kind === "pref") return [node.title, ...(node.options ?? []).map(([, label]) => label)];
  if (node.kind === "do") return [node.title];
  if (node.kind === "group" || node.kind === "pane") return [node.title ?? "", ...node.nodes.flatMap(labelsOf)];
  return [];
}

function Pref({ node, site }: { node: PrefNode; site: Site }) {
  const held = useWorld("settings/@me") as Obj | null;
  const call = useCall();
  const asked = useAsked(node.key);
  const value = held ? String(held[node.key] ?? "") : read(keyOf(node.key));
  const set = (next: string) => void call("settings.set", { key: node.key, value: next });
  const options: Choice[] = node.options ?? [];
  const drawn = () => {
    if (node.as === "color")
      return <Swatches label={node.title} value={value} options={swatches(site)} onChange={set} />;
    if (node.as === "toggle") {
      const on = node.on ?? "1";
      return (
        <Switch
          label={node.title}
          hint={asked ? "On in your system settings" : undefined}
          value={value === on || asked}
          disabled={asked}
          onChange={(next) => set(next ? on : (node.off ?? ""))}
        />
      );
    }
    if (node.art)
      return (
        <Tiles
          label={node.title}
          value={value}
          options={options.map(([one, text]) => ({ value: one, text, art: ART[node.art!]!(one) }))}
          onChange={set}
        />
      );
    if (options.length <= MOST) return <Segment label={node.title} value={value} options={options} onChange={set} />;
    return <Select label={node.title} value={value} options={options} onChange={set} />;
  };
  return (
    <div data-pref={node.key} style={{ display: "contents" }}>
      {drawn()}
    </div>
  );
}

const ACTS: Partial<Record<string, () => void>> = { wipe: () => current()?.wipe() };

function Do({ node }: { node: DoNode }) {
  const [sure, setSure] = useState(false);
  const go = () => {
    ACTS[node.action]?.();
    setSure(false);
  };
  return (
    <div data-do={node.action} style={{ display: "contents" }}>
      {sure ? (
        <Sure label={`${node.title}?`} hint="" yes={node.title} hot go={go} no={() => setSure(false)} />
      ) : (
        <Action label={node.title} danger={node.sure} onClick={() => (node.sure ? setSure(true) : go())}>
          {node.title}
        </Action>
      )}
    </div>
  );
}

function Part({ node, site }: { node: ViewNode; site: Site }) {
  if (node.kind === "pref") return <Pref node={node} site={site} />;
  if (node.kind === "do") return <Do node={node} />;
  if (node.kind === "viewer") {
    const Viewer = VIEWERS[node.name];
    return Viewer ? <Viewer site={site} /> : null;
  }
  if (node.kind !== "group" || (node.needs && !NEEDS[node.needs])) return null;
  return (
    <Section title={node.title}>
      {node.nodes.map((one, i) => (
        <Part key={i} node={one} site={site} />
      ))}
    </Section>
  );
}

export function Body({ node, site }: { node: PaneNode; site: Site }) {
  return (
    <Page title={node.title}>
      {node.nodes.map((one, i) => (
        <Part key={i} node={one} site={site} />
      ))}
    </Page>
  );
}
