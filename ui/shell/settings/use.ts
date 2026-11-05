import { useSyncExternalStore } from "react";
import { read, subscribe } from "../prefs.ts";

export const usePref = (key: string) => useSyncExternalStore(subscribe, () => read(key));
