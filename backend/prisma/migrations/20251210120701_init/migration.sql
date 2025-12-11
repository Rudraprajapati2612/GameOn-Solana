-- CreateEnum
CREATE TYPE "GameType" AS ENUM ('BTC_ONLY', 'SOL_ONLY', 'BTC_VS_SOL');

-- CreateEnum
CREATE TYPE "GameStatus" AS ENUM ('PENDING', 'ACTIVE', 'COMPLETED', 'CANCELLED');

-- CreateEnum
CREATE TYPE "RoundType" AS ENUM ('PRICE_DIRECTION', 'MAGNITUDE', 'COMPARATIVE', 'RANGE', 'TREND');

-- CreateEnum
CREATE TYPE "TransactionType" AS ENUM ('DEPOSIT', 'WITHDRAWAL', 'JOIN_GAME', 'CLAIM_PRIZE', 'PLATFORM_FEE', 'CREATE_GAME', 'EVALUATE_ROUND', 'FETCH_PRICE');

-- CreateEnum
CREATE TYPE "TxStatus" AS ENUM ('PENDING', 'CONFIRMED', 'FAILED');

-- CreateEnum
CREATE TYPE "PriceType" AS ENUM ('START', 'END');

-- CreateEnum
CREATE TYPE "LogLevel" AS ENUM ('DEBUG', 'INFO', 'WARN', 'ERROR', 'FATAL');

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "walletAddress" TEXT NOT NULL,
    "username" TEXT,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Balance" (
    "id" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "degenBalance" DECIMAL(20,2) NOT NULL DEFAULT 0,
    "solBalance" DECIMAL(20,9) NOT NULL DEFAULT 0,
    "totalDeposits" DECIMAL(20,2) NOT NULL DEFAULT 0,
    "totalWithdrawals" DECIMAL(20,2) NOT NULL DEFAULT 0,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Balance_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Game" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "gameType" "GameType" NOT NULL,
    "status" "GameStatus" NOT NULL DEFAULT 'PENDING',
    "startTime" TIMESTAMP(3) NOT NULL,
    "actualStartTime" TIMESTAMP(3),
    "endTime" TIMESTAMP(3),
    "entryFee" DECIMAL(20,2) NOT NULL,
    "prizePool" DECIMAL(20,2) NOT NULL DEFAULT 0,
    "totalPlayers" INTEGER NOT NULL DEFAULT 0,
    "maxPlayers" INTEGER NOT NULL DEFAULT 50,
    "currentRound" INTEGER NOT NULL DEFAULT 0,
    "totalRounds" INTEGER NOT NULL DEFAULT 5,
    "leaderboardFinalized" BOOLEAN NOT NULL DEFAULT false,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Game_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PlayerGame" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "entrySlot" INTEGER NOT NULL,
    "totalScore" INTEGER NOT NULL DEFAULT 0,
    "roundsEvaluated" INTEGER NOT NULL DEFAULT 0,
    "finalRank" INTEGER,
    "prizeAmount" DECIMAL(20,2) NOT NULL DEFAULT 0,
    "prizeClaimed" BOOLEAN NOT NULL DEFAULT false,
    "totalResponseTime" BIGINT NOT NULL DEFAULT 0,
    "avgResponseTime" INTEGER NOT NULL DEFAULT 0,
    "firstPredictionAt" TIMESTAMP(3),
    "joinedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "PlayerGame_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Round" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "roundNumber" INTEGER NOT NULL,
    "roundType" "RoundType" NOT NULL,
    "startTime" TIMESTAMP(3) NOT NULL,
    "endTime" TIMESTAMP(3) NOT NULL,
    "evaluatedAt" TIMESTAMP(3),
    "startPriceBtc" DECIMAL(20,2),
    "endPriceBtc" DECIMAL(20,2),
    "startPriceSol" DECIMAL(20,4),
    "endPriceSol" DECIMAL(20,4),
    "correctAnswer" TEXT,
    "totalPredictions" INTEGER NOT NULL DEFAULT 0,
    "correctPredictions" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "Round_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Prediction" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "roundNumber" INTEGER NOT NULL,
    "userId" TEXT NOT NULL,
    "playerGameId" TEXT NOT NULL,
    "choice" TEXT NOT NULL,
    "submittedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "responseTime" INTEGER NOT NULL,
    "pointsEarned" INTEGER NOT NULL DEFAULT 0,
    "isCorrect" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Prediction_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PrizeDistribution" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "rank" INTEGER NOT NULL,
    "walletAddress" TEXT NOT NULL,
    "amount" DECIMAL(20,2) NOT NULL,
    "claimed" BOOLEAN NOT NULL DEFAULT false,
    "claimedAt" TIMESTAMP(3),
    "transactionHash" TEXT,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "PrizeDistribution_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Transaction" (
    "id" TEXT NOT NULL,
    "userId" TEXT,
    "type" "TransactionType" NOT NULL,
    "signature" TEXT NOT NULL,
    "status" "TxStatus" NOT NULL DEFAULT 'PENDING',
    "amount" DECIMAL(20,2),
    "retries" INTEGER NOT NULL DEFAULT 0,
    "maxRetries" INTEGER NOT NULL DEFAULT 3,
    "error" TEXT,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "confirmedAt" TIMESTAMP(3),

    CONSTRAINT "Transaction_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "BlockchainEvent" (
    "id" TEXT NOT NULL,
    "programId" TEXT NOT NULL,
    "eventType" TEXT NOT NULL,
    "gameId" TEXT,
    "userId" TEXT,
    "data" JSONB NOT NULL,
    "slot" TEXT NOT NULL,
    "signature" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,
    "processed" BOOLEAN NOT NULL DEFAULT false,
    "processedAt" TIMESTAMP(3),
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "BlockchainEvent_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "OraclePrice" (
    "id" TEXT NOT NULL,
    "gameId" TEXT NOT NULL,
    "roundNumber" INTEGER NOT NULL,
    "asset" TEXT NOT NULL,
    "priceType" "PriceType" NOT NULL,
    "price" DECIMAL(20,4) NOT NULL,
    "confidence" DECIMAL(20,4) NOT NULL,
    "slot" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,
    "fetchedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "OraclePrice_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SystemLog" (
    "id" TEXT NOT NULL,
    "level" "LogLevel" NOT NULL,
    "service" TEXT NOT NULL,
    "message" TEXT NOT NULL,
    "metadata" JSONB,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "SystemLog_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_walletAddress_key" ON "User"("walletAddress");

-- CreateIndex
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");

-- CreateIndex
CREATE INDEX "User_walletAddress_idx" ON "User"("walletAddress");

-- CreateIndex
CREATE UNIQUE INDEX "Balance_userId_key" ON "Balance"("userId");

-- CreateIndex
CREATE UNIQUE INDEX "Game_gameId_key" ON "Game"("gameId");

-- CreateIndex
CREATE INDEX "Game_gameId_idx" ON "Game"("gameId");

-- CreateIndex
CREATE INDEX "Game_status_idx" ON "Game"("status");

-- CreateIndex
CREATE INDEX "Game_startTime_idx" ON "Game"("startTime");

-- CreateIndex
CREATE INDEX "PlayerGame_gameId_idx" ON "PlayerGame"("gameId");

-- CreateIndex
CREATE INDEX "PlayerGame_userId_idx" ON "PlayerGame"("userId");

-- CreateIndex
CREATE INDEX "PlayerGame_finalRank_idx" ON "PlayerGame"("finalRank");

-- CreateIndex
CREATE UNIQUE INDEX "PlayerGame_gameId_userId_key" ON "PlayerGame"("gameId", "userId");

-- CreateIndex
CREATE INDEX "Round_gameId_idx" ON "Round"("gameId");

-- CreateIndex
CREATE UNIQUE INDEX "Round_gameId_roundNumber_key" ON "Round"("gameId", "roundNumber");

-- CreateIndex
CREATE INDEX "Prediction_userId_idx" ON "Prediction"("userId");

-- CreateIndex
CREATE INDEX "Prediction_playerGameId_idx" ON "Prediction"("playerGameId");

-- CreateIndex
CREATE UNIQUE INDEX "Prediction_gameId_roundNumber_userId_key" ON "Prediction"("gameId", "roundNumber", "userId");

-- CreateIndex
CREATE INDEX "PrizeDistribution_walletAddress_idx" ON "PrizeDistribution"("walletAddress");

-- CreateIndex
CREATE UNIQUE INDEX "PrizeDistribution_gameId_rank_key" ON "PrizeDistribution"("gameId", "rank");

-- CreateIndex
CREATE UNIQUE INDEX "Transaction_signature_key" ON "Transaction"("signature");

-- CreateIndex
CREATE INDEX "Transaction_userId_idx" ON "Transaction"("userId");

-- CreateIndex
CREATE INDEX "Transaction_signature_idx" ON "Transaction"("signature");

-- CreateIndex
CREATE INDEX "Transaction_status_idx" ON "Transaction"("status");

-- CreateIndex
CREATE UNIQUE INDEX "BlockchainEvent_signature_key" ON "BlockchainEvent"("signature");

-- CreateIndex
CREATE INDEX "BlockchainEvent_programId_idx" ON "BlockchainEvent"("programId");

-- CreateIndex
CREATE INDEX "BlockchainEvent_eventType_idx" ON "BlockchainEvent"("eventType");

-- CreateIndex
CREATE INDEX "BlockchainEvent_gameId_idx" ON "BlockchainEvent"("gameId");

-- CreateIndex
CREATE INDEX "BlockchainEvent_processed_idx" ON "BlockchainEvent"("processed");

-- CreateIndex
CREATE INDEX "OraclePrice_gameId_roundNumber_idx" ON "OraclePrice"("gameId", "roundNumber");

-- CreateIndex
CREATE UNIQUE INDEX "OraclePrice_gameId_roundNumber_asset_priceType_key" ON "OraclePrice"("gameId", "roundNumber", "asset", "priceType");

-- CreateIndex
CREATE INDEX "SystemLog_level_idx" ON "SystemLog"("level");

-- CreateIndex
CREATE INDEX "SystemLog_service_idx" ON "SystemLog"("service");

-- CreateIndex
CREATE INDEX "SystemLog_createdAt_idx" ON "SystemLog"("createdAt");

-- AddForeignKey
ALTER TABLE "Balance" ADD CONSTRAINT "Balance_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PlayerGame" ADD CONSTRAINT "PlayerGame_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PlayerGame" ADD CONSTRAINT "PlayerGame_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Round" ADD CONSTRAINT "Round_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Prediction" ADD CONSTRAINT "Prediction_gameId_roundNumber_fkey" FOREIGN KEY ("gameId", "roundNumber") REFERENCES "Round"("gameId", "roundNumber") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Prediction" ADD CONSTRAINT "Prediction_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Prediction" ADD CONSTRAINT "Prediction_playerGameId_fkey" FOREIGN KEY ("playerGameId") REFERENCES "PlayerGame"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PrizeDistribution" ADD CONSTRAINT "PrizeDistribution_gameId_fkey" FOREIGN KEY ("gameId") REFERENCES "Game"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transaction" ADD CONSTRAINT "Transaction_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE;
