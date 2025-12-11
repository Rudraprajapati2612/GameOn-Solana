import { Transaction, PublicKey, TransactionInstruction, SystemProgram } from '@solana/web3.js';
import { sendTransaction, loadBackendKeypair } from '../blockchain/blockchain.ts';
import { prisma } from '../../db/index.ts';

const PYTH_API = 'https://hermes.pyth.network';
const BTC_FEED = process.env.PYTH_BTC_PRICE_FEED || '';
const SOL_FEED = process.env.PYTH_SOL_PRICE_FEED || '';

// ============================================
// 1. FETCH PRICE FROM PYTH
// ============================================

export const fetchPrice = async (asset: 'BTC' | 'SOL'): Promise<number> => {
  try {
    const feedId = asset === 'BTC' ? BTC_FEED : SOL_FEED;
    const url = `${PYTH_API}/api/latest_price_feeds?ids[]=${feedId}`;

    const response = await fetch(url);
    const data: unknown = await response.json();

    if (!Array.isArray(data) || !data[0]?.price) {
      throw new Error("Unexpected response format from Pyth API");
    }

    const priceData = data[0].price;
    const price = parseFloat(priceData.price);
    const exponent = priceData.expo;

    // Calculate actual price
    const actualPrice = price * Math.pow(10, exponent);

    console.log(`📊 ${asset} Price: $${actualPrice.toFixed(2)}`);

    return actualPrice;
  } catch (error) {
    console.error(`Error fetching ${asset} price:`, error);
    throw error;
  }
};

// ============================================
// 2. FETCH AND STORE START PRICE
// ============================================

export const fetchStartPrice = async (gameId: string, round: number, asset: 'BTC' | 'SOL') => {
  try {
    console.log(`📥 Fetching START price for ${asset} - Round ${round}`);

    const price = await fetchPrice(asset);

    // Store on-chain
    await storePriceOnChain(gameId, round, asset, 'START', price);

    // Store in database
    const game = await prisma.game.findUnique({ where: { gameId } });
    if (game) {
      await prisma.oraclePrice.create({
        data: {
          gameId: game.id,
          roundNumber: round,
          asset,
          priceType: 'START',
          price,
          confidence: 0,
          slot: Date.now().toString(),
          timestamp: new Date(),
        },
      });

      // Update round
      await prisma.round.updateMany({
        where: { gameId: game.id, roundNumber: round },
        data: { [asset === 'BTC' ? 'startPriceBtc' : 'startPriceSol']: price },
      });
    }

    console.log(`✅ START price stored: $${price.toFixed(2)}`);
    return price;
  } catch (error) {
    console.error('Error fetching start price:', error);
    throw error;
  }
};

// ============================================
// 3. FETCH AND STORE END PRICE
// ============================================

export const fetchEndPrice = async (gameId: string, round: number, asset: 'BTC' | 'SOL') => {
  try {
    console.log(`📥 Fetching END price for ${asset} - Round ${round}`);

    const price = await fetchPrice(asset);

    // Store on-chain
    await storePriceOnChain(gameId, round, asset, 'END', price);

    // Store in database
    const game = await prisma.game.findUnique({ where: { gameId } });
    if (game) {
      await prisma.oraclePrice.create({
        data: {
          gameId: game.id,
          roundNumber: round,
          asset,
          priceType: 'END',
          price,
          confidence: 0,
          slot: Date.now().toString(),
          timestamp: new Date(),
        },
      });

      // Update round
      await prisma.round.updateMany({
        where: { gameId: game.id, roundNumber: round },
        data: { [asset === 'BTC' ? 'endPriceBtc' : 'endPriceSol']: price },
      });
    }

    console.log(`✅ END price stored: $${price.toFixed(2)}`);
    return price;
  } catch (error) {
    console.error('Error fetching end price:', error);
    throw error;
  }
};

// ============================================
// 4. STORE PRICE ON-CHAIN
// ============================================

const storePriceOnChain = async (
  gameId: string,
  round: number,
  asset: 'BTC' | 'SOL',
  type: 'START' | 'END',
  price: number
) => {
  try {
    const ORACLE_PROGRAM_ID = new PublicKey(process.env.ORACLE_PROGRAM_ID || '');
    const authority = loadBackendKeypair().publicKey;

    // Derive price PDA
    const [pricePDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('price'),
        Buffer.from(gameId),
        Buffer.from([round]),
        Buffer.from(type.toLowerCase()),
      ],
      ORACLE_PROGRAM_ID
    );

    // TODO: Replace with your actual program instruction
    const instruction = new TransactionInstruction({
      programId: ORACLE_PROGRAM_ID,
      keys: [
        { pubkey: pricePDA, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: encodePriceData(price),
    });

    const tx = new Transaction().add(instruction);
    const signature = await sendTransaction(tx);

    console.log(`✅ Price stored on-chain. TX: ${signature}`);
  } catch (error) {
    console.error('Error storing price on-chain:', error);
  }
};

// ============================================
// 5. DETERMINE ROUND RESULT
// ============================================

export const determineResult = async (gameId: string, round: number): Promise<'UP' | 'DOWN' | null> => {
  try {
    const game = await prisma.game.findUnique({
      where: { gameId },
      include: { rounds: { where: { roundNumber: round } } },
    });

    if (!game || !game.rounds[0]) return null;

    const roundData = game.rounds[0];
    let startPrice = 0;
    let endPrice = 0;

    if (game.gameType === 'BTC_ONLY') {
      startPrice = roundData.startPriceBtc?.toNumber() || 0;
      endPrice = roundData.endPriceBtc?.toNumber() || 0;
    } else if (game.gameType === 'SOL_ONLY') {
      startPrice = roundData.startPriceSol?.toNumber() || 0;
      endPrice = roundData.endPriceSol?.toNumber() || 0;
    }

    if (!startPrice || !endPrice) return null;

    const result = endPrice > startPrice ? 'UP' : 'DOWN';

    console.log(`🎯 Round ${round} Result: ${result} (${startPrice} → ${endPrice})`);

    // Update round
    await prisma.round.updateMany({
      where: { gameId: game.id, roundNumber: round },
      data: { correctAnswer: result, evaluatedAt: new Date() },
    });

    return result;
  } catch (error) {
    console.error('Error determining result:', error);
    return null;
  }
};

// Helper: Encode price data
const encodePriceData = (price: number): Buffer => {
  const buffer = Buffer.alloc(16);
  const priceInLamports = Math.floor(price * 1e8);
  buffer.writeBigUInt64LE(BigInt(priceInLamports), 0);
  return buffer;
};