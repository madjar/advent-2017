#!/usr/bin/env stack
-- stack script --resolver nightly-2017-12-15 --package megaparsec --package text --package containers
{-# LANGUAGE OverloadedStrings, PartialTypeSignatures #-}

import Text.Megaparsec
import Text.Megaparsec.Char
import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Text.IO as T
import Data.Void
import qualified Text.Megaparsec.Char.Lexer as L
import Control.Applicative
import qualified Data.Map as M
import Data.Maybe
import Data.List
import Data.Tree as Tree
import Control.Monad
import Data.Foldable
import Data.Either

type Parser = Parsec Void T.Text

main = print . doit . parse' parser =<< T.readFile "inputs/09"

parse' :: Parser a -> Text -> a
parse' p input = case parse p "" input of
                   Left err -> error (parseErrorPretty' input err)
                   Right x -> x

parser :: Parser (Tree (Either Text ()))
parser = group <* eof
  where
    group = Node (Right ()) <$> (between (string "{") (string "}") groups)
    garbage =
      pure . Left . T.pack <$>
      (string "<" *> cancelled *>
       manyTill (anyChar <* cancelled) (string ">"))
    cancelled = skipMany (char '!' *> anyChar)
    groups = (garbage <|> group) `sepBy` (string ",")


doit i = (sum . score 1 $ i, countGarbage i)
  where score :: Int -> Tree (Either Text ()) -> Tree Int
        score n (Node (Right ()) children) = Node n (map (score (n+1)) children)
        score _ (Node (Left _) _) = Node 0 []
        countGarbage = sum . map T.length . lefts . toList
