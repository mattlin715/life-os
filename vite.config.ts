import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(({ mode }) => ({
  plugins: [react()],
  define: {
    "import.meta.env.VITE_LIFE_OS_ANDROID_FEASIBILITY_M0": JSON.stringify(mode === "android-m0" ? "1" : "0"),
    "import.meta.env.VITE_LIFE_OS_ANDROID_M1": JSON.stringify(mode === "android-m1" ? "1" : "0"),
    "import.meta.env.VITE_LIFE_OS_ANDROID_M1_DEBUG_HOOKS": JSON.stringify(mode === "android-m1" ? "1" : "0"),
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/.tools/**", "**/src-tauri/target/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2020",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}));
