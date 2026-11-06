import { mount } from "./globe/index.ts";

type Row = { lead: string };

type Site = { title: string };

export const layout = { wide: true, bare: true };

const TOKENS: [string, string][] = [
  ["--bg", "--paper"],
  ["--fg", "--ink"],
  ["--dim", "--muted"],
  ["--line", "--hairline"],
];

const CSS = `
#globe{background:radial-gradient(60% 70% at 70% 55%,color-mix(in srgb,var(--accent,#f80) 16%,transparent),transparent 70%),var(--paper,var(--bg))}
.mh-head{position:absolute;top:clamp(16px,4vh,40px);left:clamp(16px,3vw,40px);right:16px;max-width:400px;display:flex;flex-direction:column;gap:12px;pointer-events:none;z-index:1}
.mh-head h1{margin:0;font-size:clamp(26px,3.2vw,40px);line-height:1.1;color:var(--fg,inherit)}
.mh-head p{margin:0;font-size:14px;line-height:1.55;color:var(--dim,#888)}
`;

const stops = new WeakMap<HTMLElement, () => void>();

function paint(stage: HTMLElement) {
  const style = getComputedStyle(stage);
  for (const [want, from] of TOKENS)
    if (style.getPropertyValue(from).trim()) stage.style.setProperty(want, `var(${from})`);
}

export function render(row: Row, site: Site, host: HTMLElement) {
  const style = document.createElement("style");
  style.textContent = CSS;
  const stage = document.createElement("section");
  stage.id = "globe";
  stage.setAttribute("aria-label", "The dashboard");
  Object.assign(stage.style, {
    position: "relative",
    height: "max(520px, calc(100vh - 260px))",
    overflow: "hidden",
    borderRadius: "5px",
  });
  const head = document.createElement("div");
  head.className = "mh-head";
  const title = document.createElement("h1");
  title.textContent = site.title;
  const lead = document.createElement("p");
  lead.textContent = row.lead;
  head.append(title, lead);
  stage.append(head);
  host.replaceChildren(style, stage);
  paint(stage);
  stops.set(host, mount(stage));
}

export function unmount(host: HTMLElement) {
  stops.get(host)?.();
  stops.delete(host);
  host.replaceChildren();
}
