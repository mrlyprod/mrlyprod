type Prompt = Event & {
  prompt: () => Promise<void>
  userChoice: Promise<{ outcome: "accepted" | "dismissed" }>
}

let held: Prompt | undefined

export function watch(): void {
  addEventListener("beforeinstallprompt", event => {
    event.preventDefault()
    held = event as Prompt
  })
  addEventListener("appinstalled", () => {
    held = undefined
  })
}

export function standalone(): boolean {
  return matchMedia("(display-mode: standalone)").matches
}

export async function install(): Promise<string> {
  if (standalone()) return "already installed"
  if (held === undefined) return "this browser offers no install prompt"
  await held.prompt()
  const { outcome } = await held.userChoice
  if (outcome === "accepted") held = undefined
  return outcome === "accepted" ? "installing" : "install dismissed"
}
