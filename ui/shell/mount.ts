import { frames } from "site:frame";
import { site } from "site:routes";
import { design } from "./active.ts";
import { fits, LOGO, MIN } from "./gate.ts";

const html = () => document.documentElement;

function gate() {
  html().toggleAttribute("data-floor", !fits([innerWidth, innerHeight], site.min ?? MIN));
}

function logo() {
  const one = document.createElement("img");
  one.className = "floor";
  one.src = site.mark;
  one.alt = site.title;
  one.width = LOGO;
  one.height = LOGO;
  return one;
}

function mount() {
  gate();
  addEventListener("resize", gate);
  document.querySelector(".domain")?.remove();
  document.body.insertAdjacentHTML("afterbegin", frames[design.id] ?? "");
  document.body.append(logo());
}

if (typeof document !== "undefined") mount();
