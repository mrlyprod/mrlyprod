import { useSyncExternalStore } from "react";
import { read, subscribe } from "../../shell/prefs.js";

/* PREF */

export const usePref = (key) => useSyncExternalStore(subscribe, () => read(key));

/* FORM */

const html = () => document.documentElement;

const form = () => (html().dataset.form === "phone" ? "phone" : "desk");

function watch(fn) {
  const seen = new MutationObserver(fn);
  seen.observe(html(), { attributes: true, attributeFilter: ["data-form"] });
  return () => seen.disconnect();
}

export const useForm = () => useSyncExternalStore(watch, form);
