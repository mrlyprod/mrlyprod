import { designs, names } from "site:design";
import { apply } from "./active.ts";

async function start() {
  const load = designs[document.documentElement.dataset.design ?? ""] ?? designs[names[0]];
  apply(await load());
  await import("./chrome.ts");
  await import("./router.ts");
}

start().catch((error) => console.error(error));
