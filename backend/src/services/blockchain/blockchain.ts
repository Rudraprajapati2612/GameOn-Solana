import { 
    Connection, 
    Keypair, 
    PublicKey, 
    Transaction,
    sendAndConfirmTransaction,
    type Logs  // ✅ FIX: Use type-only import
  } from '@solana/web3.js';
  
  import fs from 'fs';
  import { prisma } from '../../db/index';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';
  
  // ============================================
  // 1. SETUP CONNECTION
  // ============================================
  
  const RPC_URL = process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com';
  const WS_URL = process.env.SOLANA_WS_URL || 'wss://api.devnet.solana.com';
  
  export const connection = new Connection(RPC_URL, 'confirmed');
  
  console.log('✅ Connected to Solana:', RPC_URL);
  
  // ============================================
  // 2. LOAD BACKEND KEYPAIR
  // ============================================
  
  let backendKeypair: Keypair;
  
  export const loadBackendKeypair = () => {
    if (backendKeypair) return backendKeypair;
  
    const baseKey58 = process.env.BACKEND_PRIVATE_KEY;

    if(!baseKey58){
      throw new Error("Backend Private  Key is missing ")
    }

    backendKeypair = Keypair.fromSecretKey(bs58.decode(baseKey58));

    console.log("backend key pair is loded ", backendKeypair.publicKey.toBase58());
    
    return backendKeypair;
  };
  
  // ============================================
  // 3. SEND TRANSACTION
  // ============================================
  
  export const sendTransaction = async (transaction: Transaction): Promise<string> => {
    try {
      const keypair = loadBackendKeypair();
      
      // Get recent blockhash
      const { blockhash } = await connection.getLatestBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = keypair.publicKey;
  
      // Sign and send
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [keypair],
        { commitment: 'confirmed' }
      );
  
      console.log('✅ Transaction sent:', signature);
      return signature;
  
    } catch (error: any) {
      console.error('❌ Transaction failed:', error.message);
      throw error;
    }
  };
  
  // ============================================
  // 4. LISTEN TO EVENTS
  // ============================================
  
  const PROGRAM_IDS = {
    VAULT: new PublicKey(process.env.VAULT_PROGRAM_ID || '11111111111111111111111111111111'),  // ✅ FIX: Default value
    GAME: new PublicKey(process.env.GAME_PROGRAM_ID || '11111111111111111111111111111111'),
    PRIZE: new PublicKey(process.env.PRIZE_PROGRAM_ID || '11111111111111111111111111111111'),
    ORACLE: new PublicKey(process.env.ORACLE_PROGRAM_ID || '11111111111111111111111111111111'),
  };
  
  export const startEventListener = () => {
    const wsConnection = new Connection(RPC_URL, 'confirmed');
  
    console.log('🎧 Starting event listener...');
  
    // Listen to each program
    Object.entries(PROGRAM_IDS).forEach(([name, programId]) => {
      // ✅ FIX: Check if programId is valid
      const defaultPubkey = '11111111111111111111111111111111';
      if (programId.toBase58() === defaultPubkey) {
        console.log(`   ⚠️ Skipping ${name}: No program ID set in .env`);
        return;
      }
  
      console.log(`   Listening to ${name}:`, programId.toBase58());
  
      wsConnection.onLogs(
        programId,
        async (logs: Logs) => {
          await handleEvent(name, logs);
        },
        'confirmed'
      );
    });
  
    console.log('✅ Event listener started');
  };
  
  // ============================================
  // 5. HANDLE EVENTS
  // ============================================
  
  const handleEvent = async (programName: string, logs: Logs) => {
    const { signature, err } = logs;
  
    if (err) {
      console.log('⚠️  Transaction failed:', signature);
      return;
    }
  
    // Parse events from logs
    const events = parseEvents(logs.logs);
  
    for (const event of events) {
      console.log(`📢 ${programName} Event:`, event.type, event.data);
  
      // Store in database
      await storeEvent(programName, event, signature);
  
      // Handle specific events
      await handleSpecificEvent(programName, event);
    }
  };
  
  // ============================================
  // 6. PARSE EVENTS FROM LOGS
  // ============================================
  
  const parseEvents = (logs: string[]): any[] => {
    const events: any[] = [];
  
    for (const log of logs) {
      // Look for: "Program log: EVENT_NAME: {json_data}"
      if (log.includes('Program log:')) {
        try {
          const match = log.match(/Program log: (\w+): (.+)/);
          if (match) {
            const [, eventType, eventData] = match;
            events.push({
              type: eventType,
              data: JSON.parse(eventData),
            });
          }
        } catch (e) {
          // Skip if can't parse
        }
      }
    }
  
    return events;
  };
  
  // ============================================
  // 7. STORE EVENT IN DATABASE
  // ============================================
  
  const storeEvent = async (programName: string, event: any, signature: string) => {
    try {
      await prisma.blockchainEvent.create({
        data: {
          programId: programName,
          eventType: event.type,
          gameId: event.data.game_id || null,
          userId: event.data.player || event.data.user || null,
          data: event.data,
          slot: Date.now().toString(),
          signature,
          timestamp: new Date(),
          processed: false,
        },
      });
    } catch (error) {
      console.error('Error storing event:', error);
    }
  };
  
  // ============================================
  // 8. HANDLE SPECIFIC EVENTS
  // ============================================
  
  const handleSpecificEvent = async (programName: string, event: any) => {
    const { type, data } = event;
  
    // ================= VAULT: Deposit =================
    if (programName === 'VAULT' && type === 'Deposit') {
      if (!data?.user || !data?.amount) return;
  
      const user = await prisma.user.upsert({
        where: { walletAddress: data.user as string },
        create: { walletAddress: data.user as string },
        update: {},
      });
  
      await prisma.balance.upsert({
        where: { userId: user.id },
        create: {
          userId: user.id,
          degenBalance: data.amount,
          totalDeposits: data.amount,
        },
        update: {
          degenBalance: { increment: data.amount },
          totalDeposits: { increment: data.amount },
        },
      });
  
      console.log(`💰 User ${data.user} deposited ${data.amount} DEGEN`);
      return;
    }
  
    // ================= GAME: PlayerJoined =================
    if (programName === 'GAME' && type === 'PlayerJoined') {
      if (!data?.player || !data?.game_id || data?.entry_slot == null) return;
  
      const user = await prisma.user.upsert({
        where: { walletAddress: data.player as string },
        create: { walletAddress: data.player as string },
        update: {},
      });
  
      const game = await prisma.game.findUnique({
        where: { gameId: data.game_id as string },
      });
  
      if (!game) return;
  
      await prisma.playerGame.create({
        data: {
          gameId: game.id,
          userId: user.id,
          entrySlot: Number(data.entry_slot),
        },
      });
  
      await prisma.game.update({
        where: { id: game.id },
        data: { totalPlayers: { increment: 1 } },
      });
  
      console.log(`🎮 Player ${data.player} joined game ${data.game_id}`);
      return;
    }
  
    // ================= GAME: PredictionSubmitted =================
    if (programName === 'GAME' && type === 'PredictionSubmitted') {
      if (!data?.player || !data?.game_id || data?.round == null) return;
  
      const user = await prisma.user.findUnique({
        where: { walletAddress: data.player as string },
      });
      if (!user) return;
  
      const game = await prisma.game.findUnique({
        where: { gameId: data.game_id as string },
      });
      if (!game) return;
  
      const playerGame = await prisma.playerGame.findFirst({
        where: {
          gameId: game.id,
          userId: user.id,
        },
      });
      if (!playerGame) return;
  
      await prisma.prediction.create({
        data: {
          gameId: game.id,
          roundNumber: Number(data.round),
          userId: user.id,
          playerGameId: playerGame.id,
          choice: JSON.stringify(data.choice),
          responseTime: Number(data.response_time ?? 0),
        },
      });
  
      console.log(`🎯 Player ${data.player} predicted for round ${data.round}`);
      return;
    }
  };