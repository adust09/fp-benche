module Main where

import Algorithms.MergeSort (mergeSort)
import Control.DeepSeq (rnf)
import Control.Exception (evaluate)
import qualified Data.Vector as V
import Options.Applicative

newtype Args = Args { argN :: Int }

argsParser :: ParserInfo Args
argsParser = info (Args <$> option auto (long "n" <> metavar "N") <**> helper) mempty

main :: IO ()
main = do
    Args n <- execParser argsParser
    let input = V.fromList [n, n-1 .. 1]
        sorted = mergeSort input
    _ <- evaluate $ rnf sorted
    print (V.length sorted)
