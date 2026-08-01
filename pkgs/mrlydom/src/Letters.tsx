// MARK

const GROUPS = [
  '<rect x="1" y="1" width="5" height="1"/><rect x="1" y="2" width="1" height="1"/><rect x="3" y="2" width="1" height="1"/><rect x="5" y="2" width="1" height="1"/><rect x="1" y="3" width="1" height="1"/><rect x="3" y="3" width="1" height="1"/><rect x="5" y="3" width="1" height="1"/><rect x="1" y="4" width="1" height="1"/><rect x="3" y="4" width="1" height="1"/><rect x="5" y="4" width="1" height="1"/><rect x="1" y="5" width="1" height="1"/><rect x="3" y="5" width="1" height="1"/><rect x="5" y="5" width="1" height="1"/>',
  '<rect x="7" y="1" width="5" height="1"/><rect x="7" y="2" width="1" height="1"/><rect x="7" y="3" width="1" height="1"/><rect x="7" y="4" width="1" height="1"/><rect x="7" y="5" width="1" height="1"/>',
  '<rect x="13" y="1" width="1" height="1"/><rect x="13" y="2" width="1" height="1"/><rect x="13" y="3" width="1" height="1"/><rect x="13" y="4" width="1" height="1"/><rect x="13" y="5" width="5" height="1"/>',
  '<rect x="19" y="1" width="1" height="1"/><rect x="23" y="1" width="1" height="1"/><rect x="19" y="2" width="1" height="1"/><rect x="23" y="2" width="1" height="1"/><rect x="19" y="3" width="5" height="1"/><rect x="23" y="4" width="1" height="1"/><rect x="19" y="5" width="5" height="1"/>',
  '<rect x="25" y="1" width="5" height="1"/><rect x="25" y="2" width="1" height="1"/><rect x="29" y="2" width="1" height="1"/><rect x="25" y="3" width="5" height="1"/><rect x="25" y="4" width="1" height="1"/><rect x="25" y="5" width="1" height="1"/>',
  '<rect x="31" y="1" width="5" height="1"/><rect x="31" y="2" width="1" height="1"/><rect x="31" y="3" width="1" height="1"/><rect x="31" y="4" width="1" height="1"/><rect x="31" y="5" width="1" height="1"/>',
  '<rect x="37" y="1" width="5" height="1"/><rect x="37" y="2" width="1" height="1"/><rect x="41" y="2" width="1" height="1"/><rect x="37" y="3" width="1" height="1"/><rect x="41" y="3" width="1" height="1"/><rect x="37" y="4" width="1" height="1"/><rect x="41" y="4" width="1" height="1"/><rect x="37" y="5" width="5" height="1"/>',
  '<rect x="47" y="1" width="1" height="1"/><rect x="47" y="2" width="1" height="1"/><rect x="43" y="3" width="5" height="1"/><rect x="43" y="4" width="1" height="1"/><rect x="47" y="4" width="1" height="1"/><rect x="43" y="5" width="5" height="1"/>',
]

const BODY = GROUPS.map(rects => `<g>${rects}</g>`).join("\n")

// LETTERS

export function Letters({ cls }: { cls: string }) {
  return (
    <svg
      className={cls}
      viewBox="1 1 47 5"
      fill="currentColor"
      shapeRendering="crispEdges"
      aria-hidden="true"
      dangerouslySetInnerHTML={{ __html: BODY }}
    />
  )
}
