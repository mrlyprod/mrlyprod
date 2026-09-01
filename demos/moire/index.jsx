import { useEffect, useRef, useState } from 'react';
import { ready } from '../lib/mrly.js';
import { mount, Page, Row, Pick, Slider, Btn, Stats, Stat, Note } from '../lib/app.jsx';
import { Pixels } from '../lib/draw.jsx';
import { useQuery, stamp } from '../lib/query.js';
import { useSeeds, roll, Ramp, Cropper, cropOf } from '../lib/select.jsx';

const m = await ready();
const NAMES = [...m.moire_names()];
const SIZES = [128, 256, 384, 512];

const drawn = (seed) => {
  const [preset, limit] = roll(seed, [[0, NAMES.length - 1], [1, 41]]);
  return { preset: NAMES[preset], limit: limit | 1 };
};

function App() {
  const s = useSeeds();
  const [pick, setPick] = useState({ preset: NAMES[0], limit: 9, size: 256, ...(s.get() ? drawn(s.get()) : null) });
  const [look, setLook] = useState({ ramp: 'fire', levels: 16, invert: false });
  const [crop, setCrop] = useQuery({ crop: '', 'crop-r': 16, 'crop-anti': false });
  const [playing, setPlaying] = useState(false);
  const shown = useRef(null);

  let error = null;
  try {
    const c = cropOf(crop);
    let pixels;
    if (c.active) {
      const field = m.field_crop(m.moire_field(pick.preset, pick.limit, pick.size), pick.size, 2, c.shape, c.rnum, c.rden, c.anti);
      let low = Infinity, high = -Infinity;
      for (const v of field) if (!Number.isNaN(v)) { low = Math.min(low, v); high = Math.max(high, v); }
      pixels = m.paint_span(field, pick.size, low, high, look.ramp, look.levels, look.invert);
    } else {
      pixels = m.moire(pick.preset, pick.limit, pick.size, look.ramp, look.levels, look.invert);
    }
    shown.current = { pixels, scales: Array.from(m.odd_scales(pick.limit)).join(' '), size: pick.size };
  } catch (fault) {
    error = fault;
  }

  const step = () => setPick((old) => ({ ...old, limit: old.limit >= 41 ? 1 : old.limit + 2 }));

  useEffect(() => {
    if (!playing) return;
    const timer = setInterval(step, 350);
    return () => clearInterval(timer);
  }, [playing]);

  const view = shown.current;

  return (
    <Page crumb="moire" title="Moire"
      sub="One design sampled at scale 1, 3, 5, and so on, the layers stacked into one field. Stacking is where the interference comes from: each new scale adds a finer grid on top of the coarse ones."
      foot="The heatmap sums the parity of the low corner over the odd scales, the weave folds the same layers to their parity, the hive samples on the hexagonal lattice, and the carpet keeps eight corners of nine in base 3. The field is quantized into levels and painted through a ramp; the pixels arrive already colored.">
      <Row>
        <Pick label="preset" value={pick.preset} options={NAMES} onChange={(v) => setPick({ ...pick, preset: v })} />
        <Slider label="scales up to" value={pick.limit} min={1} max={41} step={2} onChange={(v) => setPick({ ...pick, limit: v })} />
        <Pick label="size" value={pick.size} options={SIZES.map((v) => [v, v])} onChange={(v) => setPick({ ...pick, size: +v })} />
        <Ramp value={look} onChange={(patch) => setLook({ ...look, ...patch })} />
        <Cropper value={crop} onChange={(patch) => { setCrop(patch); if (!({ ...crop, ...patch }).crop) stamp({ crop: null, 'crop-r': null, 'crop-anti': null }); }} />
        <Btn onClick={() => { setPlaying(!playing); if (!playing) step(); }}>{playing ? 'Stop' : 'Play the scales'}</Btn>
        <Btn onClick={() => setPick({ ...pick, ...drawn(s.next()) })}>Randomize</Btn>
      </Row>
      {view && <Pixels data={view.pixels} style={{ maxWidth: 640 }} />}
      <Stats>
        <Stat label="scales">{view?.scales}</Stat>
        <Stat label="pixels">{view && `${view.size} by ${view.size}`}</Stat>
      </Stats>
      <Note error={error} />
    </Page>
  );
}

mount(<App />);
