import type { ViewNode, Who } from "@mrlyprod/os/contract/index.ts";
import { app, walk } from "@mrlyprod/os/contract/index.ts";

type Settings = { people: Record<string, Record<string, string>> };

const GUEST = "guest";

const NAMES =
  "history theme tint text icons always magnify saver off idle clock hours motion clear contrast targets dock widgets launchpad page wide";

const KEY = `^(${NAMES.split(" ").join("|")}|knob-[a-z]+)$`;

const LONGEST = 20000;

const mine = (state: Settings, id: string) => (Object.hasOwn(state.people, id) ? state.people[id]! : {});

const seen = (state: Settings, path: string[], who: Who | null) => {
  const id = who?.id ?? GUEST;
  const [head = "", ...rest] = path;
  if (!head) return mine(state, id);
  return head === "@me" || head === `@${id}` ? walk(mine(state, id), rest) : undefined;
};

const view: ViewNode[] = [
  {
    kind: "pane",
    id: "appearance",
    title: "Appearance",
    nodes: [
      {
        kind: "group",
        nodes: [
          {
            kind: "pref",
            key: "theme",
            as: "pick",
            title: "Appearance",
            art: "theme",
            options: [
              ["", "Auto"],
              ["light", "Light"],
              ["dark", "Dark"],
            ],
          },
          { kind: "pref", key: "tint", as: "color", title: "Accent colour" },
        ],
      },
      {
        kind: "group",
        nodes: [
          {
            kind: "pref",
            key: "text",
            as: "pick",
            title: "Text size",
            options: [
              ["small", "Small"],
              ["", "Default"],
              ["large", "Large"],
              ["larger", "Larger"],
            ],
          },
        ],
      },
      { kind: "viewer", name: "knobs" },
      {
        kind: "group",
        title: "Icon & widget style",
        needs: "icons",
        nodes: [
          {
            kind: "pref",
            key: "icons",
            as: "pick",
            title: "Icon & widget style",
            art: "icons",
            options: [
              ["", "Default"],
              ["dark", "Dark"],
              ["clear", "Clear"],
              ["tinted", "Tinted"],
            ],
          },
          { kind: "pref", key: "always", as: "toggle", on: "1", title: "Icons always dark" },
        ],
      },
      {
        kind: "group",
        title: "Clock",
        nodes: [
          {
            kind: "pref",
            key: "hours",
            as: "pick",
            title: "Time format",
            options: [
              ["", "Auto"],
              ["12", "12-hour"],
              ["24", "24-hour"],
            ],
          },
        ],
      },
    ],
  },
  { kind: "pane", id: "desktop", title: "Desktop & Dock", nodes: [{ kind: "viewer", name: "desktop" }] },
  {
    kind: "pane",
    id: "screensaver",
    title: "Screen Saver",
    nodes: [
      {
        kind: "group",
        nodes: [
          {
            kind: "pref",
            key: "idle",
            as: "pick",
            title: "Start after",
            options: [
              ["2", "2 minutes"],
              ["", "5 minutes"],
              ["10", "10 minutes"],
              ["never", "Never"],
            ],
          },
          { kind: "pref", key: "clock", as: "toggle", on: "", off: "off", title: "Show clock" },
        ],
      },
      { kind: "viewer", name: "shuffle" },
    ],
  },
  {
    kind: "pane",
    id: "accessibility",
    title: "Accessibility",
    nodes: [
      {
        kind: "group",
        title: "Display",
        nodes: [
          { kind: "pref", key: "contrast", as: "toggle", on: "more", title: "Increase contrast" },
          { kind: "pref", key: "clear", as: "toggle", on: "1", title: "Reduce transparency" },
          { kind: "pref", key: "motion", as: "toggle", on: "reduce", title: "Reduce motion" },
        ],
      },
      {
        kind: "group",
        title: "Controls",
        nodes: [{ kind: "pref", key: "targets", as: "toggle", on: "large", title: "Larger targets" }],
      },
    ],
  },
  {
    kind: "pane",
    id: "privacy",
    title: "Privacy & Data",
    nodes: [
      {
        kind: "group",
        title: "History",
        nodes: [
          {
            kind: "pref",
            key: "history",
            as: "pick",
            title: "Acts kept",
            options: [
              ["0", "Off"],
              ["50", "50"],
              ["", "200"],
              ["1000", "1000"],
            ],
          },
          { kind: "do", action: "wipe", title: "Wipe history", sure: true },
        ],
      },
      { kind: "viewer", name: "privacy" },
    ],
  },
  { kind: "pane", id: "about", title: "About", nodes: [{ kind: "viewer", name: "about" }] },
];

export default app<Settings>({
  id: "settings",
  title: "Settings",
  line: "One set of preferences per person.",
  boot: () => ({ people: {} }),
  verbs: {
    set: {
      line: "Set one preference; an empty value puts it back to its default.",
      guest: true,
      args: {
        type: "object",
        properties: { key: { type: "string", pattern: KEY }, value: { type: "string", maxLength: LONGEST } },
        required: ["key", "value"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const key = String(args.key);
        const value = String(args.value);
        const { [key]: _was, ...rest } = mine(state, ctx.who.id);
        const next = value ? { ...rest, [key]: value } : rest;
        ctx.say({ note: value ? `${key} is ${value}` : `${key} is back to its default` });
        return { people: { ...state.people, [ctx.who.id]: next } };
      },
    },
  },
  read: seen,
  view,
});
