(() => {
  const d = document.documentElement;
  const home = ["/", "/index.html"].includes(location.pathname);
  d.classList.add("js");
  const p = d.dataset.prefix || "";
  const knob = `${p}knob-`;
  try {
    for (const k of ["theme", "tint", "text", "motion", "contrast", "targets"]) {
      const v = localStorage.getItem(`${p}${k}`);
      if (v) d.dataset[k] = v;
    }
    for (let i = 0; i < localStorage.length; i++) {
      const k = localStorage.key(i) || "";
      const v = k.startsWith(knob) && localStorage.getItem(k);
      if (v) d.dataset[k.slice(knob.length)] = v;
    }
    if (localStorage.getItem(`${p}clear`) === "1" || matchMedia("(prefers-reduced-transparency: reduce)").matches)
      d.dataset.clear = "1";
    const icons = localStorage.getItem(`${p}icons`);
    const tone =
      localStorage.getItem(`${p}icons-always`) ||
      (d.dataset.theme ? d.dataset.theme === "dark" : matchMedia("(prefers-color-scheme: dark)").matches)
        ? "dark"
        : "light";
    if (icons && (icons !== "dark" || tone === "dark"))
      d.dataset.icons = icons === "dark" ? "dark" : `${icons}-${tone}`;
    if (home && CSS.supports("animation-timeline", "scroll()"))
      d.dataset.welcome =
        localStorage.getItem(`${p}welcome`) ||
        d.dataset.motion === "reduce" ||
        matchMedia("(prefers-reduced-motion: reduce)").matches
          ? "shut"
          : "open";
  } catch {}
  if (home) {
    d.dataset.desk = "";
    d.dataset.window = "none";
  }
})();
