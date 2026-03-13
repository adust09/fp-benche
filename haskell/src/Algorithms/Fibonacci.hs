module Algorithms.Fibonacci
  ( fibNaive
  , fibMemo
  ) where

import Data.Word (Word64)

-- | Naive recursive fibonacci. No memoization.
-- Uses Word64 for wrapping arithmetic (matches Rust u64).
fibNaive :: Word64 -> Word64
fibNaive 0 = 0
fibNaive 1 = 1
fibNaive n = fibNaive (n - 1) + fibNaive (n - 2)

-- | Idiomatic Haskell: lazy infinite list with zipWith.
-- Memoization emerges naturally from laziness.
fibMemo :: Word64 -> Word64
fibMemo n = fibs !! fromIntegral n
  where
    fibs :: [Word64]
    fibs = 0 : 1 : zipWith (+) fibs (tail fibs)
