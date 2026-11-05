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

export const SPEC = "0.2";
