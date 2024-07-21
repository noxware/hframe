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

export function render_fake_widget(widget) {
  console.log("Rendering fake widget", widget);
  if (widget.area.html_id) {
    if (!document.getElementById(widget.area.html_id)) {
      const el = document.createElement("div");
      el.id = widget.area.html_id;
      document.body.appendChild(el);
    }

    const el = document.getElementById(widget.area.html_id);
    el.innerHTML = widget.area.html_content;
    el.style.width = widget.area.abs_rect.size.width + "px";
    el.style.height = widget.area.abs_rect.size.height + "px";
    el.style.position = "absolute";
    el.style.left = widget.area.abs_rect.pos.x + "px";
    el.style.top = widget.area.abs_rect.pos.y + "px";
    el.style.backgroundColor = widget.color;
    document.body.appendChild(el);
  } else {
    const canvasEl = document.getElementById("canvas");
    const ctx = canvasEl.getContext("2d");

    ctx.fillStyle = widget.color;
    ctx.fillRect(
      widget.area.abs_rect.pos.x,
      widget.area.abs_rect.pos.y,
      widget.area.abs_rect.size.width,
      widget.area.abs_rect.size.height
    );
  }
}

export function clear_fake_widgets() {
  const canvasEl = document.getElementById("canvas");
  const ctx = canvasEl.getContext("2d");
  ctx.clearRect(0, 0, canvasEl.width, canvasEl.height);
}

// Rect and holes are relative to self element. They should come prepared.
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
