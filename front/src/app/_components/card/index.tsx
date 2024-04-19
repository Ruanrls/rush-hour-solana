import {
  Card as ShadcnCard,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

type Props = {
  title: string;
  content: React.ReactNode;
};

export default function Card({ title, content }: Props) {
  return (
    <ShadcnCard className="shadow-xl">
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>

      <CardContent>{content}</CardContent>
    </ShadcnCard>
  );
}
