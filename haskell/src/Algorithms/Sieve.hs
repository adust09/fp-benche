module Algorithms.Sieve
  ( sieveCount
  , sieveList
  ) where

import Control.Monad (forM_, when)
import Control.Monad.ST (runST)
import Data.Vector.Unboxed (Vector)
import qualified Data.Vector.Unboxed as V
import qualified Data.Vector.Unboxed.Mutable as MV
import Data.Word (Word8)

-- | Sieve of Eratosthenes using mutable unboxed Vector Word8.
-- Matches Rust's Vec<u8> approach.
-- Returns the count of primes up to n.
sieveCount :: Int -> Int
sieveCount n
  | n < 2     = 0
  | otherwise = V.sum $ V.map fromIntegral (sieveVec n)

-- | Returns all primes up to n.
sieveList :: Int -> [Int]
sieveList n
  | n < 2     = []
  | otherwise =
      let vec = sieveVec n
       in [ i | i <- [2..n], vec V.! i == 1 ]

-- | Internal: compute the sieve as a Word8 vector.
sieveVec :: Int -> Vector Word8
sieveVec n = runST $ do
    arr <- MV.replicate (n + 1) (1 :: Word8)
    MV.write arr 0 0
    MV.write arr 1 0
    let limit = floor (sqrt (fromIntegral n :: Double))
    forM_ [2..limit] $ \p -> do
        isPrime <- MV.read arr p
        when (isPrime == 1) $
            forM_ [p*p, p*p+p .. n] $ \multiple ->
                MV.write arr multiple 0
    V.freeze arr
