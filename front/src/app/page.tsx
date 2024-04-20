import dynamic from "next/dynamic";
import Card from "./_components/card";
import Solver from "./_components/form/solve";
import { Skeleton } from "@/components/ui/skeleton";
const WalletConnect = dynamic(
  async () => await import("./_components/wallet-connect"),
  {
    ssr: false,
    loading: () => <Skeleton className="w-[234px] h-16" />,
  }
);

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center pt-24">
      <WalletConnect />

      <div className="mt-4">
        <Card title="Rush Hour Solver" content={<Solver />} />
      </div>
    </main>
  );
}
