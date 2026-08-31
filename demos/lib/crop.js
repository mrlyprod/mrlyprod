import { $, mrly, out } from './mrly.js';
import { query, stamp } from './query.js';

export function cropper(host, { dimension = 2, on } = {}) {
  const shapes = JSON.parse(mrly.crop_shapes(dimension));
  host.innerHTML = `
    <label>crop <select id="crop"><option value="">off</option>${shapes.map((name) => `<option>${name}</option>`).join('')}</select></label>
    <label>radius <input type="range" id="crop-r" min="1" max="32" value="16"><span class="num" id="crop-r-out">16/32</span></label>
    <label><input type="checkbox" id="crop-anti"> anti</label>`;
  const params = query(['crop', 'crop-r']);
  $('crop-anti').checked = params.get('crop-anti') === '1';
  if (on) for (const id of ['crop', 'crop-r', 'crop-anti']) $(id).oninput = on;
  let last = '';
  const read = () => {
    const shape = $('crop').value, rnum = +$('crop-r').value, anti = $('crop-anti').checked;
    out('crop-r', `${rnum}/32`);
    const active = shape !== '';
    const next = active ? `${shape}:${rnum}:${anti ? 1 : 0}` : '';
    if (next !== last) {
      stamp({ crop: active ? shape : null, 'crop-r': active ? rnum : null, 'crop-anti': active && anti ? 1 : null });
      last = next;
    }
    return { shape, rnum, rden: 32, anti, active };
  };
  return { read };
}
