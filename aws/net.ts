import { lambda } from "../site/kit/lambda.ts";

/* WHERE */

const net = lambda({
  source: "mrlyprod/mrlyprod",
  head: "build/net/head",
  dir: "/tmp/src",
  agent: "mrlynet-site-builder",
  bucket: "MRLYDEV_BUCKET",
  folder: "site",
  shelf: "SHELF_REPO",
  sources: ["push", "schedule", "automator", "manual"],
  prepare: async (site, run) => {
    const out = await run([process.execPath, "scripts/pkg.ts"], site);
    return `pkg ${(out.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`;
  },
});

export default { fetch: net.fetch };
