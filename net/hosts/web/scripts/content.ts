import { join } from "node:path";
import { pin } from "../infra/wasm.ts";
import { HOME, REPO } from "./home.ts";

const { lock, local } = pin(HOME);
if (lock === null) console.log("wasm: the site pins no wasm");
else if (local === null)
  console.log(`wasm: no wasm in the site config; bun run wasm builds it, the deploy fetches pkg/${lock.slice(0, 12)}`);
else if (local === lock) console.log(`wasm: the site wasm matches wasm.lock ${lock.slice(0, 12)}`);
else
  console.log(
    `wasm: the site wasm hashes to ${local.slice(0, 12)}, wasm.lock pins ${lock.slice(0, 12)}; the deploy re-pins it`,
  );
const run = Bun.spawnSync([process.execPath, "test", join(import.meta.dir, "..", "test", "content.test.ts")], {
  cwd: REPO,
  stdio: ["inherit", "inherit", "inherit"],
});
process.exit(run.exitCode ?? 1);
