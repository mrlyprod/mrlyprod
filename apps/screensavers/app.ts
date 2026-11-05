import { app } from "@mrlyprod/os/contract/index.ts";

export default app({
  id: "screensavers",
  title: "Screensavers",
  line: "Preview and tune every screen saver.",
  boot: () => ({}),
  verbs: {},
  view: [{ kind: "viewer", name: "savers" }],
});
