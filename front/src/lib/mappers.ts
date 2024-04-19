import { Direction, Solution } from "@/types";

export const mapSolanaSolutionToString = (solution: Solution) => {
  return solution.map((s) => {
    switch (s.direction) {
      case Direction.UP:
        return `Move car ${s.id} up`;
      case Direction.RIGHT:
        return `Move car ${s.id} right`;
      case Direction.DOWN:
        return `Move car ${s.id} down`;
      default:
        return `Move car ${s.id} left`;
    }
  });
};
