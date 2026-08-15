import { useCallback, useEffect, useState } from "react"
import { Box, Button, Caption, Cluster, Crumbs, Skeleton, Stack, Symbol, Title } from "mrlyui"
import { Shell } from "../components/Shell"
import { asset, held, seek } from "../lib/films"
import type { Film } from "../lib/films"
import type { Site } from "../lib/data"
import { BASE, ROOT } from "../lib/site"

// FACTS

function facts(film: Film): string {
  const parts = [`seed ${film.seed}`]
  if (film.frames > 0) parts.push(`${String(film.frames)} generations`)
  if (film.duration > 0) parts.push(`${film.duration.toFixed(1)}s`)
  return parts.join(" · ")
}

// SHARE

function Share({ link }: { link: string }) {
  const [copied, set] = useState(false)
  const copy = useCallback(() => {
    try {
      void navigator.clipboard.writeText(link).then(
        () => {
          set(true)
        },
        () => {},
      )
    } catch {}
  }, [link])
  return (
    <Stack tight>
      <Cluster>
        <Button onClick={copy} primary>
          <Symbol name={copied ? "check" : "content_copy"} />
          {copied ? "Copied" : "Copy link"}
        </Button>
        <Button href="/">All films</Button>
      </Cluster>
      <Caption className="focus-link">{link}</Caption>
    </Stack>
  )
}

// FOCUS

export function Focus({ site, name }: { site: Site; name: string }) {
  const [film, set] = useState<Film | undefined>(() => held(name))
  const [gone, setGone] = useState(false)
  useEffect(() => {
    const got = held(name)
    if (got !== undefined) {
      set(got)
      return
    }
    let live = true
    void seek(name).then(
      found => {
        if (!live) return
        if (found === undefined) setGone(true)
        else set(found)
      },
      () => {
        if (live) setGone(true)
      },
    )
    return () => {
      live = false
    }
  }, [name])
  const desc =
    film === undefined ? `A MrlyLife film on ${ROOT}.` : `MrlyLife film ${film.name}, grown from seed ${film.seed}.`
  return (
    <Shell site={site} route={`film/${name}`} title={name} desc={desc} wide>
      <Crumbs root={ROOT} route={name} />
      {film === undefined && !gone && <Skeleton lines={2} block />}
      {gone && (
        <Box>
          <Stack tight>
            <Title>No such film.</Title>
            <Caption>It may have rolled off the reel.</Caption>
            <Cluster>
              <Button href="/" primary>
                All films
              </Button>
            </Cluster>
          </Stack>
        </Box>
      )}
      {film !== undefined && (
        <>
          <video
            className="focus-reel"
            src={asset(film.video)}
            poster={asset(film.poster)}
            autoPlay
            loop
            muted
            playsInline
            controls
          />
          <Box>
            <Stack tight>
              <Title>{film.name}</Title>
              <Caption>{facts(film)}</Caption>
              <Share link={`${BASE}/film/${encodeURIComponent(film.name)}`} />
            </Stack>
          </Box>
        </>
      )}
    </Shell>
  )
}
