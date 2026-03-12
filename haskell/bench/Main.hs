module Main where

import Criterion.Main
import Control.DeepSeq (NFData(..), force)
import Data.Word (Word64)
import qualified Data.Vector as V

import Algorithms.Fibonacci (fibMemo, fibNaive)
import Algorithms.MergeSort (mergeSort)
import Algorithms.Sieve (sieveCount)
import Concurrency.ParallelMap (parallelMapSum, sequentialMapSum)

main :: IO ()
main = defaultMain
    [ bgroup "fibonacci-naive"
        [ bench "30" $ nf fibNaive (30 :: Word64)
        , bench "35" $ nf fibNaive (35 :: Word64)
        , bench "40" $ nf fibNaive (40 :: Word64)
        ]
    , bgroup "fibonacci-memo"
        [ bench "10000"  $ nf fibMemo (10000 :: Word64)
        , bench "100000" $ nf fibMemo (100000 :: Word64)
        ]
    , bgroup "merge-sort"
        [ env (pure $ V.fromList [1000, 999 .. 1 :: Int]) $ \v ->
            bench "1000" $ nf mergeSort v
        , env (pure $ V.fromList [10000, 9999 .. 1 :: Int]) $ \v ->
            bench "10000" $ nf mergeSort v
        , env (pure $ V.fromList [100000, 99999 .. 1 :: Int]) $ \v ->
            bench "100000" $ nf mergeSort v
        ]
    , bgroup "sieve"
        [ bench "100000"   $ nf sieveCount 100000
        , bench "1000000"  $ nf sieveCount 1000000
        , bench "10000000" $ nf sieveCount 10000000
        ]
    , bgroup "parallel-map"
        [ env (pure $ V.fromList [1.0 .. 100000.0]) $ \v ->
            bench "100000" $ nf parallelMapSum v
        , env (pure $ V.fromList [1.0 .. 1000000.0]) $ \v ->
            bench "1000000" $ nf parallelMapSum v
        ]
    , bgroup "sequential-map"
        [ env (pure $ V.fromList [1.0 .. 100000.0]) $ \v ->
            bench "100000" $ nf sequentialMapSum v
        , env (pure $ V.fromList [1.0 .. 1000000.0]) $ \v ->
            bench "1000000" $ nf sequentialMapSum v
        ]
    ]
