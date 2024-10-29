import { useActor } from "@/actor";
import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";

export default function useEthBalance(address?: string) {
  const { actor: backend } = useActor();
  const { handleAgentError } = useHandleAgentError();
  return useQuery({
    queryKey: ['balance', address],
    queryFn: async () => {
      const response = await fetch(`https://api-sepolia.etherscan.io/api?module=account&action=balance&address=${address}&tag=latest&apikey=${import.meta.env.VITE_ETHERSCAN_API_KEY}`);
      const json = await response.json();
      try {
        if (json === undefined) {
          throw new Error("Undefined balance returned.")
        }
        if ('result' in json) {
          return json.result;
        }
        return "0";
      } catch (e) {
        handleAgentError(e);
        console.error(e);
        throw new Error("Invalid balance returned.")
      }
    },
    enabled: !!backend && !!address

  })
}


