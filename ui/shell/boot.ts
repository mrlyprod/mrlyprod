(() => {
  const d = document.documentElement;
  const home = ["/", "/index.html"].includes(location.pathname);
  d.classList.add("js");
  const p = d.dataset.prefix || "";
  try {
    for (const k of ["theme", "tint", "text", "motion", "contrast", "targets"]) {
      const v = localStorage.getItem(`${p}${k}`);
      if (v) d.dataset[k] = v;
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
    const password = localStorage.getItem(`${p}password`);
    if (password && !sessionStorage.getItem(`${p}unlocked`)) d.dataset.sealed = "";
    if (home) d.dataset.welcome = password || localStorage.getItem(`${p}welcome`) ? "shut" : "open";
  } catch {}
  let table: string[][] = [];
  let picked = "";
  try {
    table = JSON.parse(d.dataset.designs || "[]");
    picked = localStorage.getItem(`${p}design`) || "";
  } catch {}
  const row = table.find((one) => one[0] === picked) || table[0];
  if (row) {
    d.dataset.design = row[0];
    const knob = `${p}knob-${row[0]}-`;
    try {
      for (let i = 0; i < localStorage.length; i++) {
        const k = localStorage.key(i) || "";
        const v = k.startsWith(knob) && localStorage.getItem(k);
        if (v) d.dataset[k.slice(knob.length)] = v;
      }
    } catch {}
    document.write(`<link rel="stylesheet" href="${row[1]}">`);
    for (const meta of document.querySelectorAll<HTMLMetaElement>('meta[name="theme-color"]'))
      meta.content = row[meta.media.includes("dark") ? 3 : 2];
  }
  if (home) {
    d.dataset.desk = "";
    d.dataset.window = "none";
  }
})();
