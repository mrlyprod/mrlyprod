import { ready, $, ink, out } from '../lib/mrly.js';
import { board, axis, keep } from '../lib/chart.js';
import { seeds, roll } from '../lib/select.js';

const m = await ready();
const s = seeds();

const draw = keep(() => {
  const q = +$('q').value;
  out('q', q);
  const nodes = JSON.parse(m.farey(q));
  const stack = JSON.parse(m.farey_novelty(q));
  const b = board($('bars'), 220, { pad: 24, top: 12, bottom: 30 });
  const { ctx } = b;
  axis(b, [[0, '0'], [1, '1']]);
  for (const [num, den, bright] of nodes) {
    const x = b.x(num / den);
    ctx.strokeStyle = `rgba(232, 236, 241, ${0.14 + 0.7 * bright / q})`;
    ctx.lineWidth = bright > q / 3 ? 1.5 : 0.7;
    ctx.beginPath();
    ctx.moveTo(x, b.floor);
    ctx.lineTo(x, b.y(bright / q));
    ctx.stroke();
  }
  if ($('primes').checked) {
    ctx.fillStyle = ink.orange;
    for (const [num, den] of nodes) {
      if (stack.primes.includes(den)) ctx.fillRect(b.x(num / den) - 1, b.floor, 2, 7);
    }
  }
  $('out').textContent = `scales 1..${q}   lit nodes ${stack.lit}   1 + sum phi(n) = ${stack.novel}   match ${stack.match ? 'yes' : 'no'}\nprimes found as maximal-novelty scales: ${stack.primes.join(' ')}`;
});

$('q').oninput = draw;
$('primes').onchange = draw;
$('random').onclick = () => {
  roll(s.next(), ['q']);
  draw();
};
if (s.get()) roll(s.get(), ['q']);
draw();
