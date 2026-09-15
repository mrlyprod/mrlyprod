import { useEffect, useMemo, useState } from 'react';
import { ready, ink, fit } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Text, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid, Pixels, Sketch } from '../../lib/draw.jsx';
import { Picker, useSeeds } from '../../lib/select.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const SIZES = [128, 256, 512];
const SIDES = [3, 5, 7, 9];
const DEEPEST = 5;
const SHADES = 64;
const PAUSE = 120;

const BUGS = { blo: '0.283', bhi: '0.375', slo: '0.283', shi: '0.483' };
const CONWAY = { code: '7', side: 3, level: 1, blo: '0.375', bhi: '0.375', slo: '0.25', shi: '0.375' };

const PRESETS = [['Bugs', BUGS], ['Conway', CONWAY]];

const FIRST = { code: '7', side: 3, level: 3, ...BUGS, size: 256, steps: 64, density: 0.45, soup: 7 };

const attempt = (fn) => {
  try {
    return { read: fn(), error: null };
  } catch (error) {
    return { read: null, error };
  }
};

const alive = (types) => types.reduce((a, b) => a + b, 0);

const share = (text) => {
  const value = String(text).trim() === '' ? NaN : Number(text);
  return Number.isFinite(value) ? value : NaN;
};

const glow = (spectrum, size) => m.paint_span(spectrum, size, 0, 1, 'fire', SHADES, false);

// THE PROFILE

const rings = (still, mask) => (canvas) => {
  const [ctx, w, h] = fit(canvas, 160);
  ctx.clearRect(0, 0, w, h);
  const left = 8, right = w - 8, top = 16, floor = h - 18;
  const span = Math.max(still.profile.length, mask.profile.length) - 1;
  const x = (k) => left + (k / span) * (right - left);
  const curve = (profile, hue, width) => {
    const peak = Math.max(1e-9, ...profile.slice(1));
    ctx.strokeStyle = hue;
    ctx.lineWidth = width;
    ctx.beginPath();
    profile.forEach((v, k) => {
      if (k === 0) return;
      const y = floor - (Math.min(v, peak) / peak) * (floor - top);
      if (k === 1) ctx.moveTo(x(k), y);
      else ctx.lineTo(x(k), y);
    });
    ctx.stroke();
  };
  curve(mask.profile, ink.yellow, 1);
  curve(still.profile, ink.blue, 1.5);
  if (still.peak_ring > 0) {
    ctx.strokeStyle = ink.pink;
    ctx.lineWidth = 1;
    ctx.setLineDash([3, 3]);
    ctx.beginPath();
    ctx.moveTo(x(still.peak_ring), top - 6);
    ctx.lineTo(x(still.peak_ring), floor);
    ctx.stroke();
    ctx.setLineDash([]);
  }
  ctx.fillStyle = ink.dim;
  ctx.font = '11px ui-monospace, monospace';
  ctx.fillText('ring 1', left, h - 5);
  ctx.textAlign = 'right';
  ctx.fillText(`ring ${span}`, right, h - 5);
  ctx.textAlign = 'left';
  ctx.fillStyle = ink.pink;
  if (still.peak_ring > 0) ctx.fillText(`k = ${still.peak_ring}`, Math.min(x(still.peak_ring) + 4, right - 40), top - 6);
};

