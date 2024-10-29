import useEthBalance from "@/hooks/useEthBalance";
import { Skeleton } from "./ui/skeleton";
import { ethStringToDecimal } from "@/lib/eth";
import useEthAddress from "@/hooks/useEthAddress";

export function Balance() {
  const { data: address } = useEthAddress();
  const { data: balance, isPending: isFetchingBalance, isError } = useEthBalance(address);

  if (isFetchingBalance) {
    return <Skeleton className="w-full h-14" />
  }

  if (isError) {
    return (
      <div className="text-4xl font-semibold bg-destructive/30 rounded-lg p-2 text-destructive-foreground">
        Couldn't get wallet balance.
      </div>
    )
  };

  return (
    <div className="text-4xl font-semibold">
      {ethStringToDecimal(balance)} SepoliaETH
    </div>
  );

}
