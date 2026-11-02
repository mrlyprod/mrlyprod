import { reader } from "../sdk/md/md.ts";
import type { Own } from "./router.ts";

export const page = (): Promise<Own> => reader<Own>();
