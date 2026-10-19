/* STORAGE */

export const PREFIX = (typeof document !== 'undefined' && document.documentElement.dataset.prefix) || 'mrly-';
export const KEY = { theme: `${PREFIX}theme`, font: `${PREFIX}font`, tint: `${PREFIX}tint`, saver: `${PREFIX}saver`, cart: `${PREFIX}cart`, welcome: `${PREFIX}welcome` };

export const read = (key) => {
  try {
    return localStorage.getItem(key) ?? '';
  } catch {
    return '';
  }
};

export const write = (key, value) => {
  try {
    if (value) localStorage.setItem(key, value);
    else localStorage.removeItem(key);
  } catch {}
};

export const root = () => document.documentElement;

/* THEME */

let painted = null;

function repaint() {
  const now = `${root().dataset.theme ?? ''} ${root().dataset.tint ?? ''}`;
  if (painted === null) painted = now;
  if (now === painted) return;
  painted = now;
  window.dispatchEvent(new Event('theme'));
}

export function theme(next) {
  if (next) root().dataset.theme = next;
  else delete root().dataset.theme;
  write(KEY.theme, next);
  for (const label of document.querySelectorAll('[data-theme-toggle] b')) label.textContent = next || 'auto';
  repaint();
}

/* TINT */

export function tint(next) {
  if (next) root().dataset.tint = next;
  else delete root().dataset.tint;
  write(KEY.tint, next);
  for (const pick of document.querySelectorAll('[data-tint-pick]')) pick.value = next || '';
  repaint();
}

export function turn() {
  const now = root().dataset.theme ?? '';
  theme(now === '' ? 'light' : now === 'light' ? 'dark' : '');
}

/* FONT */

export function face(next) {
  if (next) root().dataset.font = next;
  else delete root().dataset.font;
  write(KEY.font, next);
  for (const pick of document.querySelectorAll('[data-font-pick]')) pick.value = next || '';
}

/* SAVER */

export function screen(next) {
  write(KEY.saver, next);
  const head = next.split('?')[0];
  for (const pick of document.querySelectorAll('[data-saver-pick]')) {
    const known = [...pick.options].some((one) => one.value === head);
    pick.value = known ? head : head ? 'random' : ([...pick.options].find((one) => one.defaultSelected)?.value ?? 'random');
  }
}
