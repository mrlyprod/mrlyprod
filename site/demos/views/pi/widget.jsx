import { useMemo, useState } from 'react';
import { ready } from '../../../lib/mrly.js';
import { Row, Slider, Check } from '../../../lib/app.jsx';
import { Pixels } from '../../../lib/draw.jsx';
import { embed } from '../../../lib/widget.jsx';

const m = await ready();

const SIDE = 720;

export function window() {
  const [n, setN] = useState(100);
  const [layers, setLayers] = useState(true);
  const sheet = useMemo(() => m.visible_pixels(n, SIDE, layers), [n, layers]);
  const read = useMemo(() => JSON.parse(m.visible_read(n, 2)), [n]);
  return (
    <>
      <Pixels data={sheet} role="img" aria-label="The corner window of the grid, every visible point lit and every hidden one shaded by the divisor that hides it" />
      <Row>
        <Slider label="window n" value={n} min={8} max={300} onChange={setN} />
        <Check label="shade the layers" checked={layers} onChange={setLayers} />
      </Row>
      <p className="sub">{`${read.lit} visible of ${read.total} points, a share of ${read.density.toFixed(6)}`}</p>
    </>
  );
}

embed('pi', { window });
