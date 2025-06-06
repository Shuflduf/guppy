import {
  getCurrentWindow,
  LogicalPosition,
  PhysicalPosition,
} from "@tauri-apps/api/window";
import { animate } from "animejs";

export async function moveSlightly() {
  const currentPosition = await getCurrentWindow().innerPosition();
  const angle = Math.random() * 2 * Math.PI;
  const distance = 10;
  const newPosition = {
    x: currentPosition.x + Math.round(distance * Math.cos(angle)),
    y: currentPosition.y + Math.round(distance * Math.sin(angle)),
  };
  getCurrentWindow().setPosition(
    new LogicalPosition(newPosition.x, newPosition.y),
  );
}

export async function moveWindowBy(delta: PhysicalPosition) {
  const currentPosition = await getCurrentWindow().innerPosition();

  const newPosition = new PhysicalPosition(
    currentPosition.x + delta.x,
    currentPosition.y + delta.y,
  );
  getCurrentWindow().setPosition(newPosition);
}

export async function moveSmoothly() {
  const { x: startX, y: startY } = await getCurrentWindow().outerPosition();
  const position = { x: startX, y: startY };

  animate(position, {
    x: startX - 400,
    duration: 410,
    ease: "linear",
    onUpdate: () => {
      getCurrentWindow().setPosition(
        new PhysicalPosition(Math.round(position.x), Math.round(position.y)),
      );
    },
  });
  animate(position, {
    y: startY - 200,
    duration: 200,
    ease: "outQuad",
  }).then(() => {
    animate(position, {
      y: startY,
      duration: 200,
      ease: "inQuad",
    });
  });
}
