import { reader } from "../kit/md/md.ts";
import type { Own } from "./router.ts";

export const page = (): Promise<Own> => reader<Own>();
