import { useGameState } from "@foxfriends/minigames-client-react";
import intersection from "../util/intersection";

export default function useFns() {
  const [{ size }] = useGameState();

  function pxy(point) {
    return [point % (size + 1), Math.floor(point / (size + 1))];
  }

  function fxy(face) {
    return [face % size, Math.floor(face / size)];
  }

  function getPointFaces(point) {
    const [x, y] = pxy(point, size);
    return [
      (y - 1) * size + (x - 1),
      (y - 1) * size + x,
      y * size + (x - 1),
      y * size + x,
    ];
  }

  function getLineFaces(line) {
    const [start, end] = line;
    return intersection(getPointFaces(start), getPointFaces(end));
  }

  function getFacePoints(face) {
    const [x, y] = fxy(face);
    return [
      y * size + x,
      y * size + x + 1,
      (y + 1) * size + x,
      (y + 1) * size + x + 1,
    ];
  }

  function getFaceLines(face) {
    const points = getFacePoints(face);
    return [
      [points[0], points[1]],
      [points[0], points[2]],
      [points[1], points[3]],
      [points[2], points[3]],
    ];
  }

  return {
    pxy,
    fxy,
    getPointFaces,
    getLineFaces,
    getFacePoints,
    getFaceLines,
  };
}
