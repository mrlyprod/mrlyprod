const MASK = ['11111', '10101', '11111', '10101', '11111'];

export function grid(level) {
  let rows = MASK;
  for (let k = 1; k < level; k++) {
    rows = rows.flatMap((row) => MASK.map((inner) => [...row].map((cell) => (cell === '1' ? inner : '00000')).join('')));
  }
  return rows;
}

function path(rows) {
  const parts = [];
  rows.forEach((row, y) => {
    let x = 0;
    while (x < row.length) {
      if (row[x] !== '1') {
        x++;
        continue;
      }
      let w = 0;
      while (row[x + w] === '1') w++;
      parts.push(`M${x} ${y}h${w}v1h-${w}z`);
      x += w;
    }
  });
  return parts.join('');
}

export function logoSvg(level = 1, fill = 'currentColor') {
  const rows = grid(level);
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${rows.length} ${rows.length}" role="img" aria-label="MrlyProd"><path fill="${fill}" d="${path(rows)}"/></svg>`;
}
