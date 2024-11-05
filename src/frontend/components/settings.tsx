import { ExternalLink } from "lucide-react";
import { Skeleton } from "./ui/skeleton";
import useSettings from "@/hooks/useSettings";
import useCanisterAddress from "@/hooks/useCanisterAddress";

export function Settings() {
  const { data: canister_address, isPending: isFetchingCanisterAddress } = useCanisterAddress();
  const { data: settings, isPending: isFetchingSettings } = useSettings();

  if (isFetchingSettings || !settings || isFetchingCanisterAddress || !canister_address) {
    return <Skeleton className="h-[19px] w-[125px] inline-block" />
  }

  const { token_in_address, token_out_address, token_out_name, token_in_name, amount_in, interval, slippage } = settings;

  return <section className="flex flex-col gap-2 w-full text-muted-foreground text text-sm max-w-[400px]">
    <div className="flex justify-between items-center w-full">
      <div>Canister address</div>
      <div><a href={`https://sepolia.etherscan.io/address/${canister_address}`} target="_blank">
        <code className="relative text-center rounded-sm bg-muted/40 px-[0.3rem] py-[0.2rem] font-mono hover:bg-muted/60 cursor-pointer"
        >
          {canister_address.slice(0, 5)}...{canister_address.slice(-5)}
          <ExternalLink className="inline-block h-3 w-3 ml-2 pb-[2px]" />
        </code></a>
      </div>
    </div>

    <div className="flex justify-between items-center w-full">
      <div>Token in</div>
      <div><a href={`https://sepolia.etherscan.io/address/${token_in_address}`} target="_blank">
        <code className="relative text-center rounded-sm bg-muted/40 px-[0.3rem] py-[0.2rem] font-mono hover:bg-muted/60 cursor-pointer"
        >
          {token_in_name}
          <ExternalLink className="inline-block h-3 w-3 ml-2 pb-[2px]" />
        </code></a>
      </div>
    </div>

    <div className="flex justify-between items-center w-full">
      <div>Token out</div>
      <div><a href={`https://sepolia.etherscan.io/address/${token_out_address}`} target="_blank">
        <code className="relative text-center rounded-sm bg-muted/40 px-[0.3rem] py-[0.2rem] font-mono hover:bg-muted/60 cursor-pointer"
        >
          {token_out_name}
          <ExternalLink className="inline-block h-3 w-3 ml-2 pb-[2px]" />
        </code></a>
      </div>
    </div>

    <div className="flex justify-between items-center w-full">
      <div>Buy amount</div>
      <div>
        {amount_in.toString()}
      </div>
    </div>


    <div className="flex justify-between items-center w-full">
      <div>Slippage</div>
      <div>
        {slippage.toString()}%
      </div>
    </div>


    <div className="flex justify-between items-center w-full">
      <div>Interval </div>
      <div>
        {interval.toString()} seconds
      </div>
    </div>

  </section >
}
