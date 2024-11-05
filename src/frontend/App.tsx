import { EventLog } from './components/event-log';
import { Settings } from './components/settings';

export default function App() {
  return (
    <main>
      <div className='flex flex-col gap-5' >
        <h3 className='text-muted-foreground'>DCA</h3>
        <div className='text-muted-foreground'>A description about what this canister does.</div>
        <Settings />
        <EventLog />
      </div>
    </main >
  );
}
