import { Skeleton } from "./ui/skeleton";
import { useTokenInBalance, useTokenOutBalance } from "@/hooks/useBalance";

export function Balance() {
  const { data: tokenInBalance, isPending: isFetchingInBalance } = useTokenInBalance();
  const { data: tokenOutBalance, isPending: isFetchingOutBalance } = useTokenOutBalance();

  return <section className="flex flex-col gap-2 w-full text-foreground text text-sm max-w-[400px]">
    <div className="flex justify-between items-center w-full">
      <div>Token in balance:</div>
      <div>
        {isFetchingInBalance ? <Skeleton className="h-[19px] w-[125px] inline-block" /> : tokenInBalance}
      </div>
    </div>

    <div className="flex justify-between items-center w-full">
      <div>Token out balance:</div>
      <div>
        {isFetchingOutBalance ? <Skeleton className="h-[19px] w-[125px] inline-block" /> : tokenOutBalance}
      </div>
    </div>

  </section >
}
