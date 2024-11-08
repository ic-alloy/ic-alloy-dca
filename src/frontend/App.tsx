import { Bot } from 'lucide-react';
import { Balance } from './components/balance';
import { EventLog } from './components/event-log';
import { Settings } from './components/settings';

export default function App() {
  return (
    <main>
      <div className='flex flex-col gap-5 max-w-[400px]' >
        <h3 className='text-foreground flex items-center'><Bot className='w-10 h-10 inline mr-2' /> DCA Agent</h3>
        <div className='text-foreground'>This agent implements a dollar cost average strategy to swap the in token for the out token on an interval. Swaps are made on UniSwap and tokens are held by an IC canister.</div>
        <Settings />
        <Balance />
        <EventLog />
      </div>
    </main >
  );
}
