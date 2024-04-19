"use server";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

import { serialize, deserialize } from "borsh";
import env from "./env";
import { CustomParsedTransactionMeta, DeserializedSolution } from "@/types";
import { mapSolanaSolutionToString } from "./mappers";
import { boardToSolanaSchema, solanaResultSchema } from "./schemas";

const loadKeyPair = () => {
  const keyPairBuffer = JSON.parse(env.KEY_PAIR);

  const keys = Keypair.fromSeed(new Uint8Array(keyPairBuffer.slice(0, 32)));
  return keys;
};

export const callProgram = async (
  _programId: PublicKey,
  keys: Keypair[],
  _instructions: TransactionInstruction[],
  _computeLimit: number = 1_000_000
) => {
  const connection = new Connection(env.CONNECTION_URL);
  const transaction = new Transaction();
  const limit = ComputeBudgetProgram.setComputeUnitLimit({
    units: _computeLimit,
  });

  transaction.add(limit, ..._instructions);

  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    keys
  );

  const response = await connection.getParsedTransaction(signature);
  return response;
};

export const solvePuzzle = async (
  board: number[][],
  computeLimit: number = 2_000_000
) => {
  const keys = loadKeyPair();
  const rushHourProgramId = new PublicKey(env.PROGRAM_ID);

  try {
    const rushHourProgramInstruction = Buffer.from(
      serialize(boardToSolanaSchema, {
        board,
      })
    );

    const instructions = [
      new TransactionInstruction({
        keys: [
          {
            pubkey: keys.publicKey,
            isSigner: true,
            isWritable: true,
          },
        ],
        programId: rushHourProgramId,
        data: rushHourProgramInstruction,
      }),
    ];

    const response = await callProgram(
      rushHourProgramId,
      [keys],
      instructions,
      computeLimit
    );
    const meta = response?.meta as CustomParsedTransactionMeta;
    const data = meta?.returnData.data;

    const buffer = Buffer.from(data[0], "base64");
    const deserialized = deserialize(
      solanaResultSchema,
      buffer
    ) as DeserializedSolution;

    const mapped = mapSolanaSolutionToString(deserialized?.result);
    return mapped;
  } catch (e) {
    console.error(e);
    throw e;
  }
};

// [
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [1, 1, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0]
// ]
