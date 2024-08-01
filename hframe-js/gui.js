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

function getOrCreateEmbed(id) {
  const existing = document.getElementById(id);

  if (existing) {
    return existing;
  }

  const embed = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "foreignObject"
  );
  embed.id = id;
  document.getElementById("hframe-svg").appendChild(embed);

  return embed;
}

function getOrCreateMask(id) {
  const existing = document.getElementById(id);

  if (existing) {
    return existing;
  }

  const mask = document.createElementNS("http://www.w3.org/2000/svg", "mask");
  mask.id = id;
  document.getElementById("hframe-svg-defs").appendChild(mask);
  return mask;
}

function getOrCreateMaskSlot(mask, id, mode) {
  const existing = document.getElementById(id);

  if (existing) {
    return existing;
  }

  const component = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "rect"
  );
  component.id = id;

  if (mode === "drill") {
    setAttributes(component, { fill: "black" });
    mask.appendChild(component);
  } else {
    setAttributes(component, { fill: "white" });
    mask.prepend(component);
  }

  return component;
}

function setContent(embed, content) {
  if (embed.innerHTML !== content) {
    embed.innerHTML = content;
  }
}

function setAttributes(embed, map) {
  for (const [name, value] of Object.entries(map)) {
    if (embed.getAttribute(name) !== value) {
      embed.setAttribute(name, value);
    }
  }
}

function setStyles(embed, map) {
  for (const [name, value] of Object.entries(map)) {
    if (embed.style[name] !== value) {
      embed.style[name] = value;
    }
  }
}

function fillMask(mask, id, x, y, width, height) {
  const slot = getOrCreateMaskSlot(mask, id, "fill");
  setAttributes(slot, {
    x,
    y,
    width,
    height,
  });
}

function drillMask(mask, id, x, y, width, height) {
  const slot = getOrCreateMaskSlot(mask, id, "drill");
  setAttributes(slot, {
    x,
    y,
    width,
    height,
  });
}

function interpret() {
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

  const loop = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (const widget of widgets) {
      if (widget.kind === "canvas") {
        ctx.fillStyle = "red";
        ctx.fillRect(widget.x, widget.y, widget.width, widget.height);

        // begin very specific code
        const mask = getOrCreateMask("h0-mask");
        drillMask(
          mask,
          "c0-drill",
          widget.x,
          widget.y,
          widget.width,
          widget.height
        );
        // end very specific code
      } else if (widget.kind === "html") {
        const embed = getOrCreateEmbed(widget.id);
        const mask = getOrCreateMask(widget.id + "-mask");

        setAttributes(embed, {
          x: widget.x,
          y: widget.y,
          width: widget.width,
          height: widget.height,
          mask: `url(#${mask.id})`,
        });

        setContent(embed, widget.content);

        fillMask(
          mask,
          widget.id + "-fill",
          widget.x,
          widget.y,
          widget.width,
          widget.height
        );
      }
    }

    widgets[0].x += 0.1;
    requestAnimationFrame(loop);
  };

  loop();
}

oneTimeSetup();
interpret();
