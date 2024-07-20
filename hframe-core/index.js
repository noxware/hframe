const canvasEl = document.getElementById("canvas");
const ctx = canvasEl.getContext("2d");

// draw 3 overlaped rectangles, blue red and green

ctx.fillStyle = "blue";
ctx.fillRect(10, 10, 100, 100);

ctx.fillStyle = "red";
ctx.fillRect(50, 50, 100, 100);

ctx.fillStyle = "green";
ctx.fillRect(90, 90, 100, 100);
