import Data.List (intersperse)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

triangleCoordinates :: [Int] -> Maybe ((Int, Int), (Int, Int))
triangleCoordinates [n, m, a]
    | a > n*m      = Nothing
    | mod a n == 0 = Just ((n, 0), (0, div a n))
    | otherwise    = Just ((x0, y0), (1, y1))
    where
        y1 = 1 + div a n
        x0 = 1 + div a y1
        y0 = y1*x0 - a

format :: Maybe ((Int, Int), (Int, Int)) -> String
format Nothing = "IMPOSSIBLE"
format (Just ((x0, y0), (x1, y1))) = "0 0 "
    ++ (concat
        <| intersperse " "
        <| map show [x0, y0, x1, y1])

solve :: IO String
solve = getLine
    >>= return
        <|| format
        <|| triangleCoordinates
        <|| map (read :: String -> Int)
        <|| words

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
