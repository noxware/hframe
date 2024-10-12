import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";

export function run(root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>
  );
}
