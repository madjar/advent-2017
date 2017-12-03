#!/usr/bin/env stack
-- stack --install-ghc runghc --package extra

import System.IO
import Data.List.Extra

main = interact solve
solve = show . sum . map fst . filter (uncurry (==)) . halfwayAround . map (read . return) . trim
pairs l@(x:xs) = zip l (xs ++ [x])
halfwayAround l = zip l (end ++ begin)
  where (begin, end) = splitAt (length l `div` 2) l
