import { useEffect } from "react"

// COPY

async function copy(btn: HTMLElement): Promise<void> {
  const href = btn.dataset["raw"]
  const word = btn.querySelector(".word")
  if (href === undefined || word === null) return
  const body = await fetch(href).then(r => r.text())
  await navigator.clipboard.writeText(body)
  word.textContent = "copied"
  setTimeout(() => {
    word.textContent = "copy"
  }, 1500)
}

// CHROME

export function useChrome(stray: () => void): void {
  useEffect(() => {
    const click = (event: MouseEvent): void => {
      const target = event.target instanceof Element ? event.target : null
      if (target === null) return
      const menu = document.querySelector(".more[open]")
      if (menu !== null && target.closest(".more") !== menu) menu.removeAttribute("open")
      const copier = target.closest(".copy")
      if (copier instanceof HTMLElement) void copy(copier).catch(() => {})
      if (target.closest(".lettermark") !== null) {
        event.preventDefault()
        stray()
      }
    }
    document.addEventListener("click", click)
    return () => document.removeEventListener("click", click)
  }, [stray])
}
