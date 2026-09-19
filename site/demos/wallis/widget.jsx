import { useMemo, useState } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { Row, Slider, Stats, Stat } from '../../lib/app.jsx';
import { Grid } from '../../lib/draw.jsx';
import { embed } from '../../lib/widget.jsx';

const m = await ready();

const TOP = 3;

export function sieve() {
  const [level, setLevel] = useState(2);
  const seen = useMemo(() => ({
    grid: m.wallis_grid('odd', 3, level),
    read: JSON.parse(m.wallis_read('odd', 3, level, 2)),
  }), [level]);
  return (
    <>
      <Grid grid={seen.grid} on={ink.blue} role="img" aria-label="The plane Wallis sieve, the surviving squares inked and every dropped centre left as ground" />
      <Row>
        <Slider label="level" value={level} min={1} max={TOP} onChange={setLevel} />
      </Row>
      <Stats>
        <Stat label="side">{seen.read.side}</Stat>
        <Stat label="cells">{seen.read.cells}</Stat>
        <Stat label="area">{seen.read.ratio.toFixed(6)}</Stat>
      </Stats>
    </>
  );
}

embed('wallis', { sieve });
