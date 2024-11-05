import { LogItem } from "src/backend/declarations/backend.did";
import { Skeleton } from "./ui/skeleton";
import useEventLog from "@/hooks/useEventLog";
import { cn } from "@/lib/utils";
import { ExternalLink } from "lucide-react";


function AddressLink({ address }: { address: string }) {
  return <a href={`https://sepolia.etherscan.io/address/${address}`} target="_blank">
    <code className="relative text-center rounded-sm bg-secondary px-[0.3rem] py-[0.2rem] font-mono hover:bg-secondary/60 cursor-pointer"
    >
      {address.slice(0, 5)}...{address.slice(-5)}
      <ExternalLink className="inline-block h-3 w-3 ml-2 pb-[2px]" />
    </code>
  </a>
}

function TxLink({ tx }: { tx: string }) {
  return <a href={`https://sepolia.etherscan.io/tx/${tx}`} target="_blank">
    <code className="relative text-center rounded-sm bg-secondary px-[0.3rem] py-[0.2rem] font-mono hover:bg-secondary/60 cursor-pointer"
    >
      {tx.slice(0, 5)}...{tx.slice(-5)}
      <ExternalLink className="inline-block h-3 w-3 ml-2 pb-[2px]" />
    </code>
  </a>
}

function OkMessage({ logItem }: { logItem: LogItem }) {
  if (logItem.err.length > 0 || logItem.ok.length === 0) return;
  const label = Object.keys(logItem.event)[0];

  switch (label) {
    case "SavePoolAddress": return <AddressLink address={logItem.ok[0]} />
    case "Swap": return <TxLink tx={logItem.ok[0]} />
    case "Approve": return <TxLink tx={logItem.ok[0]} />
  }
}

function ErrorMessage({ logItem }: { logItem: LogItem }) {
  if (logItem.err.length === 0 || logItem.ok.length > 0) return;

  return <div className="flex flex-col gap-2">
    <div className="leading-relaxed">
      <Label logItem={logItem} />
      {logItem.err[0]}
    </div>
    <Timestamp logItem={logItem} />
  </div>
}

function Timestamp({ logItem }: { logItem: LogItem }) {
  return <div className="text-xs text-muted-foreground/50">1 hour ago</div>
}

function Label({ logItem }: { logItem: LogItem }) {
  const labelColour = logItem.err.length > 0 ? "bg-destructive" : "bg-primary/50 text-primary-foreground";

  return <div className={cn(labelColour, "inline px-[0.3rem] py-[0.2rem] rounded-sm mr-1")}>{Object.keys(logItem.event)[0]}</div>
}

function LogItemRow({ logItem }: { logItem: LogItem }) {
  if (logItem.err.length > 0) {
    return <ErrorMessage logItem={logItem} />
  }

  return <div className="flex flex-col gap-2">
    <div className="flex w-full justify-between items-center">
      <Label logItem={logItem} />
      <OkMessage logItem={logItem} />
    </div>
    <Timestamp logItem={logItem} />
  </div >
}

export function EventLog() {
  const { data: eventLog, isPending: isFetchingEventLog } = useEventLog();

  if (isFetchingEventLog || !eventLog) {
    return <Skeleton className="h-[19px] w-[125px] inline-block" />
  }

  return <section className="flex flex-col gap-5 w-full text-muted-foreground text-sm max-w-[400px]">
    {eventLog.map((logItem) => <LogItemRow logItem={logItem} />)}
  </section >
}
