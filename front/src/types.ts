import { ParsedTransactionMeta } from "@solana/web3.js";

export interface CustomParsedTransactionMeta extends ParsedTransactionMeta {
  returnData: {
    data: [string, string];
    programId: string;
  };
}

export type Solution = {
  id: number;
  direction: number;
}[];

export type DeserializedSolution = {
  result: Solution;
};

export enum Direction {
  UP = 0,
  DOWN = 1,
  LEFT = 2,
  RIGHT = 3,
}