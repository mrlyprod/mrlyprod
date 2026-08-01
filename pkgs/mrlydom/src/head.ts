import { useEffect } from "react"

type Head = {
  route: string
  title: string
  desc: string
  root: string
  base: string
}

function meta(name: string, value: string): void {
  document.querySelector(`meta[name="${name}"]`)?.setAttribute("content", value)
}

export function useHead({ route, title, desc, root, base }: Head): void {
  useEffect(() => {
    if (title === "" && route !== "") return
    document.title = route === "" ? root : `${title} \u2014 ${root}`
    meta("description", desc)
    const canon = route === "" ? `${base}/` : `${base}/${route}`
    document.querySelector('link[rel="canonical"]')?.setAttribute("href", canon)
  }, [route, title, desc, root, base])
}
