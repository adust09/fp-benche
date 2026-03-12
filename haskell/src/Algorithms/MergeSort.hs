module Algorithms.MergeSort
  ( mergeSort
  , mergeSortList
  ) where

import Data.Vector (Vector)
import qualified Data.Vector as V

-- | Idiomatic Haskell merge sort using Data.Vector (boxed).
-- Matches Rust's Vec<i64> as a contiguous array type.
mergeSort :: Vector Int -> Vector Int
mergeSort xs
  | V.length xs <= 1 = xs
  | otherwise =
      let mid = V.length xs `div` 2
          (left, right) = V.splitAt mid xs
       in merge (mergeSort left) (mergeSort right)

-- | Merge two sorted vectors into one.
merge :: Vector Int -> Vector Int -> Vector Int
merge left right = V.fromList $ go (V.toList left) (V.toList right)
  where
    go [] ys = ys
    go xs [] = xs
    go (x:xs) (y:ys)
      | x <= y    = x : go xs (y:ys)
      | otherwise  = y : go (x:xs) ys

-- | List-based merge sort for comparison.
mergeSortList :: [Int] -> [Int]
mergeSortList [] = []
mergeSortList [x] = [x]
mergeSortList xs =
    let mid = length xs `div` 2
        (left, right) = splitAt mid xs
     in mergeList (mergeSortList left) (mergeSortList right)

mergeList :: [Int] -> [Int] -> [Int]
mergeList [] ys = ys
mergeList xs [] = xs
mergeList (x:xs) (y:ys)
  | x <= y    = x : mergeList xs (y:ys)
  | otherwise  = y : mergeList (x:xs) ys
