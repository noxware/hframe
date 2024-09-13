import { h, Component, render } from "https://esm.sh/preact@10.23.2";
import { useState, useEffect } from "https://esm.sh/preact@10.23.2/hooks";

function Area({ area, interactive }) {
  return h(
    "foreignObject",
    {
      key: area.id,
      x: area.x,
      y: area.y,
      width: area.width,
      height: area.height,
      style: {
        overflow: "hidden",
      },
    },
    [
      h("div", {
        style: {
          width: "100%",
          height: "100%",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          backgroundColor: "lightblue",
          visibility: area.visible ? "visible" : "hidden",
          pointerEvents: interactive ? "auto" : "none",
        },
        onMouseEnter: () => {
          console.log("Mouse entered");
        },
        dangerouslySetInnerHTML: {
          __html: area.content,
        },
      }),
    ]
  );
}

function MaskRect(area) {
  return h("rect", {
    key: area.id,
    x: area.x,
    y: area.y,
    width: area.width,
    height: area.height,
    // TODO: This roundness looks good for egui windows but should not be hardcoded.
    // In the future, a canvas area may not be a window.
    rx: area.kind === "canvas" ? 7 : 0,
    fill: area.kind === "canvas" ? "black" : "white",
  });
}

function App() {
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  const [windowHeight, setWindowHeight] = useState(window.innerHeight);
  const [mouseX, setMouseX] = useState(0);
  const [mouseY, setMouseY] = useState(0);
  const [areas, setAreas] = useState([]);
  window.setAreas = setAreas;

  useEffect(() => {
    const handleResize = () => {
      setWindowWidth(window.innerWidth);
      setWindowHeight(window.innerHeight);
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  useEffect(() => {
    const handleMouseMove = (event) => {
      setMouseX(event.clientX);
      setMouseY(event.clientY);
    };

    // Because of the svg having no pointer events, only the canvas can read mousemove, document, window and body can't, idk why.
    document
      .getElementById("the_canvas_id")
      .addEventListener("mousemove", handleMouseMove);
    return () =>
      document
        .getElementById("the_canvas_id")
        .removeEventListener("mousemove", handleMouseMove);
  }, []);

  const htmlAreas = areas.filter((area) => area.kind === "html");

  const hoveredHtmlArea = htmlAreas.findLast((area) => {
    return (
      area.x <= mouseX &&
      mouseX <= area.x + area.width &&
      area.y <= mouseY &&
      mouseY <= area.y + area.height
    );
  });
  console.log(mouseX, mouseY);
  console.log(htmlAreas[htmlAreas.length - 1]);
  console.log(hoveredHtmlArea);

  return h(
    "svg",
    {
      width: windowWidth,
      height: windowHeight,
      style: {
        position: "absolute",
        top: 0,
        left: 0,
        mask: "url(#mask)",
        zIndex: 1000,
        // Issue: This causes window mousemove events to be ignored.
        pointerEvents: "none",
      },
    },
    [
      h("defs", {}, [
        h(
          "mask",
          {
            id: "mask",
          },
          areas.map(MaskRect)
        ),
      ]),

      htmlAreas.map((area) =>
        Area({ area, interactive: area === hoveredHtmlArea })
      ),
    ]
  );
}

export function run() {
  window.hframeDebug = {};
  render(h(App), document.body);
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
