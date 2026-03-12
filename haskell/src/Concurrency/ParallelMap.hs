module Concurrency.ParallelMap
  ( parallelMapSum
  , sequentialMapSum
  ) where

import Control.Parallel.Strategies (parMap, rdeepseq)
import Data.Vector (Vector)
import qualified Data.Vector as V

-- | Parallel map: compute sqrt of each element using Strategies.
-- Returns the sum to force evaluation of all elements.
parallelMapSum :: Vector Double -> Double
parallelMapSum v =
    let xs = V.toList v
        results = parMap rdeepseq sqrt xs
     in sum results

-- | Sequential version for comparison.
sequentialMapSum :: Vector Double -> Double
sequentialMapSum = V.sum . V.map sqrt
