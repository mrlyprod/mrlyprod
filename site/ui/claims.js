const live = new Map();

export function mount(form) {
  if (live.has(form)) return;
  const chips = [...form.querySelectorAll('button')];
  const since = form.querySelector('input');
  const count = form.querySelector('output');
  const sections = [...form.parentElement.querySelectorAll('section.claims')];
  let tag = chips.find((chip) => chip.classList.contains('on'))?.dataset.tag ?? '';
  const run = () => {
    let total = 0;
    for (const section of sections) {
      let shown = 0;
      for (const item of section.querySelectorAll('li[data-tag]')) {
        const on = (!tag || item.dataset.tag === tag) && (!since.value || item.dataset.date >= since.value);
        item.hidden = !on;
        if (on) shown++;
      }
      section.hidden = !shown;
      total += shown;
    }
    count.textContent = `${total} claims`;
  };
  const gate = new AbortController();
  const { signal } = gate;
  for (const chip of chips) {
    chip.addEventListener('click', () => {
      tag = chip.dataset.tag;
      for (const other of chips) other.classList.toggle('on', other === chip);
      run();
    }, { signal });
  }
  since.addEventListener('input', run, { signal });
  form.addEventListener('submit', (event) => event.preventDefault(), { signal });
  live.set(form, gate);
}

export function unmount(form) {
  live.get(form)?.abort();
  live.delete(form);
}
