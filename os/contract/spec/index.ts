export type Type = "integer" | "number" | "string" | "boolean" | "object" | "array" | "null";

export type Schema = {
  type?: Type;
  title?: string;
  line?: string;
  default?: unknown;
  enum?: unknown[];
  const?: unknown;
  minimum?: number;
  maximum?: number;
  multipleOf?: number;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  properties?: Record<string, Schema>;
  required?: string[];
  additionalProperties?: boolean;
  items?: Schema;
  minItems?: number;
  maxItems?: number;
};

export type WorldSource = { world: string };

export type FileSource = { file: string; path?: string };

export type Source = WorldSource | FileSource;

export type Shown = "text" | "number" | "time" | "person";

export type Field = { key: string; as: Shown; title?: string; unit?: string; names?: WorldSource };

export type Rank = "primary" | "secondary" | "rare";

export type ValueNode = { kind: "value"; from: Source; as: Shown; title: string; unit?: string; guest?: false };

export type RowsNode = {
  kind: "rows";
  from: Source;
  fields: Field[];
  title?: string;
  reverse?: true;
  guest?: false;
};

export type FormNode = {
  kind: "form";
  verb: string;
  rank: Rank;
  choices?: Record<string, WorldSource | WorldSource[]>;
  guest?: false;
};

export type ChartNode = {
  kind: "chart";
  from: Source;
  x: string;
  y: string;
  shape: "line" | "bars";
  title?: string;
  xunit?: "second";
  skip?: string[];
  guest?: false;
};

export type ViewerNode = { kind: "viewer"; name: string; guest?: false };

export type DoNode = { kind: "do"; action: string; title: string; to?: string; sure?: true; guest?: false };

export type GroupNode = { kind: "group"; nodes: ViewNode[]; title?: string; needs?: string; guest?: false };

export type PrefNode = {
  kind: "pref";
  key: string;
  as: "pick" | "toggle" | "color";
  title: string;
  options?: [string, string][];
  on?: string;
  off?: string;
  art?: string;
  guest?: false;
};

export type PaneNode = { kind: "pane"; id: string; title: string; nodes: ViewNode[]; guest?: false };

export type ViewNode =
  | ValueNode
  | RowsNode
  | FormNode
  | ChartNode
  | ViewerNode
  | DoNode
  | GroupNode
  | PrefNode
  | PaneNode;

export const SPEC = "0.2";
