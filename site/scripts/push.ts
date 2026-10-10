import { resolve } from "node:path";
import { main } from "../kit/push.ts";
import { build } from "./site.ts";

if (import.meta.main) await main(resolve(import.meta.dir, ".."), () => build());
