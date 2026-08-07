import { useEffect, useState } from "react"

// CLASSES

export function cx(...parts: Array<string | false | null | undefined>): string {
  return parts.filter(Boolean).join(" ")
}

// STORE

export function read(key: string): string {
  try {
    return localStorage.getItem(key) ?? ""
  } catch {
    return ""
  }
}

export function write(key: string, value: string): void {
  try {
    if (value === "") localStorage.removeItem(key)
    else localStorage.setItem(key, value)
  } catch {}
}

// MEDIA

export function useMedia(query: string): boolean {
  const [matches, setMatches] = useState(false)
  useEffect(() => {
    const media = window.matchMedia(query)
    const update = () => {
      setMatches(media.matches)
    }
    update()
    media.addEventListener("change", update)
    return () => {
      media.removeEventListener("change", update)
    }
  }, [query])
  return matches
}

export function useMobile(): boolean {
  return !useMedia("(min-width: 700px)")
}
