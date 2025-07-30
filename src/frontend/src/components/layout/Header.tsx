import EthBadge from "./EthBadge";
import SessionButton from "./SessionButton";

export default function Header() {
  return (
    <header className="border-b p-4">
      <div className="flex justify-end">
        <EthBadge />
        <SessionButton />
      </div>
    </header>
  );
}
