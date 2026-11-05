import type { ChangeEvent, ComponentType, FormEvent } from "react";
import { useState } from "react";
import { createRoot } from "react-dom/client";
import { parts } from "./active.ts";
import type { Ask, Props } from "./design.ts";
import { admit, matches } from "./password.ts";
import { WORD } from "./welcome.ts";

export type Asked = {
  Lock: ComponentType<Props["lock"]>;
  word: string;
  enter: (password: string) => Promise<boolean>;
  done: () => void;
};

const bare = () => ({});

export function Unlock({ Lock, word, enter, done }: Asked) {
  const [value, setValue] = useState("");
  const [wrong, setWrong] = useState(false);
  const [busy, setBusy] = useState(false);
  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (busy || !value) return;
    setBusy(true);
    setWrong(false);
    if (await enter(value).catch(() => false)) return done();
    setValue("");
    setWrong(true);
    setBusy(false);
  };
  const ask: Ask = {
    wrong,
    busy,
    bind: {
      form: () => ({ onSubmit: submit, "aria-busy": busy || undefined }),
      field: () => ({
        type: "password",
        name: "password",
        autoComplete: "current-password",
        autoFocus: true,
        required: true,
        "aria-label": "Password",
        "aria-invalid": wrong || undefined,
        readOnly: busy,
        value,
        onChange: (e: ChangeEvent<HTMLInputElement>) => setValue(e.target.value),
      }),
    },
  };
  return <Lock word={word} state="wake" bind={bare} ask={ask} />;
}

let up = false;

export function guard() {
  if (up) return;
  if (!parts.lock) return admit();
  up = true;
  const host = document.createElement("div");
  host.className = "sealed";
  document.body.append(host);
  const root = createRoot(host);
  const done = () => {
    root.unmount();
    host.remove();
    up = false;
    admit();
  };
  root.render(<Unlock Lock={parts.lock} word={WORD} enter={matches} done={done} />);
}
