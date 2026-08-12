import { Caption, Row } from "mrlyui"

export function Status({ lang, lines }: { lang?: string | undefined; lines?: number | undefined }) {
  const bits: string[] = []
  if (lang !== undefined && lang !== "") bits.push(lang)
  if (lines !== undefined && lines > 0) bits.push(`${String(lines)} lines`)
  if (bits.length === 0) return null
  return (
    <Row>
      {bits.map(bit => (
        <Caption key={bit}>{bit}</Caption>
      ))}
    </Row>
  )
}
