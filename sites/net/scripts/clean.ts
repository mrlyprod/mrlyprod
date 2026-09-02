import { rmSync } from "node:fs";
import { join, resolve } from "node:path";

rmSync(join(resolve(import.meta.dir, ".."), "dist"), { recursive: true, force: true });
