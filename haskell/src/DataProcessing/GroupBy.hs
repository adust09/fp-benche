{-# LANGUAGE OverloadedStrings #-}

module DataProcessing.GroupBy
  ( CategoryTotal(..)
  , groupByCategory
  , groupByCategoryMem
  ) where

import Control.DeepSeq (NFData(..))
import Data.List (foldl')
import Data.HashMap.Strict (HashMap)
import qualified Data.HashMap.Strict as HM
import qualified Data.ByteString.Lazy as BL
import qualified Data.Csv as Csv
import qualified Data.Text as T
import qualified Data.Vector as V

data CategoryTotal = CategoryTotal
  { ctCategory :: !T.Text
  , ctTotal    :: !Double
  , ctCount    :: !Int
  } deriving (Show)

instance NFData CategoryTotal where
    rnf (CategoryTotal c t n) = c `seq` t `seq` n `seq` ()

-- | Read CSV and group by category, summing amounts.
-- Uses HashMap.Strict (matches Rust's HashMap).
groupByCategory :: FilePath -> IO [CategoryTotal]
groupByCategory path = do
    bs <- BL.readFile path
    case Csv.decode Csv.NoHeader bs of
        Left err -> error $ "CSV parse error: " ++ err
        Right records ->
            pure $! groupByCategoryMem (V.toList (records :: V.Vector (T.Text, Double)))

-- | In-memory version for benchmarking.
groupByCategoryMem :: [(T.Text, Double)] -> [CategoryTotal]
groupByCategoryMem records =
    let groups = foldl' accumulate HM.empty records
     in map toTotal (HM.toList groups)
  where
    accumulate :: HashMap T.Text (Double, Int) -> (T.Text, Double) -> HashMap T.Text (Double, Int)
    accumulate acc (cat, amount) =
        HM.insertWith addPair cat (amount, 1) acc

    addPair (a1, c1) (a2, c2) = (a1 + a2, c1 + c2)

    toTotal (cat, (total, count)) = CategoryTotal cat total count
