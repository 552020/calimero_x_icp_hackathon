import Layout from './components/layout/Layout';
import Leaderboard from './components/layout/Leaderboard';
// import Leaderboard from './components/Leaderboard';
// import Mempool from './components/Mempool';
// import WalletConnect from './components/WalletConnect';
import Mempool from './components/features/Mempool';
import WalletConnect from './components/features/WalletConnect';

// App.jsx
const App = () => {
  return (
    <Layout>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
        <Leaderboard />
        <Mempool />
        <WalletConnect isOpen={true} onClose={() => {}} />
      </div>
    </Layout>
  );
};

export default App;
