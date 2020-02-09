import { Universe, Cell, ayo } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
// import * as wasm from "wasm-game-of-life";
const CELL_SIZE = 5; // px
const DEAD_COLOR = "#FFFFFF";
// const DEAD_COLOR = "#000000";
const ALIVE_COLOR = "#000000";
var universe = Universe.new(64 * 3, 64 * 3);
var width = universe.width();
var height = universe.height();
console.log(width);
console.log(height);
var canvas = document.getElementById('game-of-life-canvas');
canvas.width = document.body.clientWidth; //document.width is obsolet
canvas.height = document.body.clientHeight; //document.height is obsolete
  if (canvas.getContext) {
    var ctx = canvas.getContext('2d');
  }
// document.getElementById("nextMove").addEventListener("click", nextStep());





const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};



const nextStep = () =>  {
    fps.render();
    universe.next_step();
    drawCells();
    animationId = requestAnimationFrame(nextStep);
}

// draw();
const getIndex = (y, x) => {
    return y * width + x
}

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = getIndex(y, x);

      ctx.fillStyle = cells[idx] === Cell.DEATH
        ? DEAD_COLOR
        : ALIVE_COLOR;
      ctx.fillRect(
        x * (CELL_SIZE + 1) + 1,
        y * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
// drawCells();
nextStep()
