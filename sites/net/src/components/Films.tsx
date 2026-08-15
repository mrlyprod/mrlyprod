import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react"
import { Caption, Section, Spinner, cx, useMedia } from "mrlyui"
import { asset, offset, remember, useReel } from "../lib/films"
import type { Film } from "../lib/films"

// SHAPE

const MIN = 148

const OVER = 2

const EDGE = "900px 0px"

const BLANKS = 8

type Metric = { cols: number; cell: number; gap: number }

function scroller(node: HTMLElement): { scrollBy: (options: ScrollToOptions) => void } {
  let at = node.parentElement
  while (at !== null) {
    const flow = getComputedStyle(at).overflowY
    if ((flow === "auto" || flow === "scroll") && at.scrollHeight > at.clientHeight) return at
    at = at.parentElement
  }
  return window
}

// MOTION

function Motion({ film }: { film: Film }) {
  const [ready, setReady] = useState(false)
  return (
    <video
      className={cx("film-motion", ready && "ready")}
      src={asset(film.video)}
      autoPlay
      loop
      muted
      playsInline
      preload="auto"
      tabIndex={-1}
      aria-hidden="true"
      ref={node => {
        if (node !== null) node.muted = true
      }}
      onCanPlay={event => {
        const node = event.currentTarget
        node.muted = true
        void node.play().then(
          () => {},
          () => {},
        )
        setReady(true)
      }}
    />
  )
}

// CELL

function Cell({ film, still }: { film: Film; still: boolean }) {
  const hold = useRef<HTMLAnchorElement>(null)
  const [near, setNear] = useState(false)
  const [shot, setShot] = useState(true)
  useEffect(() => {
    const node = hold.current
    if (node === null) return
    const eye = new IntersectionObserver(
      entries => {
        for (const entry of entries) setNear(entry.isIntersecting)
      },
      { threshold: 0.2 },
    )
    eye.observe(node)
    return () => {
      eye.disconnect()
    }
  }, [])
  return (
    <a className="film" href={`/film/${encodeURIComponent(film.name)}`} ref={hold}>
      {shot && (
        <img
          className="film-poster"
          src={asset(film.poster)}
          alt=""
          loading="lazy"
          decoding="async"
          onError={() => {
            setShot(false)
          }}
        />
      )}
      {near && !still && <Motion film={film} />}
      <span className="film-name">{film.name}</span>
    </a>
  )
}

// FILMS

export function Films() {
  const { films, count, done, failed, pull } = useReel()
  const still = useMedia("(prefers-reduced-motion: reduce)")
  const hold = useRef<HTMLDivElement>(null)
  const grid = useRef<HTMLDivElement>(null)
  const edge = useRef<HTMLDivElement>(null)
  const frame = useRef(0)
  const back = useRef(offset())
  const leaving = useRef(false)
  const [metric, setMetric] = useState<Metric | undefined>(undefined)
  const [span, setSpan] = useState<[number, number]>([0, 0])
  const total = films.length

  const measure = useCallback(() => {
    const outer = hold.current
    const inner = grid.current
    if (outer === null || inner === null) return
    const width = outer.clientWidth
    if (width <= 0) return
    const gap = Number.parseFloat(getComputedStyle(inner).rowGap) || 0
    const cols = Math.max(1, Math.floor((width + gap) / (MIN + gap)))
    const cell = (width - gap * (cols - 1)) / cols
    setMetric(next =>
      next !== undefined && next.cols === cols && next.cell === cell && next.gap === gap ? next : { cols, cell, gap },
    )
  }, [])

  useLayoutEffect(() => {
    measure()
    const outer = hold.current
    if (outer === null) return
    const eye = new ResizeObserver(measure)
    eye.observe(outer)
    return () => {
      eye.disconnect()
    }
  }, [measure])

  const sweep = useCallback(() => {
    const outer = hold.current
    if (outer === null || metric === undefined) return
    const step = metric.cell + metric.gap
    if (step <= 0) return
    const rows = Math.ceil(total / metric.cols)
    const top = Math.max(0, -outer.getBoundingClientRect().top)
    if (!leaving.current) remember(top)
    const first = Math.max(0, Math.floor(top / step) - OVER)
    const last = Math.min(rows, Math.ceil((top + window.innerHeight) / step) + OVER)
    setSpan(next => (next[0] === first && next[1] === last ? next : [first, Math.max(first, last)]))
  }, [metric, total])

  useEffect(() => {
    sweep()
    const tick = (): void => {
      if (frame.current !== 0) return
      frame.current = requestAnimationFrame(() => {
        frame.current = 0
        sweep()
      })
    }
    document.addEventListener("scroll", tick, { capture: true, passive: true })
    window.addEventListener("resize", tick)
    return () => {
      document.removeEventListener("scroll", tick, { capture: true })
      window.removeEventListener("resize", tick)
      if (frame.current !== 0) cancelAnimationFrame(frame.current)
      frame.current = 0
    }
  }, [sweep])

  useLayoutEffect(() => {
    const top = back.current
    const outer = hold.current
    if (top <= 0 || metric === undefined || total === 0 || outer === null) return
    back.current = 0
    scroller(outer).scrollBy({ top: outer.getBoundingClientRect().top + top })
  }, [metric, total])

  useEffect(() => {
    const node = edge.current
    if (node === null || done || failed) return
    const eye = new IntersectionObserver(
      entries => {
        for (const entry of entries) if (entry.isIntersecting) pull()
      },
      { rootMargin: EDGE },
    )
    eye.observe(node)
    return () => {
      eye.disconnect()
    }
  }, [done, failed, pull, total])

  if (total === 0 && (failed || done)) return null
  const step = metric === undefined ? 0 : metric.cell + metric.gap
  const rows = metric === undefined ? 0 : Math.ceil(total / metric.cols)
  const [first, last] = span
  const shown = metric === undefined ? [] : films.slice(first * metric.cols, last * metric.cols)
  const track = metric === undefined ? "" : `repeat(${String(metric.cols)}, minmax(0, 1fr))`
  const style = track === "" ? undefined : { gridTemplateColumns: track }
  return (
    <Section label="films" className="reel">
      <div
        className="films"
        ref={hold}
        onClickCapture={() => {
          leaving.current = true
        }}
      >
        <div className="films-pad" style={{ height: first * step }} />
        <div className="films-grid" ref={grid} style={style}>
          {total === 0
            ? Array.from({ length: BLANKS }, (_, at) => <span className="film blank" key={at} />)
            : shown.map(film => <Cell key={film.name} film={film} still={still} />)}
        </div>
        <div className="films-pad" style={{ height: Math.max(0, (rows - last) * step) }} />
        <div className="films-edge" ref={edge} />
      </div>
      <div className="films-foot">
        {!done && !failed && <Spinner size="calc(var(--font-size) * 1.4)" />}
        <Caption>{done ? `${String(total)} films` : `${String(total)} of ${String(count)} films`}</Caption>
      </div>
    </Section>
  )
}
