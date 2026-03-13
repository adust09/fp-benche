module Main where

import Algorithms.Sieve (sieveCount)
import Options.Applicative

newtype Args = Args { argN :: Int }

argsParser :: ParserInfo Args
argsParser = info (Args <$> option auto (long "n" <> metavar "N") <**> helper) mempty

main :: IO ()
main = do
    Args n <- execParser argsParser
    print (sieveCount n)
