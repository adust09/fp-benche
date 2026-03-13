module Concurrency.ChannelThroughput
  ( channelRoundtrip
  ) where

import Control.Concurrent (forkIO)
import Control.Concurrent.STM (atomically)
import Control.Concurrent.STM.TBQueue
    ( newTBQueueIO, readTBQueue, writeTBQueue )
import Control.Monad (forM_)
import Data.IORef (newIORef, readIORef, modifyIORef')
import Data.Word (Word64)

-- | Send n messages through a bounded TBQueue (STM).
-- Matches Rust's crossbeam bounded channel.
-- Returns the sum of received values.
channelRoundtrip :: Word64 -> IO Word64
channelRoundtrip n = do
    queue <- newTBQueueIO 1024
    sumRef <- newIORef (0 :: Word64)

    -- Sender thread
    _ <- forkIO $
        forM_ [0..n-1] $ \i ->
            atomically $ writeTBQueue queue i

    -- Receiver (on main thread)
    forM_ [1..n] $ \_ -> do
        val <- atomically $ readTBQueue queue
        modifyIORef' sumRef (+ val)

    readIORef sumRef
