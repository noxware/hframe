const mousePosition = { x: 0, y: 0 };

window.addEventListener("mousemove", (e) => {
  mousePosition.x = e.clientX;
  mousePosition.y = e.clientY;
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

const incomingMessages = [];
const outgoingMessages = [];

/** Exposes a way for WASM to send messages to JS. */
export function sendMessage(message) {
  incomingMessages.push(message);
}

/** Exposes a way for WASM to read messages from JS. Clears the messages. */
export function receiveMessages() {
  return outgoingMessages.splice(0, outgoingMessages.length);
}

/** Exposes a way for WASM to tell the JS side to do the work of a cycle,
 *  processing all pending messages.
 */
export function tick() {
  for (const message of incomingMessages) {
    switch (message.type) {
      case "TransformElement":
        transformElement(message.id, message.rect, message.holes);
        break;
      case "Log":
        console.log(message.message);
        break;
      default:
        throw new Error(`Unknown message type: ${message.type}`);
    }
  }
  incomingMessages.length = 0;
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
