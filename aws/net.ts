import { lambda } from "../site/infra/builder.ts";

/* WHERE */

const net = lambda({
  source: "mrlyprod/mrlyprod",
  head: "build/net/head",
  dir: "/tmp/src",
  agent: "mrlynet-site-builder",
  bucket: "MRLYDEV_BUCKET",
  folder: "site",
  sources: ["push", "schedule", "hook", "manual"],
  prepare: async (site, run) => {
    const out = await run([process.execPath, "infra/wasm.ts"], site);
    return `wasm ${(out.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`;
  },
});

export default { fetch: net.fetch };
