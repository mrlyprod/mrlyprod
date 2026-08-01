import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [
    react({
      babel: {
        plugins: [["babel-plugin-react-compiler"]],
      },
    }),
  ],
  server: {
    host: true,
  },
  build: {
    outDir: process.env["MRLY_OUT"] ?? "../../data/jsx/dist",
    emptyOutDir: true,
  },
});
