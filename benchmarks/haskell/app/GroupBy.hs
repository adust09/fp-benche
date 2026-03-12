module Main where

import DataProcessing.GroupBy (groupByCategory)
import Control.DeepSeq (rnf)
import Control.Exception (evaluate)
import Options.Applicative

newtype Args = Args { argInput :: FilePath }

argsParser :: ParserInfo Args
argsParser = info (Args <$> strOption (long "input" <> metavar "FILE") <**> helper) mempty

main :: IO ()
main = do
    Args path <- execParser argsParser
    totals <- groupByCategory path
    _ <- evaluate (rnf totals)
    putStrLn $ show (length totals) ++ " categories"
