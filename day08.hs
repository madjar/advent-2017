#!/usr/bin/env stack
-- stack script --resolver nightly-2017-12-15 --package megaparsec --package text --package containers
{-# LANGUAGE OverloadedStrings #-}

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

type Parser = Parsec Void T.Text

main = print . doit . parse' parser =<< T.readFile "inputs/08"

parse' :: Parser a -> Text -> a
parse' p input = case parse p "" input of
                   Left err -> error (parseErrorPretty' input err)
                   Right x -> x


parser :: Parser [(Text, Int -> Int, Text, Int -> Bool)]
parser = many line <* eof
  where line = do
          reg <- register
          op <- lexeme ((+) <$ string "inc" <|> (-) <$ string "dec")
          value <- number
          lexeme (string "if")
          condReg <- register
          cond <- lexeme ((/=) <$ string "!="
                          <|> (==) <$ string "=="
                          <|> (<=) <$ string "<="
                          <|> (>=) <$ string ">="
                          <|> (<) <$ string "<"
                          <|> (>) <$ string ">"
                          )
          condValue <- number
          return (reg, (`op` value), condReg, (`cond` condValue))

        register = T.pack <$> lexeme (many letterChar)
        number = lexeme (L.signed sc L.decimal)
        sc = L.space space1 empty empty
        lexeme = L.lexeme sc

doit ::  [(Text, Int -> Int, Text, Int -> Bool)] -> (Int, Int)
doit instructions = (part1, part2)
  where part1 = maximum . M.elems . foldl' apply M.empty $ instructions
        part2 = maximum . concat . map M.elems . scanl' apply M.empty $ instructions
        apply state (reg, op, condReg, cond) =
          if cond (M.findWithDefault 0 condReg state)
            then M.alter (Just . op . fromMaybe 0) reg state
            else state
        apply :: M.Map Text Int -> (Text, Int -> Int, Text, Int -> Bool) -> M.Map Text Int
