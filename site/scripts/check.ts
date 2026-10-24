import { resolve } from "node:path";
import { pin } from "../infra/wasm.ts";

const site = resolve(import.meta.dir, "..");
const { lock, local } = pin(site);
if (local === null)
  console.log(`wasm: no site/wasm; bun run wasm builds it, the deploy fetches pkg/${lock.slice(0, 12)}`);
else if (local === lock) console.log(`wasm: site/wasm matches wasm.lock ${lock.slice(0, 12)}`);
else
  console.log(
    `wasm: site/wasm hashes to ${local.slice(0, 12)}, wasm.lock pins ${lock.slice(0, 12)}; the deploy re-pins it`,
  );
const run = Bun.spawnSync([process.execPath, "test", "test/content.test.ts"], {
  cwd: site,
  stdio: ["inherit", "inherit", "inherit"],
});
process.exit(run.exitCode ?? 1);
