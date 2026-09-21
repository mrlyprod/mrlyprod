import { inlineScripts } from "../ui/config.js";
import SITE from "../lib/site.js";

for (const text of inlineScripts(SITE.prefix)) console.log(JSON.stringify(text));
