import { lambda } from "@mrlyprod/os/hosts/cloud/builder.ts";

const net = lambda({
  source: "mrlyprod/mrlyprod",
  head: "build/net/head",
  dir: "/tmp/src",
  agent: "mrlynet-site-builder",
  bucket: "MRLYDEV_BUCKET",
  folder: "sites/net",
  sources: ["push", "schedule", "hook", "manual"],
  prepare: async (site, run) => {
    const out = await run([process.execPath, "../../os/hosts/cloud/wasm.ts"], site);
    return `wasm ${(out.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`;
  },
});

export default { fetch: net.fetch };
