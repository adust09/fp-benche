module Main where

import DataProcessing.WordCount (countWords)
import qualified Data.HashMap.Strict as HM
import Options.Applicative

newtype Args = Args { argInput :: FilePath }

argsParser :: ParserInfo Args
argsParser = info (Args <$> strOption (long "input" <> metavar "FILE") <**> helper) mempty

main :: IO ()
main = do
    Args path <- execParser argsParser
    counts <- countWords path
    let totalWords = sum (HM.elems counts)
    putStrLn $ "unique=" ++ show (HM.size counts) ++ " total=" ++ show totalWords
