import { h, Component, render } from "https://esm.sh/preact@10.23.2";
import { useState, useEffect } from "https://esm.sh/preact@10.23.2/hooks";

function Area(area) {
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
          pointerEvents: "auto",
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
  const [width, setWidth] = useState(window.innerWidth);
  const [height, setHeight] = useState(window.innerHeight);
  const [areas, setAreas] = useState([]);
  window.setAreas = setAreas;

  useEffect(() => {
    window.addEventListener("resize", () => {
      setWidth(window.innerWidth);
      setHeight(window.innerHeight);
    });
  }, []);

  console.log(`Width: ${width}, Height: ${height}`);

  useEffect(() => {
    console.log("Component mounted");
  }, []);

  return h(
    "svg",
    {
      width,
      height,
      style: {
        position: "absolute",
        top: 0,
        left: 0,
        mask: "url(#mask)",
        zIndex: 1000,
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

      ...areas.filter((area) => area.kind === "html").map(Area),
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
