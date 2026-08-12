import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

export default defineConfig({
  root: "demo",
  plugins: [react()],
  build: {
    outDir: process.env["MRLY_OUT"] ?? "../../../../data/ui/dist",
    emptyOutDir: true,
    assetsInlineLimit: 0,
  },
  server: {
    host: true,
  },
})
