import { Cell, Universe } from "rustwasm-game-of-life";
import { memory } from "rustwasm-game-of-life/rustwasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct universe, and get its width and height
const universe = new Universe(128, 128);
const width = universe.width;
const height = universe.height;

// Draw Canvas
const canvas = document.getElementById("game-of-life-canvas") as HTMLCanvasElement;
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");
if (!ctx) {
    throw Error("CanvasRenderingContext2D is null");
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.lineWidth = 1 / window.devicePixelRatio;
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        let x = i * (CELL_SIZE + 1) + 1;
        ctx.moveTo(x, 0);
        ctx.lineTo(x, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines
    for (let j = 0; j <= height; j++) {
        let y = j * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, y);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, y);
    }

    ctx.stroke();
};

const getIndex = (row: number, column: number) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells_ptr();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE,
            );
        }
    }
    ctx.stroke();
};

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
