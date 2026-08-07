import { names } from "../palette.ts"

export const SURFACES = ["grid", "canvas"]

export const NUMBERS = ["3", "5", "7", "9"]

export const LEVELS = ["1", "2", "3", "4"]

export const SUBPIXELS = ["1", "2", "4"]

export const colors = (): string[] => names()

export const DESIGNS_SOLID = ["carpet", "net", "vtree", "htree", "solid"]

export const DESIGNS_VOID = ["carpet", "net", "htree", "vtree", "void"]

export const SKINS = ["tiles", "emojis", "digits"]
