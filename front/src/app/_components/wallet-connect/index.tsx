"use client";
import { Button } from "@/components/ui/button";
import { useWallet } from "@solana/wallet-adapter-react";
import Image from "next/image";

import "@solana/wallet-adapter-react-ui/styles.css";
import { useMemo } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { LoaderCircle } from "lucide-react";
import { WalletName } from "@solana/wallet-adapter-base";
import { useToast } from "@/components/ui/use-toast";

export default function WalletConnect() {
  const toast = useToast();
  const { wallets, publicKey, select, disconnect, disconnecting } = useWallet();

  const noWalletsInstalled = useMemo(
    () => wallets?.every((wallet) => wallet.readyState !== "Installed"),
    [wallets]
  );

  const handleSelectWallet = (name: WalletName) => {
    select(name);
  };

  const handleCopyAddres = async () => {
    if (!publicKey) return;

    await navigator.clipboard.writeText(publicKey.toBase58());
    toast.toast({
      title: "Copied",
      description: "Address copied to clipboard",
    });
  };

  if (noWalletsInstalled)
    return <span className="text-destructive my-4">No wallets installed</span>;

  if (publicKey) {
    return (
      <div>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button>
              <h1>Connected: &nbsp;</h1>
              <p>{publicKey.toBase58().slice(0, 20)}...</p>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-[300px]">
            <DropdownMenuItem
              className="w-full cursor-pointer"
              onClick={handleCopyAddres}
            >
              Copy address
            </DropdownMenuItem>
            <DropdownMenuItem
              onClick={() => disconnect()}
              className="w-full cursor-pointer"
              disabled={disconnecting}
            >
              {disconnecting ? (
                <LoaderCircle className="animate-spin" />
              ) : (
                "Disconnect"
              )}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    );
  }

  return (
    <div>
      <h1 className="bold text-gray-600">Wallet Connect: </h1>
      <div className="flex gap-x-4">
        {wallets
          .filter((wallet) => wallet.readyState === "Installed")
          .map((wallet) => (
            <Button
              key={wallet.adapter.name}
              onClick={() => handleSelectWallet(wallet.adapter.name)}
              variant="outline"
            >
              <Image
                className="mr-2"
                src={wallet.adapter.icon}
                alt={wallet.adapter.name}
                width={14}
                height={14}
              />
              {wallet.adapter.name}
            </Button>
          ))}
      </div>
    </div>
  );
}
