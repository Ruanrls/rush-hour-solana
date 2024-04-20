import {
  Connection,
  PublicKey,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

import { serialize, deserialize } from "borsh";
import env from "@/lib/env";
import { CustomParsedTransactionMeta, DeserializedSolution } from "@/types";
import { mapSolanaSolutionToString } from "@/lib/mappers";
import { boardToSolanaSchema, solanaResultSchema } from "@/lib/schemas";

export const createSolvePuzzleTransaction = async (
  board: number[][],
  publicKey: PublicKey,
  computeLimit: number = 2_000_000
) => {
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
            pubkey: publicKey,
            isSigner: true,
            isWritable: true,
          },
        ],
        programId: rushHourProgramId,
        data: rushHourProgramInstruction,
      }),
    ];

    const transaction = await buildTransaction(
      publicKey,
      rushHourProgramId,
      instructions,
      computeLimit
    );
    return transaction;
  } catch (e) {
    console.error(e);
    throw e;
  }
};

export const buildTransaction = async (
  payerKey: PublicKey,
  _programId: PublicKey,
  _instructions: TransactionInstruction[],
  _computeLimit: number = 1_000_000
) => {
  const message = new TransactionMessage({
    payerKey,
    instructions: _instructions,
    recentBlockhash: (
      await new Connection(env.CONNECTION_URL).getLatestBlockhash()
    ).blockhash,
  }).compileToV0Message();

  const transaction = new VersionedTransaction(message);
  return transaction;
};

export const parsePuzzleResponse = async (signature: string) => {
  const connection = new Connection(env.CONNECTION_URL);
  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature,
  });

  const response = await connection.getParsedTransaction(signature, {
    maxSupportedTransactionVersion: 0,
    commitment: "confirmed",
  });

  const meta = response?.meta as CustomParsedTransactionMeta;
  const data = meta?.returnData.data;

  const buffer = Buffer.from(data[0], "base64");
  const deserialized = deserialize(
    solanaResultSchema,
    buffer
  ) as DeserializedSolution;

  const mapped = mapSolanaSolutionToString(deserialized?.result);
  return mapped;
};

// [
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [1, 1, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0],
//   [0, 0, 0, 0, 0, 0]
// ]
