import { createRoot } from "react-dom/client"
import "mrlyui/mrly.css"
import "./eyes.css"
import { App } from "./App"
import { boot } from "./boot"

const root = document.getElementById("root")
if (root !== null) {
  const face = await boot()
  createRoot(root).render(<App face={face} />)
}
