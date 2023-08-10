import Data.List (intersperse, sort)

infixr 0 <|
(<|) f = f
(<||) f g x = f <| g x

solve :: IO String
solve = do
    xs <- fmap (map (read :: String -> Integer) <|| words) getLine
    ys <- fmap (map (read :: String -> Integer) <|| words) getLine
    return
        <| show
        <| sum
        <| map (uncurry (*)) <| zip (sort xs) (reverse <| sort ys)

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate (getLine >> solve)
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
