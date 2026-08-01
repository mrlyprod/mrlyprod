import type { Dir } from "../lib/repo"
import { join } from "../lib/repo"
import { RAW } from "../lib/site"

export function Listing({ route, dir }: { route: string; dir: Dir }) {
  if (dir.subdirs.length === 0 && dir.files.length === 0 && dir.assets.length === 0) return null
  return (
    <ul className="listing">
      {dir.subdirs.map(sub => (
        <li key={sub}>
          <a href={`/${join(route, sub)}`}>{sub}/</a>
        </li>
      ))}
      {dir.files.map(([name, file]) => (
        <li key={name}>
          <a href={`/${file}`}>{name}</a>
        </li>
      ))}
      {dir.assets.map(([name, path]) => (
        <li key={name}>
          <a href={`${RAW}${path}`}>{name}</a>
        </li>
      ))}
    </ul>
  )
}
