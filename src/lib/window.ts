import {
  getCurrentWindow,
  LogicalPosition,
  PhysicalPosition,
  PhysicalSize,
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

export async function jump() {
  const { x: startX, y: startY } = await getCurrentWindow().outerPosition();
  const { width: startXScale, height: startYScale } = await getCurrentWindow().innerSize()
  const transform = { x: startX, y: startY, sx: startXScale, sy: startYScale };

  animate(transform, {
    y: startY - 200,
    sy: startYScale * 1.2,
    sx: startXScale * 0.8,
    duration: 200,
    ease: "outQuad",
  }).then(() => {
    animate(transform, {
      y: startY,
      sy: startYScale,
      sx: startXScale,
      duration: 200,
      ease: "inQuad",
    });
  });
  animate(transform, {
    x: startX - 400,
    duration: 410,
    ease: "linear",
    onUpdate: async () => {
      console.log(transform)
      await getCurrentWindow().setPosition(
        new PhysicalPosition(Math.round(transform.x), Math.round(transform.y)),
      );
      await getCurrentWindow().setSize(
        new PhysicalSize(Math.round(transform.sx), Math.round(transform.sy))
      )
    },
  });
}
