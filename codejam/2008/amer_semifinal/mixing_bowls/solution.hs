import Data.List (intersperse, sort)
import Data.Map  (Map, empty, insert, lookup)
import Prelude   hiding (lookup)

infixr 0 <|
(<|) f = f
(<||) f g x = f <| g x

renameRoot :: [String] -> [String]
renameRoot (r:rs) = (concat <| intersperse " " <| ("ROOT":) <| tail <| words r):rs

parseTree :: [String] -> Map String [String]
parseTree = foldr insertEntry empty
    where insertEntry s m = insert (head <| words s) (drop 2 <| words s) m

minimumBowls :: String -> Map String [String] -> Int
minimumBowls node tree = case lookup node tree of
    Nothing -> 0
    Just children -> spareBowlsAdded + if last spareBowls > 0 then 0 else 1
        where
            spareBowls = scanl ((flip (-) 1 <||) <|| max) 0
                <| reverse <| sort <| filter (>0) <| map (flip minimumBowls tree) children
            spareBowlsAdded = sum
                <| map ((+1) <|| uncurry (-)) <| (tail >>= zip) spareBowls

solve :: IO String
solve = (readLn :: IO Int)
    >>= sequence <|| flip replicate getLine
    >>= return <|| show <|| minimumBowls "ROOT" <|| parseTree <|| renameRoot

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
