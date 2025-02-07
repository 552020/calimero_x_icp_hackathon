// components/features/WalletConnect.jsx
// import Win98Dialog from '../windows/Win98Dialog';
// import Win98Dialog from '../dialogs/Win98Dialog';
import Win98Dialog from '../dialogs/Win98Dialog';
const WalletConnect = ({
  isOpen,
  onClose,
}: {
  isOpen: boolean;
  onClose: () => void;
}) => (
  <Win98Dialog isOpen={isOpen} onClose={onClose} title="Connected Wallet">
    {/* Dialog content goes here */}
    <div>Your wallet content</div>
  </Win98Dialog>
);

export default WalletConnect;
