import { useQuery } from "@tanstack/react-query";
import { backend } from "../../backend/declarations";

export function useTokenInBalance() {
  return useQuery({
    queryKey: ['get_balance_in'],
    queryFn: async () => {
      try {
        const result = await backend.get_balance_in();

        if (result === undefined) {
          throw new Error("Undefined balance returned.");
        }

        if ('Err' in result) {
          throw new Error(result.Err);
        }

        return result.Ok;
      } catch (e) {
        console.error(e);
        throw new Error("Invalid balance returned.");
      }
    },
    enabled: !!backend
  })
}

export function useTokenOutBalance() {
  return useQuery({
    queryKey: ['get_balance_out'],
    queryFn: async () => {
      try {
        const result = await backend.get_balance_out();

        if (result === undefined) {
          throw new Error("Undefined balance returned.");
        }

        if ('Err' in result) {
          throw new Error(result.Err);
        }

        return result.Ok;
      } catch (e) {
        console.error(e);
        throw new Error("Invalid balance returned.");
      }
    },
    enabled: !!backend
  })
}
