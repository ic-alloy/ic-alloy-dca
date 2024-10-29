import { LoaderCircle, Send } from "lucide-react";
import { Button } from "./ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "./ui/input";
import useEthAddress from "@/hooks/useEthAddress";
import { useMutation } from "@tanstack/react-query";
import { useActor } from "@/actor";
import { decimalStringToEth } from "@/lib/eth";
import useHandleAgentError from "@/hooks/useHandleAgentError";
import { queryClient } from "@/main";

export default function SendButton() {
  const { isPending: isFetchingAddress } = useEthAddress();
  const { actor: backend } = useActor();
  const { handleAgentError } = useHandleAgentError();
  const { mutate: sendEth, isPending: isSending, isError, data: sendResult } = useMutation({
    mutationFn: async ({ to, amount }: { to: string, amount: string }) => {
      if (!backend) {
        throw new Error('backend actor not initialized');
      }
      try {
        const result = await backend.send_eth(to, decimalStringToEth(amount));
        // Refresh the balance in 5 seconds to give the Etherscan API time to catch up.
        // A better way to update balace would of course be:
        // 1. Parse response and check that transaction was successful
        // 2. Update balance manually, no API calls required.
        setTimeout(() => {
          queryClient.invalidateQueries({ queryKey: ['balance'] });
        }, 5000);
        return result;
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
    }
  });

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    sendEth({
      to: event.currentTarget.toAddress.value,
      amount: event.currentTarget.amount.value,
    });
  };

  return <Dialog >
    <DialogTrigger asChild>
      <Button disabled={isFetchingAddress} className="flex flex-col h-30 w-full items-start gap-1">
        <Send className="w-5 h-5" />Send
      </Button>
    </DialogTrigger>
    <DialogContent className='w-[400px]'>
      <DialogHeader>
        <DialogTitle>Send</DialogTitle>
      </DialogHeader>
      <form onSubmit={handleSubmit} className="flex flex-col gap-3">
        <Input type='text' placeholder='To address' name="toAddress" data-1p-ignore />
        <Input type='text' placeholder='Amount' name="amount" data-1p-ignore />
        <Button disabled={isSending} type="submit">{isSending ? <><LoaderCircle className="animate-spin w-4 h-4 mr-1" />Sending ...</> : "Send"}</Button>
        {isError &&
          <div className="font-semibold bg-destructive/30 rounded-lg p-2 text-destructive-foreground">
            There was an error sending ETH.
          </div>
        }
        {sendResult &&
          <pre className="bg-muted text-xs rounded-lg p-2 whitespace-pre-wrap break-all break-words box-border overflow-x-auto text-left">
            {JSON.stringify(sendResult)}
          </pre>
        }
      </form>
    </DialogContent>
  </Dialog>
}

