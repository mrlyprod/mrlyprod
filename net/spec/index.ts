import act from "./act.json";
import config from "./config.json";
import envelope from "./envelope.json";
import fault from "./fault.json";
import line from "./line.json";
import manifest from "./manifest.json";
import move from "./move.json";

/* TYPES */

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

/* SPEC */

export const SPEC = "0.1";

export type Name = "act" | "config" | "envelope" | "fault" | "line" | "manifest" | "move";

export const SCHEMAS = { act, config, envelope, fault, line, manifest, move } as unknown as Record<Name, Schema>;
