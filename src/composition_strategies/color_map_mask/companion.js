// Companion JS code for the color_map_mask composition strategy.

function setup() {
  window.color_map_mask = {};
  const cmm = window.color_map_mask;

  cmm.mouse_pos = { x: 0, y: 0 };
  window.addEventListener("mousemove", (e) => {
    cmm.mouse_pos = { x: e.clientX, y: e.clientY };
  });

  const canvasEl = document.createElement("canvas");
  canvasEl.id = "color_map_mask_canvas";
  canvasEl.style.position = "absolute";
  canvasEl.style.top = "0";
  canvasEl.style.left = "0";
  canvasEl.style.zIndex = "999999999";
  canvasEl.style.width = "100vw";
  canvasEl.style.height = "100vh";
  canvasEl.style.transform = "none";
  canvasEl.style.pointerEvents = "none";
  canvasEl.style.backgroundColor = "blue";
  canvasEl.style.opacity = "0.2";

  canvasEl.width = window.innerWidth;
  canvasEl.height = window.innerHeight;

  window.addEventListener("resize", () => {
    canvasEl.width = window.innerWidth;
    canvasEl.height = window.innerHeight;
  });

  document.body.appendChild(canvasEl);

  const ctx = canvasEl.getContext("2d");

  cmm.actions = {};

  cmm.actions.clear = () => {
    ctx.clearRect(0, 0, canvasEl.width, canvasEl.height);
  };

  cmm.actions.drawRect = (x, y, w, h, color) => {
    const radius = 5;
    ctx.beginPath();
    ctx.roundRect(x, y, w, h, radius);
    ctx.fillStyle = color;
    ctx.fill();
  };

  cmm.actions.getPixelColor = (x, y) => {
    const pixel = ctx.getImageData(x, y, 1, 1).data;
    return { r: pixel[0], g: pixel[1], b: pixel[2], a: pixel[3] };
  };
}

if (!window.color_map_mask) {
  setup();
}
