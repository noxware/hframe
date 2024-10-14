import React from "react";

export function MaskRect({ id, x, y, width, height, kind }) {
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
