import { useMemo } from 'react';
import { ready, ink } from '../../lib/mrly.js';
import { mount, Page, Group, Pick, Slider, Text, Btn, Stats, Stat, Note } from '../../lib/app.jsx';
import { Grid } from '../../lib/draw.jsx';
import { Terms } from '../../lib/series.jsx';
import { useQuery } from '../../lib/query.js';

const m = await ready();

const FIRST = { d: 1, k: 2, code: '7', level: 9 };
const SPAN = m.memory_span();
const CAPS = { 1: m.memory_cap(1), 2: m.memory_cap(2) };

const PRESETS = [
  ['7/1/2', 'the golden rule'],
  ['23/1/3', 'the supergolden rule'],
  ['54/1/3', 'the plastic rule'],
  ['127/1/3', 'the tribonacci rule'],
  ['3/1/1', 'the full shift'],
  ['0/1/2', 'the empty rule'],
  ['7/2/1', 'the plane design 7'],
  ['31710/2/2', 'no digit twice'],
];

const DIAL =
  'A design word is a string of digits, one digit a level, and the digit is a corner of the cube: at dim 1 the two ends of an interval, at dim 2 the four corners of a square, read as the corner integer `c = sum_i d_i 2^i` with `x` bit `0` and `y` bit `1`. Today every word is allowed and the digits are independent, so the design is the same tile folded into itself. The memory dial breaks that independence: a rule names which windows of `k` consecutive digits may stand side by side, and a word survives only when every one of its windows is allowed. At `k = 1` the window is one digit, the rule is a plain digit set, and the accepted words are the cells of `bang dim <dim>, code <code>` at that level, cell for cell. Turn `k` past one and the design remembers what it just wrote.';

const CODE =
  'The alphabet is the `2^dim` corner digits. A window is `k` of them, `(c_1, ..., c_k)`, read as the integer `w = sum_j c_j 2^(dim (k - j))` with the first digit most significant, and bit `w` of the code says whether that window may stand. So a rule of width `k` at dim `dim` is one number below `2^(2^(k dim))`, and the span `k dim <= ' + SPAN + '` keeps it inside a 64-bit code. Click a window below to forbid it or let it back in.';

const GROWTH =
  'Accepted words of length `level` are the walks of length `level - k + 1` on the transfer matrix `A`, whose states are the `2^((k - 1) dim)` windows one digit short, so `N_W(level) = 1^T A^(level - k + 1) 1` and for `level < k` every word counts, `2^(dim level)` of them. The count grows like the Perron root `rho` of `A`, and the growth exponent is `log_2 rho`. The memory number `kappa = log_2(card W) / k - log_2 rho`, for `card W` the allowed windows, is the bits per digit a rule spends on memory, zero on every memoryless design. A word of length `m k` cuts into `m` windows that never meet, so `rho^k <= card W` and `kappa >= 0`; a rule whose windows are a digit set repeated `k` times reaches that bound and spends nothing, and the golden rule spends about a tenth of a bit a digit.';

const attempt = (fn) => {
  try {
    return { ...fn(), error: null };
  } catch (error) {
    return { error };
  }
};

const memoryless = (card) => Number.isFinite(card.kappa) && Math.abs(card.kappa) < 1e-9;

const digits = (w, d, k) => Array.from({ length: k }, (_, j) => (w >> (d * (k - 1 - j))) & ((1 << d) - 1));

const stack = (sheet, level) => {
  const src = sheet.types;
  const band = Math.max(1, Math.round(sheet.width / (2 * level)));
  const types = new Uint8Array(sheet.width * level * band);
  for (let row = 0; row < level * band; row++) {
    const from = Math.floor(row / band) * sheet.width;
    types.set(src.subarray(from, from + sheet.width), row * sheet.width);
  }
  return { width: sheet.width, height: level * band, types };
};

