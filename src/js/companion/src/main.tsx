import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";

export function run(root: HTMLElement) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>
  );
}
