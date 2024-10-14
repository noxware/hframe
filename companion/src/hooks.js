import { useState, useEffect } from "react";

export function useWindowSize() {
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  const [windowHeight, setWindowHeight] = useState(window.innerHeight);

  useEffect(() => {
    const handleResize = () => {
      setWindowWidth(window.innerWidth);
      setWindowHeight(window.innerHeight);
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return { windowWidth, windowHeight };
}

export function useMousePosition() {
  const [mouseX, setMouseX] = useState(0);
  const [mouseY, setMouseY] = useState(0);

  useEffect(() => {
    const handleMouseMove = (event) => {
      setMouseX(event.clientX);
      setMouseY(event.clientY);
    };

    // Because of the svg having no pointer events, only the canvas can read mousemove, document, window and body can't, idk why.
    // And this will not trigger if canvas is not receiving mousemove events.
    document
      .getElementById("the_canvas_id")
      .addEventListener("mousemove", handleMouseMove);

    // When the canvas does not receive mousemove, maybe other elements are capturing it and window should receive it.
    window.addEventListener("mousemove", handleMouseMove);

    return () => {
      document
        .getElementById("the_canvas_id")
        .removeEventListener("mousemove", handleMouseMove);

      window.removeEventListener("mousemove", handleMouseMove);
    };
  }, []);

  return { mouseX, mouseY };
}

export function useAreas() {
  const [areas, setAreas] = useState([]);
  window.setAreas = setAreas;

  return areas;
}