function App() {
  const taps = useSeeds();
  const [pick, set] = useQuery(FIRST);
  const [world, setWorld] = useState(null);

  const code = pick.code.trim();
  const size = SIZES.includes(pick.size) ? pick.size : FIRST.size;
  const cap = Math.min(DEEPEST, m.level_cap(pick.side, 1, size));
  const level = Math.max(1, Math.min(pick.level, cap));
  const windows = [share(pick.blo), share(pick.bhi), share(pick.slo), share(pick.shi)];
  const unread = windows.some((v) => Number.isNaN(v)) ? new Error('every window edge is a fraction of the mask budget, a number between 0 and 1.') : null;

  const kernel = useMemo(() => attempt(() => m.chladni_kernel(code, pick.side, level, size)), [code, pick.side, level, size]);
  const budget = kernel.read ? alive(kernel.read.types) : 0;
  const maskSpectrum = useMemo(() => kernel.read && glow(m.chladni_spectrum(kernel.read.types, size), size), [kernel.read]);
  const maskProfile = useMemo(() => kernel.read && JSON.parse(m.chladni_profile(kernel.read.types, size)), [kernel.read]);

  const run = () => {
    if (unread) return setWorld({ types: null, age: 0, error: unread });
    const made = attempt(() => m.chladni_run(code, pick.side, level, ...windows, size, pick.steps, pick.density, pick.soup));
    setWorld({ types: made.read ? made.read.types : null, age: 0, error: made.error });
  };

  const step = () => {
    if (!world?.types || unread) return;
    const made = attempt(() => m.chladni_next(world.types, size, code, pick.side, level, ...windows));
    setWorld(made.read ? { types: made.read, age: world.age + 1, error: null } : { ...world, error: made.error });
  };

  const key = [code, pick.side, level, ...windows, size, pick.steps, pick.density, pick.soup].join(' ');
  useEffect(() => {
    const timer = setTimeout(run, PAUSE);
    return () => clearTimeout(timer);
  }, [key]);

  const still = useMemo(() => (world?.types ? { width: size, height: size, types: world.types } : null), [world]);
  const stillSpectrum = useMemo(() => still && glow(m.chladni_spectrum(still.types, size), size), [still]);
  const stillProfile = useMemo(() => still && JSON.parse(m.chladni_profile(still.types, size)), [still]);
  const live = still ? alive(still.types) : 0;

  const wears = (values) => Object.entries(values).every(([k, v]) => (k === 'level' ? level : pick[k]) === v);

  const counts = (lo, hi) => {
    if (!budget || Number.isNaN(lo) || Number.isNaN(hi)) return 'none';
    const from = Math.ceil(lo * budget - 1e-9), to = Math.floor(hi * budget + 1e-9);
    return from > to ? 'none' : from === to ? `${from}` : `${from} to ${to}`;
  };

  const controls = (
    <>
      <Group name="Run">
        <Btn primary onClick={run}>Run</Btn>
        <Btn onClick={step}>Step</Btn>
        <Pick label="size" value={size} options={SIZES.map((s) => [s, s])} onChange={(v) => set({ size: +v, level: Math.min(level, Math.min(DEEPEST, m.level_cap(pick.side, 1, +v))) })} />
        <Slider label="steps" value={pick.steps} min={1} max={128} onChange={(v) => set({ steps: v })} />
        <Slider label="density" value={pick.density} min={0.05} max={0.95} step={0.01} onChange={(v) => set({ density: v })} />
        <Slider label="soup" value={pick.soup} min={1} max={99} onChange={(v) => set({ soup: v })} />
      </Group>
      <Group name="The mask">
        <Picker dimension={2} code={pick.code} seeds={taps} onChange={(patch) => set(patch)} />
        <Pick label="side" value={pick.side} options={SIDES.map((s) => [s, s])} onChange={(v) => set({ side: +v, level: Math.min(level, Math.min(DEEPEST, m.level_cap(+v, 1, size))) })} />
        <Pick label="level" value={level} options={Array.from({ length: cap }, (_, i) => [i + 1, i + 1])} onChange={(v) => set({ level: +v })} />
      </Group>
      <Group name="The rule">
        <Text label="birth from" value={pick.blo} onChange={(v) => set({ blo: v })} />
        <Text label="birth to" value={pick.bhi} onChange={(v) => set({ bhi: v })} />
        <Text label="survive from" value={pick.slo} onChange={(v) => set({ slo: v })} />
        <Text label="survive to" value={pick.shi} onChange={(v) => set({ shi: v })} />
        {PRESETS.map(([label, values]) => <Btn key={label} on={wears(values)} onClick={() => set(values)}>{label}</Btn>)}
      </Group>
    </>
  );

  return (
    <Page crumb="chladni" title="chladni"
      sub="A soup run under a Larger-than-Life rule on a big design mask settles into a still with a grain of its own. The mask's spectrum and the still's are drawn side by side, and the ring where the still's spectrum peaks reads the wavelength the rule prefers."
      foot={<>The mask is a design at side n and level L with its centre popped, m cells in all; the rule is two closed windows on the fraction count / m: a dead cell is born inside the birth window, a live cell is kept inside the survive window, and the count is the mask laid on the torus, read by FFT convolution. Bugs is Evans' rule on the radius-5 box, birth 34 to 45 and survive 34 to 58 of 120, kept here as fractions so it moves to any mask; Conway is 3/8 3/8 2/8 3/8 on the Moore mask, the level-1 carpet. The spectrum is the log magnitude of the field's transform with the zero frequency at the centre, the profile its mean over rings of radius k, and ring k on a torus of side N is the wavelength N / k. The same masks and counts run without the fractions on <a href="../mrlylife">the mrlylife page</a>, and the mask's own modes are drawn on <a href="../modes">the modes page</a>. The research page is <a href="/research/automata/">automata</a>.</>}
      controls={controls}>

      <div className="arena">
        <div className="panel">
          <h2>the mask <span>a design, centre popped, on the torus</span></h2>
          <Note error={kernel.error} />
          {kernel.read && <Grid grid={kernel.read} on={ink.yellow} aria-label="The neighbourhood mask centred on the torus" />}
          <Stats>
            <Stat label="mask">{`code ${code} side ${pick.side} level ${level}`}</Stat>
            <Stat label="span">{kernel.read ? pick.side ** level : 0}</Stat>
            <Stat label="cells">{budget}</Stat>
            <Stat label="birth">{counts(windows[0], windows[1])}</Stat>
            <Stat label="survive">{counts(windows[2], windows[3])}</Stat>
          </Stats>
        </div>
        <div className="panel">
          <h2>the still <span>{`${size} by ${size}, wrapped`}</span></h2>
          <Note error={world?.error ?? unread} />
          {still && <Grid grid={still} on={ink.green} aria-label="The soup after the run" />}
          <Stats>
            <Stat label="generation">{world ? pick.steps + world.age : 0}</Stat>
            <Stat label="live">{live}</Stat>
            <Stat label="share"><span className="num">{(live / (size * size)).toFixed(3)}</span></Stat>
          </Stats>
        </div>
      </div>

      <div className="arena">
        <div className="panel">
          <h2>the mask's spectrum <span>log magnitude, DC at the centre</span></h2>
          {maskSpectrum && <Pixels data={maskSpectrum} role="img" aria-label="The log spectrum of the mask" />}
          <Stats>
            <Stat label="peak ring">{maskProfile ? maskProfile.peak_ring : 0}</Stat>
            <Stat label="wavelength"><span className="num">{maskProfile ? maskProfile.wavelength.toFixed(2) : '0.00'}</span></Stat>
          </Stats>
        </div>
        <div className="panel">
          <h2>the still's spectrum <span>the same transform of the still</span></h2>
          {stillSpectrum && <Pixels data={stillSpectrum} role="img" aria-label="The log spectrum of the still" />}
          <Stats>
            <Stat label="peak ring">{stillProfile ? stillProfile.peak_ring : 0}</Stat>
            <Stat label="wavelength"><span className="num">{stillProfile ? stillProfile.wavelength.toFixed(2) : '0.00'}</span></Stat>
          </Stats>
        </div>
      </div>

      <div className="panel">
        <h2>the ring profile <span>mean log magnitude at radius k, the still in blue, the mask in yellow</span></h2>
        {stillProfile && maskProfile && <Sketch draw={rings(stillProfile, maskProfile)} deps={[stillProfile, maskProfile]} className="bars" aria-label="The ring profile of both spectra with the peak ring marked" />}
        <Stats>
          <Stat label="peak ring">{stillProfile ? stillProfile.peak_ring : 0}</Stat>
          <Stat label="wavelength"><span className="num">{stillProfile ? `${stillProfile.wavelength.toFixed(2)} cells` : 'none'}</span></Stat>
        </Stats>
        <p className="sub">Each curve is scaled to its own peak past ring 0, so the two are read for shape, not height. The dashed line is the still's peak ring; on an empty field the profile is flat and the tie goes to ring 1, so a dead soup reads wavelength {size}. Every value on this page is a link: the code, the side, the level, the four window edges, the size, the steps, the density and the soup all live in the address bar.</p>
      </div>
    </Page>
  );
}

mount(<App />);
