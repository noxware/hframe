const pointerPosition = { x: 0, y: 0 };

window.addEventListener("mousemove", (e) => {
  pointerPosition.x = e.clientX;
  pointerPosition.y = e.clientY;
});

const maskTemplate = `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <defs>
    <mask id="mask" x="0" y="0" width="{width}" height="{height}">
      <rect x="0" y="0" width="{width}" height="{height}" fill="white" />
      {holes}      
    </mask>
  </defs>
  <rect x="{x}" y="{y}" width="{width}" height="{height}" fill="blue" mask="url(#mask)" />
</svg>
`;

const holeTemplate = `
<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />
`;

export function log(message) {
  console.log(message);
}

export function getPointerPosition() {
  return pointerPosition;
}

export function sleepMs(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function transformElement(id, rect, holes) {
  const el = document.getElementById(id);
  const mask = maskTemplate
    .replaceAll("{width}", rect.width)
    .replaceAll("{height}", rect.height)
    .replaceAll("{x}", rect.x)
    .replaceAll("{y}", rect.y)
    .replaceAll(
      "{holes}",
      holes.map((hole) =>
        holeTemplate
          .replaceAll("{width}", hole.width)
          .replaceAll("{height}", hole.height)
          .replaceAll("{x}", hole.x)
          .replaceAll("{y}", hole.y)
      )
    );
  el.style.mask = `url(data:image/svg+xml,${mask})`;
}
