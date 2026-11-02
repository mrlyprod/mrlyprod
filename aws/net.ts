import { lambda } from "../net/hosts/web/infra/builder.ts";

/* WHERE */

const net = lambda({
  source: "mrlyprod/mrlyprod",
  head: "build/net/head",
  dir: "/tmp/src",
  agent: "mrlynet-site-builder",
  bucket: "MRLYDEV_BUCKET",
  folder: ".",
  sources: ["push", "schedule", "hook", "manual"],
  prepare: async (site, run) => {
    const out = await run([process.execPath, "net/hosts/web/infra/wasm.ts"], site);
    return `wasm ${(out.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`;
  },
});

export default { fetch: net.fetch };
