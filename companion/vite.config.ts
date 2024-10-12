import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    lib: {
      entry: "src/main.jsx", // Your main entry file
      // name: "Companion", // Global variable name for UMD/IIFE builds
      formats: ["es"], // Output as ES module
      fileName: "companion", // Output file name
    },
    rollupOptions: {
      // If you want to include all dependencies in the bundle
      external: [], // Empty array to bundle all dependencies
      // Or specify external dependencies to exclude them from the bundle
      // external: ['react', 'react-dom'],
    },
  },
  define: {
    "process.env.NODE_ENV": JSON.stringify("production"),
  },
});
