/* KEYS */

const WORDS = { ' ': 'Space', Spacebar: 'Space', ArrowLeft: 'Left', ArrowRight: 'Right', ArrowUp: 'Up', ArrowDown: 'Down', Escape: 'Esc', Esc: 'Esc' };

export function label(key) {
  return WORDS[key] ?? (key?.length === 1 ? key.toUpperCase() : key);
}

/* MENUS */

const own = (rows, name) => rows.filter((row) => row.menu === name).flatMap((row) => {
  const keys = [].concat(row.key);
  return keys.length > 1 && keys.every((key) => key.startsWith('Arrow')) ? keys.map((key) => ({ label: `${row.label} ${label(key)}`, act: `${row.act}@${key}`, keys: [key] })) : [{ label: row.label, act: row.act, keys: keys.slice(0, 1) }];
});

const ruled = (...parts) => parts.filter((part) => part.length).flatMap((part, n) => (n ? [null, ...part] : part));

export function menus({ exports = [], undo = false, redo = false, reset = false, view = null, panes = {}, keys = [], told = false }) {
  const file = ruled([{ label: 'Export...', act: 'export', keys: ['e'] }, ...exports.map((label, n) => ({ label: `Export ${label}`, act: `export:${n}` }))], [{ label: 'Copy Link', act: 'copy' }], own(keys, 'File'));
  const edit = ruled([{ label: 'Undo', act: 'undo', keys: ['mod', 'z'], off: !undo }, { label: 'Redo', act: 'redo', keys: ['mod', 'shift', 'z'], off: !redo }], [{ label: 'Reroll', act: 'reroll', keys: ['r'] }, ...(reset ? [{ label: 'Reset Knobs', act: 'reset' }] : [])], own(keys, 'Edit'));
  const sides = [['left', 'Library', '['], ['right', 'Knobs', ']']].filter(([side]) => side in panes).map(([side, label, key]) => ({ label: `Show ${label}`, act: `pane:${side}`, keys: ['mod', key], on: panes[side] }));
  const looks = view ? view.options.map(([v, text]) => ({ label: text, act: `view:${v}`, on: String(v) === String(view.value), radio: true })) : [];
  const shown = ruled(looks, sides, [{ label: 'Zen', act: 'zen', keys: ['z'] }], own(keys, 'View'));
  const help = ruled([{ label: 'Keys', act: 'keys', keys: ['?'] }, ...(told ? [{ label: 'Info', act: 'info', keys: ['i'] }] : [])], own(keys, 'Help'));
  return [
    { name: 'File', items: file },
    { name: 'Edit', items: edit },
    { name: 'View', items: shown },
    { name: 'Help', items: help },
  ];
}

export function publish(model) {
  if (typeof dispatchEvent === 'function' && typeof CustomEvent === 'function') dispatchEvent(new CustomEvent('menus', { detail: model }));
}
