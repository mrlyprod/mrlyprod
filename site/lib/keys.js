const KEPT = '.controls, textarea, select, [contenteditable]:not([contenteditable=false]), input:not([type=range], [type=checkbox], [type=radio], [type=button], [type=submit], [type=reset])';

const PRESSED = 'button, a[href], summary, [role=button], [role=link]';

const WORDS = { ' ': 'Space', Spacebar: 'Space', ArrowLeft: 'Left', ArrowRight: 'Right', ArrowUp: 'Up', ArrowDown: 'Down', Escape: 'Esc', Esc: 'Esc' };

const tag = (key) => (key === ' ' || key === 'Spacebar' ? 'Space' : key === 'Esc' ? 'Escape' : key?.length === 1 ? key.toLowerCase() : key);

const within = (e, selector) => typeof e.target?.closest === 'function' && !!e.target.closest(selector);

export function hit(map, e, open = () => false) {
  if (e.metaKey || e.ctrlKey || e.altKey) return null;
  const key = tag(e.key);
  const row = map.find((one) => [].concat(one.key).some((k) => tag(k) === key));
  if (!row || within(e, KEPT) || open(e)) return null;
  if ((key === 'Space' || key === 'Enter') && within(e, PRESSED)) return null;
  return row;
}

export function label(key) {
  return WORDS[key] ?? (key?.length === 1 ? key.toUpperCase() : key);
}
