import { createRoot } from "react-dom/client"
import "mrlyui/mrly.css"
import { Sink } from "./Sink"

const root = document.getElementById("root")
if (root !== null) createRoot(root).render(<Sink />)
