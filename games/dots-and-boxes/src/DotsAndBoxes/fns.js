import { useGameInfo } from "@foxfriends/minigames-client-react";
import intersection from "../util/intersection";

export default function useFns() {
  const { options: { size = 5 } = {} } = useGameInfo();

  function pxy(point) {
    return [point % (size + 1), Math.floor(point / (size + 1))];
  }

  function xyp(x, y) {
    return y * (size + 1) + x;
  }

  function fxy(face) {
    return [face % size, Math.floor(face / size)];
  }

  function xyf(x, y) {
    return y * size + x;
  }

  function getPointFaces(point) {
    const [x, y] = pxy(point, size);
    return [xyf(x - 1, y - 1), xyf(x - 1, y), xyf(x, y - 1), xyf(x, y)];
  }

  function getLineFaces(line) {
    const [start, end] = line;
    return intersection(getPointFaces(start), getPointFaces(end));
  }

  function getFacePoints(face) {
    const [x, y] = fxy(face);
    return [xyp(x, y), xyp(x + 1, y), xyp(x, y + 1), xyp(x + 1, y + 1)];
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
    xyp,
    fxy,
    xyf,
    getPointFaces,
    getLineFaces,
    getFacePoints,
    getFaceLines,
  };
}
