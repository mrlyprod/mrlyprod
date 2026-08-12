import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: process.env["MRLY_OUT"] ?? "../../data/bot/dist",
    emptyOutDir: true,
    assetsInlineLimit: 0,
  },
  server: { host: true },
})
