import type { FileSource, ViewNode } from "@mrlyprod/os/contract/index.ts";
import { app } from "@mrlyprod/os/contract/index.ts";

const at = (path: string): FileSource => ({ file: "/stats/stats.json", path });

const view: ViewNode[] = [
  {
    kind: "group",
    title: "CDN",
    nodes: [
      { kind: "value", from: at("cdn/requests"), as: "number", unit: "request", title: "requests" },
      { kind: "value", from: at("cdn/bytes"), as: "number", unit: "byte", title: "sent" },
      { kind: "value", from: at("cdn/error_4xx"), as: "number", title: "4xx %" },
      { kind: "value", from: at("cdn/error_5xx"), as: "number", title: "5xx %" },
      { kind: "chart", from: at("cdn/hourly"), x: "at", y: "requests", shape: "line", xunit: "second" },
    ],
  },
  {
    kind: "group",
    title: "Lambdas",
    nodes: [
      {
        kind: "rows",
        from: at("lambdas/*"),
        fields: [
          { key: "key", as: "text", title: "lambda" },
          { key: "invocations", as: "number", title: "calls" },
          { key: "errors", as: "number", title: "err" },
          { key: "throttles", as: "number", title: "thr" },
          { key: "duration_ms", as: "number", title: "ms" },
        ],
      },
      { kind: "chart", from: at("lambdas/*/hourly"), x: "at", y: "invocations", shape: "line", xunit: "second" },
    ],
  },
  {
    kind: "group",
    title: "Bucket",
    nodes: [
      { kind: "value", from: at("bucket/site_bytes"), as: "number", unit: "byte", title: "site" },
      {
        kind: "chart",
        from: at("bucket/*"),
        x: "key",
        y: "value",
        shape: "bars",
        title: "objects",
        skip: ["site_bytes"],
      },
    ],
  },
  {
    kind: "group",
    title: "Errors",
    nodes: [
      {
        kind: "rows",
        from: at("errors/*"),
        fields: [
          { key: "at", as: "time", title: "time" },
          { key: "key", as: "text", title: "lambda" },
          { key: "level", as: "text" },
          { key: "message", as: "text" },
        ],
      },
    ],
  },
  {
    kind: "group",
    nodes: [
      { kind: "value", from: at("at"), as: "time", unit: "second", title: "updated" },
      { kind: "value", from: at("hours"), as: "number", unit: "hour", title: "last" },
      { kind: "do", action: "open", to: "/stats/stats.json", title: "raw" },
    ],
  },
];

export default app({
  id: "stats",
  title: "Stats",
  line: "The CDN, the Lambdas and the bucket.",
  boot: () => ({}),
  verbs: {},
  view,
});
