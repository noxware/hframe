/** Main, mainly for scoping */
function main() {
  // When the mouse moves, adjust pointer events.
  window.addEventListener("mousemove", (e) => {
    const x = e.clientX;
    const y = e.clientY;

    document.querySelectorAll(".hframe-html-area").forEach((el) => {
      // TODO: Use the information at the tree of rects to determine.
    });
  });
}

main();

export function doSomething() {
  document.write("Hello, World!");
}
