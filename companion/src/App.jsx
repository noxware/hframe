import React, { useState, useEffect } from "react";

function Area({ area, interactive }) {
  return (
    <foreignObject
      key={area.id}
      x={area.x}
      y={area.y}
      width={area.width}
      height={area.height}
      style={{ overflow: "hidden" }}
    >
      <div
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          backgroundColor: "lightblue",
          visibility: area.visible ? "visible" : "hidden",
          pointerEvents: interactive ? "auto" : "none",
        }}
        onMouseEnter={() => {
          console.log("Mouse entered");
        }}
        dangerouslySetInnerHTML={{
          __html: area.content,
        }}
      />
    </foreignObject>
  );
}

function MaskRect({ id, x, y, width, height, kind }) {
  return (
    <rect
      key={id}
      x={x}
      y={y}
      width={width}
      height={height}
      // TODO: This roundness looks good for egui windows but should not be hardcoded.
      // In the future, a canvas area may not be a window.
      rx={kind === "canvas" ? 7 : 0}
      fill={kind === "canvas" ? "black" : "white"}
    />
  );
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

  const hoveredArea = areas
    .slice()
    .reverse()
    .find((area) => {
      return (
        area.x <= mouseX &&
        mouseX <= area.x + area.width &&
        area.y <= mouseY &&
        mouseY <= area.y + area.height
      );
    });

  console.log(mouseX, mouseY);
  console.log(hoveredArea);

  return (
    <svg
      width={windowWidth}
      height={windowHeight}
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        mask: "url(#mask)",
        zIndex: 1000,
        // Issue: This causes window mousemove events to be ignored.
        pointerEvents: "none",
      }}
    >
      <defs>
        <mask id="mask">
          {areas.map((area) => (
            <MaskRect key={area.id} {...area} />
          ))}
        </mask>
      </defs>

      {areas
        .filter((area) => area.kind === "html")
        .map((area) => (
          <Area key={area.id} area={area} interactive={area === hoveredArea} />
        ))}
    </svg>
  );
}

export default App;
