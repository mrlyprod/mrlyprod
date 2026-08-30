import { $, out } from './mrly.js';

export function ramp(host, { levels = 16, on } = {}) {
  host.innerHTML = `
    <label>ramp <select id="ramp"><option>fire</option><option>heat</option><option>diverge</option></select></label>
    <label>levels <input type="range" id="levels" min="2" max="64" value="${levels}"><span class="num" id="levels-out">${levels}</span></label>
    <label><input type="checkbox" id="invert"> invert</label>`;
  if (on) for (const id of ['ramp', 'levels', 'invert']) $(id).oninput = on;
  const read = () => {
    const levels = +$('levels').value;
    out('levels', levels);
    return { ramp: $('ramp').value, levels, invert: $('invert').checked };
  };
  return { read };
}
