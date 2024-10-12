import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";

export function run() {
  window.hframeDebug = {};

  const root = document.createElement("div");
  root.style.display = "contents";
  document.body.appendChild(root);

  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>
  );
}

export function set_areas(areas) {
  if (window.setAreas) {
    window.hframeDebug.areas = areas;
    window.setAreas(areas);
  } else {
    console.error("set_areas function not available");
  }
}

export function log(message) {
  console.log(message);
}
