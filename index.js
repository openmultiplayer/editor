const rust = import("./pkg/editor");
const canvas = document.getElementById("canvas");

rust.then((m) => {
  const FPS = 1000.0 / 60.0; // ms/frames

  var lastDrawTime = -1;
  function render() {
    window.requestAnimationFrame(render);
    const currentTime = Date.now();
    if (currentTime >= lastDrawTime + FPS) {
      lastDrawTime = currentTime;
      if (
        window.innerHeight != canvas.height ||
        window.innerWidth != canvas.width
      ) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;
        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;
      }
    }
  }
  render();
});
