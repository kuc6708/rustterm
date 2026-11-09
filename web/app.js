// rustterm web frontend: xterm.js over a WebSocket PTY. Clean-room original.
(function () {
  "use strict";

  const term = new Terminal({
    cursorBlink: true,
    fontFamily: "monospace",
    fontSize: 14,
    theme: { background: "#111111" },
    scrollback: 5000,
  });
  const fit = new FitAddon.FitAddon();
  term.loadAddon(fit);
  term.open(document.getElementById("term"));
  fit.fit();
  term.focus();

  const status = document.getElementById("status");
  const scheme = location.protocol === "https:" ? "wss" : "ws";
  const ws = new WebSocket(`${scheme}://${location.host}/ws`);
  ws.binaryType = "arraybuffer";

  function sendResize() {
    if (ws.readyState !== WebSocket.OPEN) return;
    ws.send(JSON.stringify({ type: "resize", rows: term.rows, cols: term.cols }));
  }

  term.onData((data) => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify({ type: "input", data }));
    }
  });

  ws.onopen = () => {
    status.textContent = "connected";
    fit.fit();
    sendResize();
  };
  ws.onclose = () => {
    status.textContent = "disconnected";
  };
  ws.onmessage = (ev) => {
    if (ev.data instanceof ArrayBuffer) {
      term.write(new Uint8Array(ev.data));
    } else {
      term.write(ev.data);
    }
  };

  term.onResize(() => sendResize());
  window.addEventListener("resize", () => {
    fit.fit();
    sendResize();
  });
})();
