module DataProcessing.WordCount
  ( countWords
  , countWordsBS
  ) where

import qualified Data.ByteString as BS
import Data.Char (isAlphaNum)
import Data.List (foldl')
import Data.HashMap.Strict (HashMap)
import qualified Data.HashMap.Strict as HM
import qualified Data.Text as T
import qualified Data.Text.Encoding as TE

-- | Count word frequencies from a file using strict ByteString + HashMap.
-- Matches Rust's BufReader + HashMap approach.
countWords :: FilePath -> IO (HashMap T.Text Int)
countWords path = do
    bs <- BS.readFile path
    pure $! countWordsBS bs

-- | In-memory version for benchmarking.
countWordsBS :: BS.ByteString -> HashMap T.Text Int
countWordsBS bs =
    let text = TE.decodeUtf8Lenient bs
        ws = T.words text
     in foldl' insertWord HM.empty ws
  where
    insertWord acc w =
        let cleaned = T.toLower $ T.filter isAlphaNum w
         in if T.null cleaned
              then acc
              else HM.insertWith (+) cleaned 1 acc
