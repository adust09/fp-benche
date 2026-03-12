module Main where

import DataProcessing.JsonFilter (filterUsers)
import Control.DeepSeq (rnf)
import Control.Exception (evaluate)
import Options.Applicative

newtype Args = Args { argInput :: FilePath }

argsParser :: ParserInfo Args
argsParser = info (Args <$> strOption (long "input" <> metavar "FILE") <**> helper) mempty

main :: IO ()
main = do
    Args path <- execParser argsParser
    users <- filterUsers path
    _ <- evaluate (rnf users)
    print (length users)
