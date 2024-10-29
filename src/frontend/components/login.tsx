import { useInternetIdentity } from "ic-use-internet-identity";
import { Button } from "./ui/button";

export default function LoginButton() {
  const { login, loginStatus } = useInternetIdentity();

  const disabled = loginStatus === "logging-in" || loginStatus === "success";
  const text = loginStatus === "logging-in" ? "Logging in..." : "Login";

  return <section className="flex flex-col gap-5">
    <h3> Login to wallet</h3>
    <Button onClick={login} disabled={disabled}>
      {text}
    </Button>
  </section>
}
