import { useQuery } from '@tanstack/react-query';
import { agent } from '../../agent/declarations';

export default function useCanisterAddress() {
  return useQuery({
    queryKey: ['get_canister_address'],
    queryFn: async () => {
      try {
        const result = await agent.get_canister_address();

        if (result === undefined) {
          throw new Error('Undefined canister address returned.');
        }

        if ('Err' in result) {
          throw new Error(result.Err);
        }

        return result.Ok;
      } catch (e) {
        console.error(e);
        throw new Error('Invalid canister address returned.');
      }
    },
    enabled: !!agent,
  });
}
