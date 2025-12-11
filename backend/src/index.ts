import "dotenv/config";
import express from "express";
import cors from "cors";
import { loadBackendKeypair, startEventListener } from './services/blockchain/blockchain.ts';
import { startScheduler } from './services/game-scheduler/scheduler.ts';
const app  = express();

app.use(cors());
app.use(express.json());

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date() });
});



const start = async () => {
  try {
    console.log('🚀 Starting Backend...');

    // 1. Load backend keypair
    const keypair = loadBackendKeypair();
    console.log('   Backend Pubkey:', keypair.publicKey.toBase58());

    // 2. Start event listener
    startEventListener();

    // 3. Start game schedulerprocess.env.
    startScheduler();

    // 4. Start Express server
    app.listen(process.env.PORT, () => {
      console.log(`✅ Server running on port ${process.env.PORT}`);
      console.log(`   Health: http://localhost:${process.env.PORT}/health`);
    });

  } catch (error) {
    console.error('❌ Failed to start:', error);
    process.exit(1);
  }
};

// Shutdown
process.on('SIGTERM', () => {
  console.log('Shutting down...');
  process.exit(0);
});

// Start
start();