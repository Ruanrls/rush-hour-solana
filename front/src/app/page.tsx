import Card from "./_components/card";
import Solver from "./_components/form/solve";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center">
      <div>
        <Card title="Rush Hour Solver" content={<Solver />} />
      </div>
    </main>
  );
}