function App() {
  const [pick, set] = useQuery(FIRST);

  const d = pick.d === 2 ? 2 : 1;
  const k = Math.max(1, Math.min(Math.floor(SPAN / d), Math.round(pick.k) || 1));
  const cap = CAPS[d];
  const level = Math.max(1, Math.min(cap, Math.round(pick.level) || 1));
  const code = /^\d+$/.test(pick.code) ? pick.code : '0';

  const read = useMemo(() => attempt(() => ({ read: JSON.parse(m.memory_read(d, k, code, level)) })), [d, k, code, level]);
  const drawn = useMemo(() => attempt(() => ({ sheet: m.memory_sheet(d, k, code, level) })), [d, k, code, level]);

  const card = read.read;
  const toggle = (w) => set({ code: (BigInt(code) ^ (1n << BigInt(w))).toString() });
  const preset = (value) => {
    const [c, dim, width] = value.split('/');
    set({ code: c, d: +dim, k: +width, level: Math.min(CAPS[+dim], level) });
  };

  const controls = (
    <>
      <Group name="The rule">
        <Pick label="preset" value="" options={[['', 'pick one'], ...PRESETS]} onChange={preset} />
        <Pick label="dimension" value={String(d)} options={[['1', '1, the interval'], ['2', '2, the square']]}
          onChange={(value) => set({ d: +value, code: '0', k: 1, level: Math.min(CAPS[+value], level) })} />
        <Slider label="width k" value={k} min={1} max={Math.floor(SPAN / d)} onChange={(value) => set({ k: value, code: '0' })} />
      </Group>
      <Group name="The code">
        <Text label="code" wide value={code} onChange={(value) => set({ code: value.replace(/\D/g, '') || '0' })} />
        <Btn onClick={() => set({ code: card ? (BigInt(card.codes) - 1n).toString() : '0' })}>Allow all</Btn>
        <Btn onClick={() => set({ code: '0' })}>Forbid all</Btn>
      </Group>
      <Group name="The depth">
        <Slider label={`level 1 to ${cap}`} value={level} min={1} max={cap} onChange={(value) => set({ level: value })} />
      </Group>
    </>
  );

  const sheet = drawn.sheet;

  return (
    <Page crumb="memory" title="The memory dial"
      sub="A design accepts every word of digits it can write. Give it a memory of the last k digits and it accepts fewer: the cells thin out, the count stops doubling, and the growth exponent slides off the dimension. At width one the dial is off and the picture is the ordinary design of the same code."
      controls={controls}
      foot={<>Every count, root and exponent on this page comes from the crates through wasm; the page only draws. The sheet is capped at 2^16 sites, so the level runs to {CAPS[1]} in one dimension and to {CAPS[2]} in two. Nearby: <a href="../universe">the universe</a> is the gallery of the codes this dial reads at width one, <a href="../words">words</a> changes the rule level by level instead of digit by digit, <a href="../wolfram">the rules</a> reads the same byte as a cellular automaton.</>}>

      <div className="panel">
        <h2>the accepted words <span>{d === 1 ? 'one row a level, the intervals a word still reaches' : `the ${1 << level} by ${1 << level} grid of level ${level}`}</span></h2>
        <Note error={drawn.error ?? read.error} />
        {sheet ? (
          <Grid grid={d === 1 ? stack(sheet, level) : sheet} on={ink.yellow}
            style={d === 1 ? { height: 'clamp(200px, 34vw, 420px)' } : undefined}
            role="img" aria-label={`the words of width ${k} rule ${code} at dimension ${d}, level ${level}`} />
        ) : null}
        {card ? (
          <Stats>
            <Stat label="rule">{`dim ${card.dimension}, k ${card.width}, code ${card.code}`}</Stat>
            <Stat label="level">{level}</Stat>
            <Stat label="accepted words">{card.counts[level - 1]}</Stat>
            <Stat label="every word">{card.letters ** level}</Stat>
            {k === 1 ? <span className="chip verified">width one, so this sheet is the design bang dim {card.dimension}, code {card.code} at level {level}, cell for cell</span> : null}
          </Stats>
        ) : null}
        <p className="sub">{DIAL}</p>
      </div>

      <div className="arena">
        <div className="panel">
          <h2>the count a level <span>N_W(level) for level one to {level}</span></h2>
          {card ? <Terms terms={card.counts} start={1} /> : null}
          {card ? (
            <Stats>
              <Stat label="Perron root">{card.perron.toFixed(9)}</Stat>
              <Stat label="log_2 rho">{Number.isFinite(card.exponent) ? card.exponent.toFixed(9) : 'none'}</Stat>
              <Stat label="kappa">{Number.isFinite(card.kappa) ? card.kappa.toFixed(9) : 'all of it'}</Stat>
              <Stat label="allowed windows">{card.window_count}</Stat>
              <Stat label="states">{card.states}</Stat>
              <Stat label="alphabet">{card.alphabet.length ? card.alphabet.join(', ') : 'empty'}</Stat>
              <span className={`chip ${memoryless(card) ? 'verified' : 'proved'}`}>
                {memoryless(card) ? 'a memoryless rule, no bits spent on memory' : 'the windows spend this rule bits'}
              </span>
            </Stats>
          ) : null}
          <p className="sub">{GROWTH}</p>
        </div>

        <div className="panel">
          <h2>the windows <span>{card ? `${card.windows} of them, ${card.windows} bits of code below ${card.codes}` : 'the dial itself'}</span></h2>
          {card ? (
            <div className="ribbon tight">
              {card.allowed.map((on, w) => (
                <span key={w} role="button" tabIndex={0} className={on ? 'yellow' : undefined}
                  onClick={() => toggle(w)} onKeyDown={(event) => { if (event.key === 'Enter') toggle(w); }}>
                  <b>{digits(w, d, k).join('·')}</b>
                </span>
              ))}
            </div>
          ) : null}
          <p className="sub">{CODE}</p>
        </div>
      </div>
    </Page>
  );
}

mount(<App />);
