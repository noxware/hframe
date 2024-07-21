const pointerPosition = { x: 0, y: 0 };

window.addEventListener("mousemove", (e) => {
  pointerPosition.x = e.clientX;
  pointerPosition.y = e.clientY;
});

export function log(message) {
  console.log(message);
}

export function dangerous_eval(code) {
  return eval(code);
}

export function get_pointer_position() {
  return pointerPosition;
}

export function sleep_ms(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function transform_element(id, rect, holes) {
  // TODO: Reuse canvas, adjusting sizes and cleaning when necessary.

  const canvas = new OffscreenCanvas(rect.size.width, rect.size.height);
  const ctx = canvas.getContext("2d");

  // Fill everything with solid color
  ctx.fillStyle = "white";
  ctx.fillRect(0, 0, rect.size.width, rect.size.height);

  // Cut holes
  ctx.fillStyle = "black";
  holes.forEach((hole) => {
    ctx.fillRect(hole.pos.x, hole.pos.y, hole.size.width, hole.size.height);
  });

  // Get the image as blob
  const blob = await canvas.convertToBlob();

  // Make an url from the blob
  const url = URL.createObjectURL(blob);

  // Use that as a mask
  const el = document.getElementById(id);
  el.style.maskImage = `url(${url})`;
  el.style.maskSize = "100% 100%";
  el.style.maskMode = "luminance";
}
