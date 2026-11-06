import type { App, Knob, Schema, Source, ViewNode } from "../kernel/index.ts";

const homeOf = (world: string) => world.split("/").filter(Boolean)[0] ?? "";

const VIEWERS = ["savers", "code", "design", "knobs", "desktop", "shuffle", "privacy", "about"];

const STUB = (() => 1) as unknown as Knob;

const argsOf = (one: App, name: string): Schema => {
  const given = one.verbs[name]?.args;
  return typeof given === "function" ? given(STUB) : (given ?? {});
};

export function viewFaults(one: App, apps: readonly string[]): string[] {
  const out: string[] = [];
  const say = (text: string) => out.push(`${one.id} view: ${text}`);
  const source = (given: Source) => {
    if ("file" in given) {
      if (!given.file.startsWith("/")) say(`reads the file ${given.file}, and a file is a path from the site root`);
    } else if (!apps.includes(homeOf(given.world))) {
      say(`reads ${given.world}, and this world holds no ${homeOf(given.world)}`);
    }
  };
  const name = (given: Source) => ("file" in given ? `${given.file} ${given.path ?? ""}`.trim() : given.world);
  const walk = (node: ViewNode) => {
    if (node.kind === "value") source(node.from);
    if (node.kind === "rows") {
      source(node.from);
      if (!node.fields.length) say(`rows of ${name(node.from)} name no fields`);
      if (new Set(node.fields.map((field) => field.key)).size !== node.fields.length)
        say(`rows of ${name(node.from)} name a field twice`);
      for (const field of node.fields) if (field.names) source(field.names);
    }
    if (node.kind === "form") {
      const short = node.verb.slice(node.verb.indexOf(".") + 1);
      const verb = node.verb.startsWith(`${one.id}.`) ? one.verbs[short] : undefined;
      if (!verb) return void say(`a form names ${node.verb}, and ${one.id} has no such verb`);
      if (verb.inner) say(`a form names ${node.verb}, which only another app may call`);
      const props = argsOf(one, short).properties ?? {};
      for (const [key, given] of Object.entries(node.choices ?? {})) {
        if (!Object.hasOwn(props, key)) say(`${node.verb} offers choices for ${key}, and it takes no such arg`);
        for (const each of [given].flat()) source(each);
      }
    }
    if (node.kind === "chart") source(node.from);
    if (node.kind === "viewer" && !VIEWERS.includes(node.name))
      say(`names the viewer ${node.name}; the shell has ${VIEWERS.join(", ")}`);
    if (node.kind === "pref" && !one.verbs.set) say(`a pref names ${node.key}, and ${one.id} has no set verb`);
    if (node.kind === "group" || node.kind === "pane") node.nodes.forEach(walk);
  };
  (one.view ?? []).forEach(walk);
  return out;
}
