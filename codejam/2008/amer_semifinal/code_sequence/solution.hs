import Data.List (intersperse, nub, partition, sort)

infixr 0 <|
(<|) f = f
(<||) f g x = f <| g x

modp :: Int -> Int
modp = flip mod 10007

bifurcate :: [Int] -> ([Int], [Int])
bifurcate = (fmap (,) (map snd <|| fst) <*> (map snd <|| snd))
    <|| partition (even <|| fst)
    <|| zip [0..]

previousDiff :: [Int] -> Maybe Int
previousDiff diffs
    | length evenDiffs <= 1 = Nothing
    | varyingEvenDiffs = Just <| head oddDiffs
    | otherwise = previousDiff oddDiffs
    where
        (evenDiffs, oddDiffs) = bifurcate diffs
        varyingEvenDiffs = (>1) <| length <| nub evenDiffs

nextTerm :: [Int] -> Maybe Int
nextTerm vs = (fmap <| modp <|| (+ last vs))
    <| previousDiff
    <| reverse
    <| map (modp <|| uncurry (-))
    <| (tail >>= zip) vs

format :: Maybe Int -> String
format Nothing  = "UNKNOWN"
format (Just v) = show v

solve :: IO String
solve = getLine >> fmap (map (read :: String -> Int) <|| words) getLine
    >>= return
        <|| format
        <|| nextTerm

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
