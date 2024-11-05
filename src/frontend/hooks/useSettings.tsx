import { useQuery } from "@tanstack/react-query";
import { backend } from "../../backend/declarations";

export default function useEthAddress() {
  return useQuery({
    queryKey: ['settings'],
    queryFn: async () => {
      try {
        const result = await backend.get_settings();

        if (result === undefined) {
          throw new Error("Undefined settings returned.");
        }

        if ('Err' in result) {
          throw new Error(result.Err);
        }

        return result.Ok;
      } catch (e) {
        console.error(e);
        throw new Error("Invalid settings returned.");
      }
    },
    enabled: !!backend
  });
}
