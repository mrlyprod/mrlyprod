import { useSyncExternalStore } from "react";
import { read, subscribe } from "../../shell/prefs.ts";

export const usePref = (key: string) => useSyncExternalStore(subscribe, () => read(key));
