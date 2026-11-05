import type { ChangeEvent, ComponentType, FormEvent } from "react";
import { useState } from "react";
import { createRoot } from "react-dom/client";
import type { Ask, Props } from "./design.ts";

export const DOOR = "/_unlock";

export const HEADER = "x-lock";

export async function knock(password: string, ask: typeof fetch = fetch): Promise<boolean> {
  const answer = await ask(DOOR, {
    headers: { [HEADER]: encodeURIComponent(password) },
    cache: "no-store",
    credentials: "same-origin",
  });
  return answer.status === 204;
}

export type Asked = {
  Lock: ComponentType<Props["lock"]>;
  word: string;
  enter?: (password: string) => Promise<boolean>;
  done?: () => void;
};

const bare = () => ({});

export function Unlock({ Lock, word, enter = knock, done = () => location.reload() }: Asked) {
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

export function open(Lock: Asked["Lock"], word: string, host: Element = document.body) {
  document.documentElement.classList.add("js");
  const root = document.createElement("div");
  host.append(root);
  createRoot(root).render(<Unlock Lock={Lock} word={word} />);
}
