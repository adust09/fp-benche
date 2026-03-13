{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE DeriveAnyClass #-}
{-# LANGUAGE OverloadedStrings #-}

module DataProcessing.JsonFilter
  ( User(..)
  , filterUsers
  , filterUsersBS
  ) where

import Control.DeepSeq (NFData)
import Data.Aeson (FromJSON(..), eitherDecodeStrict', withObject, (.:))
import qualified Data.ByteString as BS
import GHC.Generics (Generic)

data User = User
  { userId     :: !Int
  , userName   :: !String
  , userAge    :: !Int
  , userCity   :: !String
  , userActive :: !Bool
  } deriving (Show, Generic, NFData)

instance FromJSON User where
    parseJSON = withObject "User" $ \v -> User
        <$> v .: "id"
        <*> v .: "name"
        <*> v .: "age"
        <*> v .: "city"
        <*> v .: "active"

-- | Filter active users aged >= 30 from a JSON file.
filterUsers :: FilePath -> IO [User]
filterUsers path = do
    bs <- BS.readFile path
    pure $! filterUsersBS bs

-- | In-memory version for benchmarking.
filterUsersBS :: BS.ByteString -> [User]
filterUsersBS bs =
    case eitherDecodeStrict' bs of
        Left err    -> error $ "JSON parse error: " ++ err
        Right users -> filter predicate users
  where
    predicate u = userActive u && userAge u >= 30
