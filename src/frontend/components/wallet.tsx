import ReceiveButton from "./receive-button";
import SendButton from "./send-button";
import { EthAddress } from "./eth-address";
import { Balance } from "./balance";
import Logout from "./logout";
import { IcpAddress } from "./icp-address";

export default function Wallet() {
  return <section className="flex flex-col gap-5">
    <div className="flex justify-between items-center">
      <h3>Wallet</h3>
      <Logout />
    </div>
    <div className="flex items-center gap-2">ETH:<EthAddress /></div>
    <div className="flex items-center gap-2">ICP:<IcpAddress /></div>
    <Balance />
    <div className="flex gap-5">
      <ReceiveButton />
      <SendButton />
    </div>
  </section>

}
