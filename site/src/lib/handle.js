/* HANDLE */

const clamp = (value, min, max) => Math.min(max, Math.max(min, value));

export function handle(grip, { get, set, done = () => {}, reset, min = 0, max = () => Infinity, rate = 1, step = 16, label = 'Resize', busy = () => {} }) {
  const top = () => (typeof max === 'function' ? max() : max);
  const show = (value) => {
    grip.setAttribute('aria-valuenow', String(Math.round(value)));
    grip.setAttribute('aria-valuemin', String(min));
    grip.setAttribute('aria-valuemax', String(Math.round(top())));
  };
  const put = (value) => {
    const next = clamp(value, min, top());
    set(next);
    show(next);
    return next;
  };
  grip.setAttribute('role', 'separator');
  grip.setAttribute('aria-orientation', 'vertical');
  grip.setAttribute('aria-label', label);
  grip.tabIndex = 0;
  show(get());
  let from = null;
  const down = (e) => {
    if (e.button !== 0) return;
    e.preventDefault();
    grip.setPointerCapture(e.pointerId);
    from = { x: e.clientX, value: get() };
    busy(true);
  };
  const move = (e) => {
    if (!from) return;
    put(from.value + (e.clientX - from.x) * rate);
  };
  const up = (e) => {
    if (!from) return;
    const last = put(from.value + (e.clientX - from.x) * rate);
    from = null;
    if (grip.hasPointerCapture(e.pointerId)) grip.releasePointerCapture(e.pointerId);
    busy(false);
    done(last);
  };
  const key = (e) => {
    const by = { ArrowLeft: -step, ArrowRight: step }[e.key];
    if (by !== undefined) {
      e.preventDefault();
      done(put(get() + by * Math.sign(rate)));
      return;
    }
    if (e.key === 'Home' || e.key === 'End') {
      e.preventDefault();
      done(put((e.key === 'Home') === rate > 0 ? min : top()));
    }
  };
  const twice = () => {
    const to = typeof reset === 'function' ? reset() : reset;
    if (to === undefined) return;
    done(put(to));
  };
  const look = () => show(get());
  grip.addEventListener('pointerdown', down);
  grip.addEventListener('pointermove', move);
  grip.addEventListener('pointerup', up);
  grip.addEventListener('pointercancel', up);
  grip.addEventListener('keydown', key);
  grip.addEventListener('dblclick', twice);
  grip.addEventListener('focus', look);
  return () => {
    grip.removeEventListener('pointerdown', down);
    grip.removeEventListener('pointermove', move);
    grip.removeEventListener('pointerup', up);
    grip.removeEventListener('pointercancel', up);
    grip.removeEventListener('keydown', key);
    grip.removeEventListener('dblclick', twice);
    grip.removeEventListener('focus', look);
  };
}
