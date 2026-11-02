import { useSyncExternalStore } from "react";
import { read, subscribe } from "../../shell/prefs.ts";

/* PREF */

export const usePref = (key: string) => useSyncExternalStore(subscribe, () => read(key));
