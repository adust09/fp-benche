module Main where

import Algorithms.Fibonacci (fibMemo, fibNaive)
import Control.DeepSeq (force)
import Control.Exception (evaluate)
import Options.Applicative

data Args = Args
  { argN    :: !Int
  , argMode :: !String
  }

argsParser :: ParserInfo Args
argsParser = info (args <**> helper) mempty
  where
    args = Args
        <$> option auto (long "n" <> metavar "N")
        <*> strOption (long "mode" <> value "iter" <> metavar "MODE")

main :: IO ()
main = do
    Args n mode <- execParser argsParser
    result <- evaluate . force $ case mode of
        "naive" -> fibNaive (fromIntegral n)
        "iter"  -> fibMemo (fromIntegral n)
        _       -> error $ "unknown mode: " ++ mode
    print result
