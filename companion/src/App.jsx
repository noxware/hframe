import React from "react";
import { useAreas, useMousePosition, useWindowSize } from "./hooks";
import { Area } from "./components/Area";
import { MaskRect } from "./components/MaskRect";

function App() {
  const { windowWidth, windowHeight } = useWindowSize();
  const { mouseX, mouseY } = useMousePosition();
  const areas = useAreas();

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
