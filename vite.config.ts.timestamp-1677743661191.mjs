// vite.config.ts
import { defineConfig } from "file:///C:/Users/Vinayak/Desktop/stuff/code/rust/foyer/node_modules/vite/dist/node/index.js";
import { svelte } from "file:///C:/Users/Vinayak/Desktop/stuff/code/rust/foyer/node_modules/@sveltejs/vite-plugin-svelte/dist/index.js";
import sveltePreprocess from "file:///C:/Users/Vinayak/Desktop/stuff/code/rust/foyer/node_modules/svelte-preprocess/dist/index.js";
var vite_config_default = defineConfig({
  plugins: [
    svelte({
      preprocess: [
        sveltePreprocess({
          typescript: true
        })
      ]
    })
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJDOlxcXFxVc2Vyc1xcXFxWaW5heWFrXFxcXERlc2t0b3BcXFxcc3R1ZmZcXFxcY29kZVxcXFxydXN0XFxcXGZveWVyXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCJDOlxcXFxVc2Vyc1xcXFxWaW5heWFrXFxcXERlc2t0b3BcXFxcc3R1ZmZcXFxcY29kZVxcXFxydXN0XFxcXGZveWVyXFxcXHZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9DOi9Vc2Vycy9WaW5heWFrL0Rlc2t0b3Avc3R1ZmYvY29kZS9ydXN0L2ZveWVyL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHsgc3ZlbHRlIH0gZnJvbSBcIkBzdmVsdGVqcy92aXRlLXBsdWdpbi1zdmVsdGVcIjtcclxuaW1wb3J0IHN2ZWx0ZVByZXByb2Nlc3MgZnJvbSBcInN2ZWx0ZS1wcmVwcm9jZXNzXCI7XHJcblxyXG4vLyBodHRwczovL3ZpdGVqcy5kZXYvY29uZmlnL1xyXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xyXG4gICAgcGx1Z2luczogW1xyXG4gICAgICAgIHN2ZWx0ZSh7XHJcbiAgICAgICAgICAgIHByZXByb2Nlc3M6IFtcclxuICAgICAgICAgICAgICAgIHN2ZWx0ZVByZXByb2Nlc3Moe1xyXG4gICAgICAgICAgICAgICAgICAgIHR5cGVzY3JpcHQ6IHRydWUsXHJcbiAgICAgICAgICAgICAgICB9KSxcclxuICAgICAgICAgICAgXSxcclxuICAgICAgICB9KSxcclxuICAgIF0sXHJcblxyXG4gICAgLy8gVml0ZSBvcHRpb25zIHRhaWxvcmVkIGZvciBUYXVyaSBkZXZlbG9wbWVudCBhbmQgb25seSBhcHBsaWVkIGluIGB0YXVyaSBkZXZgIG9yIGB0YXVyaSBidWlsZGBcclxuICAgIC8vIHByZXZlbnQgdml0ZSBmcm9tIG9ic2N1cmluZyBydXN0IGVycm9yc1xyXG4gICAgY2xlYXJTY3JlZW46IGZhbHNlLFxyXG4gICAgLy8gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICAgIHNlcnZlcjoge1xyXG4gICAgICAgIHBvcnQ6IDE0MjAsXHJcbiAgICAgICAgc3RyaWN0UG9ydDogdHJ1ZSxcclxuICAgIH0sXHJcbiAgICAvLyB0byBtYWtlIHVzZSBvZiBgVEFVUklfREVCVUdgIGFuZCBvdGhlciBlbnYgdmFyaWFibGVzXHJcbiAgICAvLyBodHRwczovL3RhdXJpLnN0dWRpby92MS9hcGkvY29uZmlnI2J1aWxkY29uZmlnLmJlZm9yZWRldmNvbW1hbmRcclxuICAgIGVudlByZWZpeDogW1wiVklURV9cIiwgXCJUQVVSSV9cIl0sXHJcbiAgICBidWlsZDoge1xyXG4gICAgICAgIC8vIFRhdXJpIHN1cHBvcnRzIGVzMjAyMVxyXG4gICAgICAgIHRhcmdldDogcHJvY2Vzcy5lbnYuVEFVUklfUExBVEZPUk0gPT0gXCJ3aW5kb3dzXCIgPyBcImNocm9tZTEwNVwiIDogXCJzYWZhcmkxM1wiLFxyXG4gICAgICAgIC8vIGRvbid0IG1pbmlmeSBmb3IgZGVidWcgYnVpbGRzXHJcbiAgICAgICAgbWluaWZ5OiAhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcgPyBcImVzYnVpbGRcIiA6IGZhbHNlLFxyXG4gICAgICAgIC8vIHByb2R1Y2Ugc291cmNlbWFwcyBmb3IgZGVidWcgYnVpbGRzXHJcbiAgICAgICAgc291cmNlbWFwOiAhIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHLFxyXG4gICAgfSxcclxufSk7XHJcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBNFUsU0FBUyxvQkFBb0I7QUFDelcsU0FBUyxjQUFjO0FBQ3ZCLE9BQU8sc0JBQXNCO0FBRzdCLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQ3hCLFNBQVM7QUFBQSxJQUNMLE9BQU87QUFBQSxNQUNILFlBQVk7QUFBQSxRQUNSLGlCQUFpQjtBQUFBLFVBQ2IsWUFBWTtBQUFBLFFBQ2hCLENBQUM7QUFBQSxNQUNMO0FBQUEsSUFDSixDQUFDO0FBQUEsRUFDTDtBQUFBO0FBQUE7QUFBQSxFQUlBLGFBQWE7QUFBQTtBQUFBLEVBRWIsUUFBUTtBQUFBLElBQ0osTUFBTTtBQUFBLElBQ04sWUFBWTtBQUFBLEVBQ2hCO0FBQUE7QUFBQTtBQUFBLEVBR0EsV0FBVyxDQUFDLFNBQVMsUUFBUTtBQUFBLEVBQzdCLE9BQU87QUFBQTtBQUFBLElBRUgsUUFBUSxRQUFRLElBQUksa0JBQWtCLFlBQVksY0FBYztBQUFBO0FBQUEsSUFFaEUsUUFBUSxDQUFDLFFBQVEsSUFBSSxjQUFjLFlBQVk7QUFBQTtBQUFBLElBRS9DLFdBQVcsQ0FBQyxDQUFDLFFBQVEsSUFBSTtBQUFBLEVBQzdCO0FBQ0osQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
