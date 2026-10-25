import { useSyncExternalStore } from "react";
import { read, subscribe } from "../../shell/prefs.ts";

/* PREF */

export const usePref = (key: string) => useSyncExternalStore(subscribe, () => read(key));

/* FORM */

const html = () => document.documentElement;

const form = () => (html().dataset.form === "phone" ? "phone" : "desk");

function watch(fn: () => void) {
  const seen = new MutationObserver(fn);
  seen.observe(html(), { attributes: true, attributeFilter: ["data-form"] });
  return () => seen.disconnect();
}

export const useForm = () => useSyncExternalStore(watch, form);
