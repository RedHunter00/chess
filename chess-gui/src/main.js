const chessboard = document.getElementById("chessboard");

function allowDrop(ev) {
  ev.preventDefault();
}

function drag(ev) {
  ev.dataTransfer.setData("text", ev.target.id);
}

async function drop(ev) {
  ev.preventDefault();
  const data = ev.dataTransfer.getData("text");
  const draggedPiece = document.getElementById(data);

  const fromPosition = draggedPiece.parentElement.dataset.position;
  const targetElement = ev.target;
  const toPosition = targetElement.classList.contains("piece")
    ? targetElement.parentElement.dataset.position
    : targetElement.dataset.position;

  // Check if the move is legal

  const isLegalMove = await makeMove(fromPosition, toPosition);
  if (isLegalMove) {
    // Check if the drop target is a piece or a cell
    let targetCell = targetElement.classList.contains("piece") ? targetElement.parentElement : targetElement;

    // Remove the existing piece in the target cell, if any
    while (targetCell.firstChild) {
      targetCell.removeChild(targetCell.firstChild);
    }

    // Append the dragged piece to the target cell
    targetCell.appendChild(draggedPiece);
  }
}


async function makeMove(from, to) {
  const moveData = { from, to };
  return await window.__TAURI__.invoke("make_move", { moveData });
}


function createChessBoard() {
  for (let i = 0; i < 8; i++) {
    for (let j = 0; j < 8; j++) {
      const cell = document.createElement("div");
      cell.classList.add("cell");
      cell.classList.add((i + j) % 2 === 0 ? "white" : "black");
      cell.setAttribute("data-position", String.fromCharCode(97 + j) + (8 - i));
      cell.setAttribute("ondrop", "drop(event)");
      cell.setAttribute("ondragover", "allowDrop(event)");
      chessboard.appendChild(cell);
    }
  }
}

function renderGame(fen) {
  const cells = document.querySelectorAll(".cell");
  const x = fen.split("/");

  for (let rank = 7; rank >= 0; rank--) {
    let file = 0;
    for (const char of x[rank]) {
      if (Number.isInteger(parseInt(char))) {
        file += parseInt(char);
      } else {
        const position = String.fromCharCode(97 + file) + (8 - rank);
        const cell = document.querySelector(`[data-position="${position}"]`);
        if (cell) {
          const img = document.createElement("img");
          img.src = "images/" + char + ".svg";
          img.classList.add("piece");
          img.id = "piece-" + position;
          img.setAttribute("draggable", "true");
          img.setAttribute("ondragstart", "drag(event)");
          cell.appendChild(img);
        }
        file++;
      }
    }
  }
}

async function fetchFenAndRenderGame() {
  createChessBoard();
  const fen = await window.__TAURI__.invoke("get_fen");
  renderGame(fen);
}

fetchFenAndRenderGame();
