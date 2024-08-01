function initCanvasLayer() {
  const canvas = document.createElement("canvas");
  canvas.id = "hframe-canvas";
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  document.body.appendChild(canvas);
}

function initSvgLayer() {
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.id = "hframe-svg";
  svg.setAttribute("width", window.innerWidth);
  svg.setAttribute("height", window.innerHeight);
  svg.style.position = "absolute";
  svg.style.top = 0;
  svg.style.left = 0;
  svg.innerHTML = `<defs id="hframe-svg-defs"></defs>`;

  document.body.appendChild(svg);
}

function oneTimeSetup() {
  initCanvasLayer();
  initSvgLayer();
}

oneTimeSetup();

function play() {
  const widgets = [
    {
      id: "c0",
      kind: "canvas",
      x: 120,
      y: 180,
      width: 200,
      height: 200,
    },
    {
      id: "h0",
      kind: "html",
      x: 300,
      y: 300,
      width: 200,
      height: 200,
      content: `<h1 style="background-color: blue; width: 100%; height: 100%;">Hello</h1>`,
    },
  ];

  const canvas = document.getElementById("hframe-canvas");
  const ctx = canvas.getContext("2d");
  let lastTime = 0;

  const loop = (now) => {
    const dt = now - lastTime;
    lastTime = now;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (const widget of widgets) {
      if (widget.kind === "canvas") {
        ctx.fillStyle = "red";
        ctx.fillRect(widget.x, widget.y, widget.width, widget.height);
      } else if (widget.kind === "html") {
        const existing = document.getElementById(widget.id);

        if (existing) {
          const wrapper = existing.firstChild;
          if (wrapper.innerHTML !== widget.content) {
            wrapper.innerHTML = widget.content;
          }
        } else {
          const embed = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "foreignObject"
          );
          embed.id = widget.id;
          embed.setAttribute("x", widget.x);
          embed.setAttribute("y", widget.y);
          embed.setAttribute("width", widget.width);
          embed.setAttribute("height", widget.height);

          const wrapper = document.createElement("div");
          wrapper.innerHTML = widget.content;
          wrapper.style.width = "100%";
          wrapper.style.height = "100%";
          wrapper.style.maxWidth = "100%";
          wrapper.style.maxHeight = "100%";
          wrapper.style.overflow = "auto";
          embed.appendChild(wrapper);

          const svg = document.getElementById("hframe-svg");
          svg.appendChild(embed);
        }
      }
    }

    requestAnimationFrame(loop);
  };

  loop();
}

play();
