type Props = {
  lang?: string | undefined
  lines?: number | undefined
  updated?: string | undefined
}

export function Status({ lang, lines, updated }: Props) {
  const bits: string[] = []
  if (lang !== undefined && lang !== "") bits.push(lang)
  if (lines !== undefined && lines > 0) bits.push(`${String(lines)} lines`)
  if (updated !== undefined && updated !== "") bits.push(updated)
  if (bits.length === 0) return null
  return (
    <div className="status">
      {bits.map(bit => (
        <span className="bit" key={bit}>
          {bit}
        </span>
      ))}
    </div>
  )
}
