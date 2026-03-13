module Main where

import Concurrency.ChannelThroughput (channelRoundtrip)
import Options.Applicative
import Data.Word (Word64)

newtype Args = Args { argN :: Word64 }

argsParser :: ParserInfo Args
argsParser = info (Args <$> option auto (long "n" <> metavar "N") <**> helper) mempty

main :: IO ()
main = do
    Args n <- execParser argsParser
    result <- channelRoundtrip n
    print result
