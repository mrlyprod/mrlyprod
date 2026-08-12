try {
  const root = document.documentElement
  const set = (name, value) => {
    if (value !== "") root.style.setProperty(name, value)
  }
  const theme = localStorage.getItem("mrly-theme")
  if (theme === "light" || theme === "dark") root.dataset.theme = theme
  if (localStorage.getItem("mrly-font")) root.classList.add("mrlyfont")
  const held = localStorage.getItem("mrly-prefs")
  if (held) {
    const prefs = JSON.parse(held)
    set("--unit", !prefs.unit ? "" : prefs.unit + "px")
    set("--border-width", prefs.border === undefined || prefs.border === 1 ? "" : prefs.border + "px")
    set("--radius", prefs.radius === undefined || prefs.radius === 2 ? "" : "calc(var(--unit) * " + prefs.radius + ")")
    set("--accent-color", !prefs.accent ? "" : "var(--c-" + prefs.accent + ")")
    set("--background-color", !prefs.background ? "" : "var(--c-" + prefs.background + ")")
    set("--border-color", !prefs.line ? "" : "var(--c-" + prefs.line + ")")
  }
} catch (e) {}
document.documentElement.classList.add("js")
