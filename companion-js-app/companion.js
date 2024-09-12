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
          pointerEvents: area.interactive ? "auto" : "none",
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
    fill: area.type === "canvas" ? "black" : "white",
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

      ...areas.filter((area) => area.type === "html").map(Area),
    ]
  );
}

render(h(App), document.body);
window.dispatchEvent(new CustomEvent("hframeJsLoaded"));
