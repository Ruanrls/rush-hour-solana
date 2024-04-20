"use client";

import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { createSolvePuzzleTransaction, parsePuzzleResponse } from "./actions";
import { LoaderCircleIcon } from "lucide-react";
import { useState } from "react";
import Card from "../card";
import { cn } from "@/lib/utils";
import { useToast } from "@/components/ui/use-toast";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { connect } from "http2";
import { sendAndConfirmTransaction } from "@solana/web3.js";

export default function Solver() {
  const toast = useToast();
  const { connection } = useConnection();
  const { publicKey, signTransaction } = useWallet();

  const [board, setBoard] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [solution, setSolution] = useState<string[]>([]);

  const handleSolve = async () => {
    if (!publicKey) {
      return toast.toast({
        variant: "destructive",
        title: "Error",
        description: "Please connect your wallet",
      });
    }

    try {
      setLoading(true);
      const boardParsed = JSON.parse(board);
      const transaction = await createSolvePuzzleTransaction(
        boardParsed,
        publicKey
      );

      const signedTransaction = await signTransaction?.(transaction);
      if (!signedTransaction) {
        return null;
      }

      const signature = await connection.sendTransaction(signedTransaction);
      const result = await parsePuzzleResponse(signature);

      setSolution(result);
    } catch (e: any) {
      console.log("🚀 ~ handleSolve ~ e:", e);
      if (e.message.includes("JSON")) {
        return toast.toast({
          variant: "destructive",
          title: "Invalid JSON",
          description: "Please provide a valid JSON board",
        });
      }

      if (e.message.includes("The user rejected the request")) {
        return toast.toast({
          variant: "destructive",
          title: "Error",
          description: "Please approve the transaction in your wallet",
        });
      }

      toast.toast({
        variant: "destructive",
        title: "Error",
        description: "Something went wrong, please try again!",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="w-[420px] relative">
      <div>
        <Label>Solve your rush hour: </Label>
        <Textarea
          onChange={(e) => setBoard(e.target.value)}
          className="mt-2 h-48 resize-none"
          placeholder="Place your board game here"
        />

        <Button className="mt-4 w-full" disabled={!board} onClick={handleSolve}>
          {loading ? <LoaderCircleIcon className="animate-spin" /> : "Solve"}
        </Button>
      </div>
      <div
        className={cn(
          "absolute mt-12 -left-[24px] w-[468px] transition-all ease-in duration-700 opacity-0 translate-y-5",
          {
            "opacity-1 translate-y-0": !!solution?.length,
          }
        )}
      >
        <Card
          title="Your Result"
          content={
            <div className="">
              <Label className="text-md">Moves:</Label>
              <div className="overflow-y-auto text-sm text-gray-500">
                {solution.map((move, index) => (
                  <div key={index}>{move}</div>
                ))}
              </div>
            </div>
          }
        />
      </div>
    </div>
  );
}
