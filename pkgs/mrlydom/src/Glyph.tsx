// SHAPES

const RECTS: Record<string, string> = {
  plus: '<rect x="2" y="0" width="1" height="1"/><rect x="2" y="1" width="1" height="1"/><rect x="0" y="2" width="5" height="1"/><rect x="2" y="3" width="1" height="1"/><rect x="2" y="4" width="1" height="1"/>',
  minus: '<rect x="0" y="2" width="5" height="1"/>',
  cross: '<rect x="0" y="0" width="5" height="1"/><rect x="0" y="1" width="1" height="1"/><rect x="2" y="1" width="1" height="1"/><rect x="4" y="1" width="1" height="1"/><rect x="0" y="2" width="5" height="1"/><rect x="0" y="3" width="1" height="1"/><rect x="2" y="3" width="1" height="1"/><rect x="4" y="3" width="1" height="1"/><rect x="0" y="4" width="5" height="1"/>',
  ring: '<rect x="0" y="0" width="5" height="1"/><rect x="0" y="1" width="1" height="1"/><rect x="4" y="1" width="1" height="1"/><rect x="0" y="2" width="1" height="1"/><rect x="4" y="2" width="1" height="1"/><rect x="0" y="3" width="1" height="1"/><rect x="4" y="3" width="1" height="1"/><rect x="0" y="4" width="5" height="1"/>',
}

// GLYPH

export function Glyph({ name, cls }: { name: string; cls: string }) {
  return (
    <svg
      className={cls}
      viewBox="0 0 5 5"
      fill="currentColor"
      shapeRendering="crispEdges"
      aria-hidden="true"
      dangerouslySetInnerHTML={{ __html: RECTS[name] ?? "" }}
    />
  )
}
