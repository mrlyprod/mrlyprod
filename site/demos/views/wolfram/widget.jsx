import { useMemo, useState } from 'react';
import { ready, ink } from '../../../lib/mrly.js';
import { Row, Slider } from '../../../lib/app.jsx';
import { Grid } from '../../../lib/draw.jsx';
import { embed } from '../../../lib/widget.jsx';

const m = await ready();

const STEPS = 127;

export const run = (number) => {
  const grid = m.eca_seed(number, STEPS);
  let live = 0;
  for (const bit of grid.types) live += bit;
  return { grid: { width: grid.width, height: grid.height, types: grid.types }, live };
};

export function rule() {
  const [number, setNumber] = useState(90);
  const seen = useMemo(() => run(number), [number]);
  return (
    <>
      <Grid grid={seen.grid} on={ink.yellow} role="img" aria-label={`Rule ${number} from one live cell, ${seen.grid.height} generations`} />
      <Row>
        <Slider label="rule" value={number} min={0} max={255} onChange={setNumber} />
      </Row>
      <p className="sub">{`rule ${number} from one live cell, ${seen.grid.height} generations on ${seen.grid.width} cells, ${seen.live} of them live`}</p>
    </>
  );
}

embed('wolfram', { rule });
