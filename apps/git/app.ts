import { app } from "@mrlyprod/os/contract/index.ts";

export default app({
  id: "git",
  title: "Git",
  line: "The source: every tracked file in a tree.",
  boot: () => ({}),
  verbs: {},
  view: [{ kind: "viewer", name: "code" }],
});
