import FONT from '../kit/font/font.json' with { type: 'json' };

export const WORD = 'MRLYPROD';

function trim(rows) {
  let from = Infinity;
  let to = 0;
  for (const row of rows) {
    for (let c = 0; c < row.length; c++) {
      if (row[c] !== '1') continue;
      from = Math.min(from, c);
      to = Math.max(to, c + 1);
    }
  }
  return rows.map((row) => row.slice(from, to));
}

const lit = (rows, left) => rows.flatMap((row, y) => [...row].flatMap((ch, x) => (ch === '1' ? [[left + x, y]] : [])));

export function welcome(word = WORD) {
  const glyphs = [...word].map((char) => trim(FONT[char].rows));
  const starts = [];
  let col = 0;
  for (const rows of glyphs) {
    starts.push(col);
    col += rows[0].length + 1;
  }
  const cols = col - 1;
  const n = glyphs.length;
  const half = n >> 1;
  const target = glyphs.map((rows) => (cols - rows[0].length) >> 1);
  const letters = glyphs.map((rows, i) => {
    const track = [];
    let at = starts[i];
    for (let p = 1; p < half; p++) {
      if (i < p) at = starts[p];
      else if (i >= n - p) at = starts[n - p - 1];
      track.push(at - starts[i]);
    }
    track.push(target[i] - starts[i]);
    return { track, cells: lit(rows, starts[i]) };
  });
  const left = (cols - 5) >> 1;
  const x = lit(FONT.X.rows, left);
  const stacked = new Set(letters.flatMap(({ track, cells }) => cells.map(([cx, cy]) => `${cx + track.at(-1)},${cy}`)));
  const want = new Set(x.map(([cx, cy]) => `${cx},${cy}`));
  if (stacked.size !== want.size || [...want].some((cell) => !stacked.has(cell))) throw new Error(`welcome: ${word} does not fold into the X`);
  if (letters.some(({ track }) => track.length !== 4)) throw new Error(`welcome: ${word} folds in ${letters[0].track.length} beats, the css holds 4`);
  return { cols, left, letters, x };
}

const rect = (x, y, k) => `<rect x="${x}" y="${y}" width="1" height="1" style="--k:${k}"/>`;

export function markup({ left, letters, x } = welcome()) {
  const groups = letters.map(({ track: [a, b, c, d], cells }) => {
    const body = cells.map(([cx, cy]) => rect(cx, cy, cx + cy)).join('');
    return `<g class="l" style="--a:${a};--b:${b};--c:${c};--d:${d}">${body}<g class="glint">${body}</g></g>`;
  });
  const path = x.map(([cx, cy]) => `M${cx} ${cy}h1v1h-1z`).join('');
  const glint = x.map(([cx, cy]) => rect(cx, cy, (cx - left + cy) * 5)).join('');
  return `${groups.join('')}<g class="x"><path d="${path}"/><g class="glint">${glint}</g></g>`;
}
