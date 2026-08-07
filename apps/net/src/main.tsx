import { createRoot } from "react-dom/client"
import "mrlyui/mrly.css"
import { App } from "./App"

document.documentElement.classList.add("js")

const root = document.getElementById("root")
if (root !== null) createRoot(root).render(<App />)
