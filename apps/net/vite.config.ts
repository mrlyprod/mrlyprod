import { fileURLToPath } from "node:url"
import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import { content } from "./scripts/dev"

const root = fileURLToPath(new URL(".", import.meta.url))

export default defineConfig({
  plugins: [react(), content(root)],
  build: {
    outDir: process.env["MRLY_OUT"] ?? "../../data/net/dist",
    emptyOutDir: true,
    assetsInlineLimit: 0,
  },
  server: { host: true },
})
