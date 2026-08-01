import { fileURLToPath } from "node:url"
import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import { repo } from "./scripts/dev"

const root = fileURLToPath(new URL(".", import.meta.url))

export default defineConfig({
  plugins: [react(), repo(root)],
  build: {
    outDir: process.env["MRLY_OUT"] ?? "../../data/git/dist",
    emptyOutDir: true,
    assetsInlineLimit: 0,
  },
  server: { host: true },
})
