"use client";

import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { solvePuzzle } from "@/lib/api";
import { LoaderCircleIcon } from "lucide-react";
import { useState } from "react";
import Card from "../card";
import { cn } from "@/lib/utils";
import { useToast } from "@/components/ui/use-toast";

export default function Solver() {
  const toast = useToast();

  const [board, setBoard] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [solution, setSolution] = useState<string[]>([]);

  const handleSolve = async () => {
    try {
      setLoading(true);
      const parsed = JSON.parse(board);
      const result = await solvePuzzle(parsed);
      setSolution(result);
      setLoading(false);
    } catch (e: any) {
      if (e.message.includes("JSON")) {
        toast.toast({
          variant: "destructive",
          title: "Invalid JSON",
          description: "Please provide a valid JSON board",
        });
      } else {
        toast.toast({
          variant: "destructive",
          title: "Error",
          description: "Something went wrong, please try again!",
        });
      }

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

        <Button className="mt-4 w-full" onClick={handleSolve}>
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
