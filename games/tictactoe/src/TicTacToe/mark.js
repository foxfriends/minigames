import { X, O } from "./constants";
import cond, { otherwise } from "../util/cond";
import equals from "../util/equals";

export default cond([
  [equals(X), () => "✕"],
  [equals(O), () => "◯"],
  [otherwise, () => ""],
]);
