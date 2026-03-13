module Main where

import Concurrency.ParallelMap (parallelMapSum)
import qualified Data.Vector as V
import Options.Applicative
import Text.Printf (printf)

data Args = Args
  { argN :: !Int
  }

argsParser :: ParserInfo Args
argsParser = info (Args <$> option auto (long "n" <> metavar "N") <**> helper) mempty

main :: IO ()
main = do
    Args n <- execParser argsParser
    let input = V.fromList [1.0 .. fromIntegral n]
    let result = parallelMapSum input
    printf "%.6f\n" result
