import { ready, $, ink, fit } from './mrly.js';

const m = await ready();

function draw() {
  const q = +$('q').value;
  $('q-out').textContent = q;
  const nodes = JSON.parse(m.farey(q));
  const phi = m.totients(q);
  const primes = [];
  let novel = 0;
  for (let n = 1; n <= q; n++) {
    novel += phi[n];
    if (n > 1 && phi[n] === n - 1) primes.push(n);
  }
  const canvas = $('bars');
  const [ctx, w, h] = fit(canvas, 220);
  ctx.clearRect(0, 0, w, h);
  const pad = 24, span = w - 2 * pad, floor = h - 30;
  ctx.strokeStyle = ink.line;
  ctx.beginPath();
  ctx.moveTo(pad, floor);
  ctx.lineTo(w - pad, floor);
  ctx.stroke();
  for (const [num, den, bright] of nodes) {
    const x = pad + span * num / den;
    ctx.strokeStyle = `rgba(232, 236, 241, ${0.14 + 0.7 * bright / q})`;
    ctx.lineWidth = bright > q / 3 ? 1.5 : 0.7;
    ctx.beginPath();
    ctx.moveTo(x, floor);
    ctx.lineTo(x, floor - (floor - 12) * bright / q);
    ctx.stroke();
  }
  if ($('primes').checked) {
    ctx.fillStyle = ink.orange;
    for (const [num, den] of nodes) {
      if (primes.includes(den)) ctx.fillRect(pad + span * num / den - 1, floor, 2, 7);
    }
  }
  ctx.fillStyle = ink.dim;
  ctx.font = `11px ${getComputedStyle(document.body).getPropertyValue('--mono')}`;
  ctx.fillText('0', pad - 3, h - 8);
  ctx.fillText('1', w - pad - 3, h - 8);
  $('out').textContent = `scales 1..${q}   lit nodes ${nodes.length}   1 + sum phi(n) = ${1 + novel}   match ${nodes.length === 1 + novel ? 'yes' : 'no'}\nprimes found as maximal-novelty scales: ${primes.join(' ')}`;
}

$('q').oninput = draw;
$('primes').onchange = draw;
addEventListener('resize', draw);
draw();
