export type Choice = { id: string; name: string };

export type Picker = { label: (text: string) => void; dispose: () => void };

const CSS = `
.mg-ui{position:absolute;left:max(16px,env(safe-area-inset-left));bottom:max(16px,env(safe-area-inset-bottom));display:flex;flex-direction:column;gap:10px;font:inherit;font-size:13px;line-height:1.3;color:var(--dim,#888);pointer-events:none;z-index:1}
.mg-pick{display:flex;flex-wrap:wrap;gap:6px;pointer-events:auto}
.mg-pick button{all:unset;box-sizing:border-box;cursor:pointer;display:inline-flex;align-items:center;min-height:36px;padding:0 14px;border-radius:999px;border:1px solid var(--line,#8886);color:var(--fg,inherit);background:var(--bg,transparent);transition:background .15s,color .15s,border-color .15s}
.mg-pick button:hover{border-color:var(--dim,#888)}
.mg-pick button[aria-checked="true"]{background:var(--fg,#000);color:var(--bg,#fff);border-color:transparent}
.mg-pick button:focus-visible{outline:2px solid var(--accent,#f80);outline-offset:2px}
.mg-label{margin:0;font-variant-numeric:tabular-nums}
.mg-label b{color:var(--fg,inherit);font-weight:600}
`;

export function picker(el: HTMLElement, choices: Choice[], current: string, onPick: (id: string) => void): Picker {
  const style = document.createElement("style");
  style.textContent = CSS;
  const ui = document.createElement("div");
  ui.className = "mg-ui";
  const group = document.createElement("div");
  group.className = "mg-pick";
  group.setAttribute("role", "radiogroup");
  group.setAttribute("aria-label", "Source");
  const text = document.createElement("p");
  text.className = "mg-label";
  text.setAttribute("aria-live", "polite");
  const buttons = choices.map(({ id, name }, i) => {
    const button = document.createElement("button");
    button.type = "button";
    button.setAttribute("role", "radio");
    button.setAttribute("aria-label", `Watch ${name}`);
    button.dataset.id = id;
    button.textContent = name;
    button.addEventListener("click", () => choose(i, true));
    button.addEventListener("keydown", (e) => {
      const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1, Home: -i, End: choices.length - 1 - i }[
        e.key
      ];
      if (step === undefined) return;
      e.preventDefault();
      choose((i + step + choices.length) % choices.length, true);
      buttons[(i + step + choices.length) % choices.length].focus();
    });
    group.append(button);
    return button;
  });
  function choose(i: number, tell: boolean) {
    buttons.forEach((b, j) => {
      b.setAttribute("aria-checked", String(i === j));
      b.tabIndex = i === j ? 0 : -1;
    });
    if (tell) onPick(choices[i].id);
  }
  choose(
    Math.max(
      0,
      choices.findIndex((c) => c.id === current),
    ),
    false,
  );
  ui.append(group, text);
  el.append(style, ui);
  return {
    label(html) {
      text.innerHTML = html;
    },
    dispose() {
      style.remove();
      ui.remove();
    },
  };
}
