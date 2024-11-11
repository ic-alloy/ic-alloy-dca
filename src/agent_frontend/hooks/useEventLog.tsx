import { useQuery } from '@tanstack/react-query';
import { agent } from '../../agent/declarations';

export default function useEventLog() {
  return useQuery({
    queryKey: ['get_log'],
    queryFn: async () => {
      try {
        const result = await agent.get_log();

        if (result === undefined) {
          throw new Error('Undefined settings returned.');
        }

        return result.reverse();
      } catch (e) {
        console.error(e);
        throw new Error('Invalid settings returned.');
      }
    },
    enabled: !!agent,
  });
}
