import { useQuery } from "@tanstack/react-query";
import { backend } from "../../backend/declarations";

export default function useEventLog() {
  return useQuery({
    queryKey: ['get_log'],
    queryFn: async () => {
      try {
        const result = await backend.get_log();

        if (result === undefined) {
          throw new Error("Undefined settings returned.");
        }

        return result.reverse();
      } catch (e) {
        console.error(e);
        throw new Error("Invalid settings returned.");
      }
    },
    enabled: !!backend
  });
}
