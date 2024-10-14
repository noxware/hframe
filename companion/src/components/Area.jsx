import React from "react";

export function Area({ area, interactive }) {
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
