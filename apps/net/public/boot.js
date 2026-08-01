try {
  var held = localStorage.getItem("mrly-theme")
  if (held) document.documentElement.dataset.theme = held
  if (localStorage.getItem("mrly-font")) document.documentElement.classList.add("mrlyfont")
} catch (e) {}
document.documentElement.classList.add("js")
