import { LogOut } from "lucide-react";
import { Button } from "./ui/button";
import { useInternetIdentity } from "ic-use-internet-identity";

export default function Logout() {
  const { clear } = useInternetIdentity();

  return <Button variant="ghost" onClick={clear}><LogOut className="w-4 h-4" /></Button>
}
