(() => {
  const d = document.documentElement;
  const home = ["/", "/index.html"].includes(location.pathname);
  d.classList.add("js");
  try {
    for (const k of ["theme", "font", "tint", "glass", "text", "motion", "contrast", "targets"]) {
      const v = localStorage.getItem(`mrly-${k}`);
      if (v) d.dataset[k] = v;
    }
    if (localStorage.getItem("mrly-clear") === "1" || matchMedia("(prefers-reduced-transparency: reduce)").matches)
      d.dataset.glass = "off";
    const icons = localStorage.getItem("mrly-icons");
    const tone =
      localStorage.getItem("mrly-icons-always") ||
      (d.dataset.theme ? d.dataset.theme === "dark" : matchMedia("(prefers-color-scheme: dark)").matches)
        ? "dark"
        : "light";
    if (icons && (icons !== "dark" || tone === "dark"))
      d.dataset.icons = icons === "dark" ? "dark" : `${icons}-${tone}`;
    for (const k of ["page", "wide"]) {
      const v = Number(localStorage.getItem(`mrly-${k}`));
      if (v > 0) d.style.setProperty(k === "page" ? "--win" : "--wide", `${v}px`);
    }
    if (home && CSS.supports("animation-timeline", "scroll()"))
      d.dataset.welcome =
        localStorage.getItem("mrly-welcome") ||
        d.dataset.motion === "reduce" ||
        matchMedia("(prefers-reduced-motion: reduce)").matches
          ? "shut"
          : "open";
  } catch {}
  if (home) {
    d.dataset.desk = "";
    d.dataset.window = "none";
  }
  const forms = ["(pointer: coarse) and (max-width: 44rem)", "(max-width: 40rem)"].map((one) => matchMedia(one));
  const form = () => {
    d.dataset.form = forms.some((one) => one.matches) ? "phone" : "desk";
  };
  form();
  for (const one of forms) one.addEventListener("change", form);
})();
