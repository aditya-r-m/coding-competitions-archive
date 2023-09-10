import Data.List (intersperse, sort)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

optimalAssignmentCost :: Int -> [Int] -> Int
optimalAssignmentCost k frequencies = sum
    <| map (uncurry (*))
    <| zip (reverse <| sort frequencies)
    <| [1..] >>= replicate k

solve :: IO String
solve = do
    line <- getLine
    let [_, k, _] = map (read :: String -> Int) <| words line
    line <- getLine
    let frequencies = map (read :: String -> Int) <| words line
    return <| show <| optimalAssignmentCost k frequencies

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
