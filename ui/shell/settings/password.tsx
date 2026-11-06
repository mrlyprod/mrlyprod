import type { FormEvent } from "react";
import { useEffect, useRef, useState } from "react";
import { ask } from "../../kit/controls.tsx";
import { choose, forget, held, matches } from "../password.ts";
import { Action, Item, Note, Section } from "./ui.tsx";

type Step = "set" | "change" | "remove";

type FieldProps = { label: string; name: string; auto: string };

const YES: Record<Step, string> = { set: "Set", change: "Change", remove: "Remove" };

function Field({ label, name, auto }: FieldProps) {
  return (
    <label className="item">
      <span className="words">
        <span className="name">{label}</span>
      </span>
      <span className="end">
        <input className="priv-field" type="password" name={name} autoComplete={auto} required />
      </span>
    </label>
  );
}

export function Password() {
  const [on, setOn] = useState(held);
  const [step, open] = useState<Step | null>(null);
  const [busy, setBusy] = useState(false);
  const form = useRef<HTMLFormElement>(null);

  useEffect(() => {
    if (step) form.current?.querySelector("input")?.focus();
  }, [step]);

  async function apply(given: FormData) {
    const said = (name: string) => String(given.get(name) ?? "");
    if (step !== "set" && !(await matches(said("current")))) return ask({ title: "Wrong password." });
    if (step !== "remove" && said("next") !== said("again")) return ask({ title: "The new passwords differ." });
    if (step === "remove") forget();
    else await choose(said("next"));
    setOn(held());
    open(null);
  }

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (busy) return;
    setBusy(true);
    apply(new FormData(e.currentTarget))
      .catch(() => ask({ title: "This browser cannot set a password here." }))
      .finally(() => setBusy(false));
  }

  return (
    <Section title="Lock screen">
      {step ? (
        <form ref={form} className="priv-form" onSubmit={submit} aria-busy={busy || undefined}>
          {step !== "set" && <Field label="Current password" name="current" auto="current-password" />}
          {step !== "remove" && (
            <>
              <Field label="New password" name="next" auto="new-password" />
              <Field label="Confirm new password" name="again" auto="new-password" />
            </>
          )}
          <Item label="">
            <span className="priv-pair">
              <button type="button" className="priv-btn" onClick={() => open(null)}>
                Cancel
              </button>
              <button type="submit" className={step === "remove" ? "priv-btn hot" : "priv-btn"} disabled={busy}>
                {YES[step]}
              </button>
            </span>
          </Item>
        </form>
      ) : on ? (
        <Item label="Password" hint="On. Asked for once per tab. On the desktop, click the logo to lock now.">
          <span className="priv-pair">
            <button type="button" className="priv-btn" onClick={() => open("change")}>
              Change…
            </button>
            <button type="button" className="priv-btn" onClick={() => open("remove")}>
              Remove…
            </button>
          </span>
        </Item>
      ) : (
        <Action label="Password" hint="Off. The lock screen opens without one." onClick={() => open("set")}>
          Set…
        </Action>
      )}
      <Note>
        It only covers this screen, in this browser. It does not encrypt or protect your data, and the developer tools
        can remove it. Kept here as a salted hash, never sent or exported. Forgot it? Clear this site&apos;s data in
        your browser.
      </Note>
    </Section>
  );
}
