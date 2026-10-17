const LOAD = { font: () => import('./font.js'), life: () => import('./life.js'), math: () => import('./math.js'), num: () => import('./num.js') };

export const wasm = (...names) => () => Promise.all(names.map((name) => LOAD[name]())).then((mods) => ({ ...Object.fromEntries(names.map((name, n) => [name, mods[n][name]])), ready: Promise.all(mods.map((one) => one.ready)) }));
