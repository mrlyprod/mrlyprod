import { useEffect, useRef, useState } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"
import { VARIANT_ICON, type Variant } from "./variant"

let held: HTMLElement | null = null

function layer(): HTMLElement {
  if (held === null) {
    held = document.createElement("div")
    held.className = "toast-layer"
    document.body.append(held)
  }
  return held
}

function Body({ variant, children }: { variant?: Variant; children: ReactNode }) {
  return (
    <div className={cx("toast", variant)} role="status">
      {variant && (
        <span className="toast-icon" aria-hidden="true">
          <Symbol name={VARIANT_ICON[variant]} size="var(--font-size)" />
        </span>
      )}
      <span>{children}</span>
    </div>
  )
}

// TOAST

export function Toast({ open, children, variant, duration = 2500, onClose }: {
  open: boolean
  children: ReactNode
  variant?: Variant
  duration?: number
  onClose: () => void
}) {
  const close = useRef(onClose)
  useEffect(() => {
    close.current = onClose
  }, [onClose])
  useEffect(() => {
    if (!open) return
    const id = setTimeout(() => close.current(), duration)
    return () => clearTimeout(id)
  }, [open, duration])
  if (!open || typeof document === "undefined") return null
  return createPortal(<Body variant={variant}>{children}</Body>, layer())
}

// QUEUE

type Note = { id: number; body: ReactNode; variant: Variant | undefined }

let notes: Note[] = []
let seq = 0
const watchers = new Set<(notes: Note[]) => void>()

function publish(): void {
  for (const watcher of watchers) watcher(notes)
}

function dismiss(id: number): void {
  notes = notes.filter(note => note.id !== id)
  publish()
}

export function toast(body: ReactNode, variant?: Variant): void {
  seq += 1
  notes = [...notes, { id: seq, body, variant }]
  publish()
}

function Popped({ note, duration }: { note: Note; duration: number }) {
  useEffect(() => {
    const id = setTimeout(() => dismiss(note.id), duration)
    return () => clearTimeout(id)
  }, [note.id, duration])
  return <Body variant={note.variant}>{note.body}</Body>
}

// TOASTER

export function Toaster({ duration = 4000 }: { duration?: number }) {
  const [shown, setShown] = useState<Note[]>(notes)
  useEffect(() => {
    watchers.add(setShown)
    setShown(notes)
    return () => {
      watchers.delete(setShown)
    }
  }, [])
  if (typeof document === "undefined") return null
  return createPortal(
    shown.map(note => <Popped key={note.id} note={note} duration={duration} />),
    layer(),
  )
}
