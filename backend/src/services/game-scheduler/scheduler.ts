import cron from 'node-cron';
import { Transaction, PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import { sendTransaction, loadBackendKeypair } from '../blockchain/blockchain.ts';
import { prisma } from '../../db/index.ts';
import * as crypto from 'crypto';

// ============================================
// 1. START SCHEDULER
// ============================================

export const startScheduler = () => {
  console.log('⏰ Starting game scheduler...');

  // Create games at 10 AM and 8 PM daily
  cron.schedule('0 10,20 * * *', async () => {
    console.log('🎮 Cron triggered - Creating game...');
    await createGame();
  });

  // Monitor games every minute
  cron.schedule('* * * * *', async () => {
    await monitorGames();
  });

  console.log('✅ Scheduler started (10 AM, 8 PM daily)');
};

// ============================================
// 2. CREATE GAME
// ============================================

const createGame = async () => {
  try {
    const gameId = `GAME_${Date.now()}_${crypto.randomBytes(4).toString('hex')}`;
    const startTime = new Date(Date.now() + 30 * 60 * 1000); // Starts in 30 minutes

    console.log(`Creating game ${gameId} for ${startTime}`);

    // Build transaction
    const tx = new Transaction();
    const instruction = buildCreateGameInstruction(gameId, startTime);
    tx.add(instruction);

    // Send transaction
    const signature = await sendTransaction(tx);

    // Store in database
    await prisma.game.create({
      data: {
        gameId,
        gameType: 'BTC_ONLY',
        status: 'PENDING',
        startTime,
        entryFee: 10000,
        maxPlayers: 50,
        totalRounds: 5,
      },
    });

    console.log(`✅ Game ${gameId} created! TX: ${signature}`);
  } catch (error) {
    console.error('❌ Error creating game:', error);
  }
};

// ============================================
// 3. BUILD CREATE GAME INSTRUCTION
// ============================================

const buildCreateGameInstruction = (gameId: string, startTime: Date): TransactionInstruction => {
  const GAME_PROGRAM_ID = new PublicKey(process.env.GAME_PROGRAM_ID || '');
  const authority = loadBackendKeypair().publicKey;

  // Derive game PDA
  const [gamePDA] = PublicKey.findProgramAddressSync(
    [Buffer.from('game'), Buffer.from(gameId)],
    GAME_PROGRAM_ID
  );

  // TODO: Replace with your actual program instruction
  // This is a placeholder - you'll need to use your Anchor IDL
  const instruction = new TransactionInstruction({
    programId: GAME_PROGRAM_ID,
    keys: [
      { pubkey: gamePDA, isSigner: false, isWritable: true },
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: Buffer.from([
      0, // Instruction discriminator
      ...Buffer.from(gameId),
    ]),
  });

  return instruction;
};

// ============================================
// 4. MONITOR GAMES
// ============================================

const monitorGames = async () => {
  const now = new Date();

  // Find games starting soon
  const pendingGames = await prisma.game.findMany({
    where: {
      status: 'PENDING',
      startTime: {
        gte: now,
        lte: new Date(now.getTime() + 5 * 60 * 1000), // Next 5 min
      },
    },
  });

  for (const game of pendingGames) {
    const timeLeft = game.startTime.getTime() - now.getTime();

    // Start game if time is up
    if (timeLeft <= 0) {
      await startGame(game);
    }
  }
};

// ============================================
// 5. START GAME
// ============================================

const startGame = async (game: any) => {
  try {
    console.log(`🚀 Starting game ${game.gameId}`);

    await prisma.game.update({
      where: { id: game.id },
      data: {
        status: 'ACTIVE',
        actualStartTime: new Date(),
        currentRound: 1,
      },
    });

    // Create first round
    await prisma.round.create({
      data: {
        gameId: game.id,
        roundNumber: 1,
        roundType: 'PRICE_DIRECTION',
        startTime: new Date(),
        endTime: new Date(Date.now() + 60 * 1000), // 60 seconds
      },
    });

    console.log(`✅ Game ${game.gameId} started!`);
  } catch (error) {
    console.error('Error starting game:', error);
  }
};