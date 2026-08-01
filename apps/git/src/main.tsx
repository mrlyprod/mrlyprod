import { createRoot } from "react-dom/client"
import "mrlycss/mrly.css"
import "mrlycss/seti.css"
import { App } from "./App"

document.documentElement.classList.add("js")

const root = document.getElementById("root")
if (root !== null) createRoot(root).render(<App />)
