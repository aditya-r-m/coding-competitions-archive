{-

xi + yi + zi - pi T <= x + y + z <= xi + yi + zi + pi T
xi + yi - zi - pi T <= x + y - z <= xi + yi - zi + pi T
xi - yi + zi - pi T <= x - y + z <= xi - yi + zi + pi T
xi - yi - zi - pi T <= x - y - z <= xi - yi - zi + pi T

c1 <= x + y + z <= c0
c3 <= x + y - z <= c2
c5 <= x - y + z <= c4
c7 <= x - y - z <= c6

c1 - x <= y + z <= c0 - x
-c6 + x <= y + z <= -c7 + x
c3 - x <= y - z <= c2 - x
-c4 + x <= y - z <= -c5 + x

c1 <= c0
-c6 <= -c7
c3 <= c2
-c4 <= -c5

c1 + c7 <= 2x <= c0 + c6
c3 + c5 <= 2x <= c2 + c4

c1 + c7 <= c0 + c6
c3 + c5 <= c2 + c4
c1 + c7 <= c2 + c4
c3 + c5 <= c0 + c6

-}

import Data.List (intersperse)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

data Ship = Ship { x :: Double, y :: Double, z :: Double, p :: Double } deriving (Show)

c'' :: Double -> Int -> Ship -> Double
c'' t i ship
    = x ship
    + ((-1)^(div i 4)) * y ship
    + ((-1)^(div i 2)) * z ship
    + ((-1)^i) * t * p ship

c' :: [Ship] -> Double -> Int -> Double
c' ships t i
    | even i = foldr1 min <| map (c'' t i) ships
    | otherwise = foldr1 max <| map (c'' t i) ships

reachable :: [Ship] -> Double -> Bool
reachable ships t
    = c 0 >= c 1 && c 2 >= c 3 && c 4 >= c 5 && c 6 >= c 7
    && min (c 0 + c 6) (c 2 + c 4) >= max (c 1 + c 7) (c 3 + c 5)
    where c = c' ships t

upperBound :: [Ship] -> Double
upperBound ships = snd
    <| head
    <| dropWhile (not <|| uncurry reachable)
    <| map ((,) ships) (iterate (*2) 1)

binarySearch :: [Ship] -> Int -> Double -> Double -> Double
binarySearch ships p lb ub
    | p == 0 = m
    | reachable ships m = binarySearch ships (p - 1) lb m
    | otherwise = binarySearch ships (p - 1) m ub
    where m = ((lb + ub) / 2)

parse :: String -> Ship
parse s = Ship { x = ds!!0, y = ds!!1, z = ds!!2, p = ds!!3 }
    where ds = map (read :: String -> Double) <| words s

solve :: IO String
solve = do
    n <- readLn :: IO Int
    s <- sequence <| replicate n getLine
    let ships = map parse s
    return
        <| show
        <| binarySearch ships 127 0
        <| upperBound ships

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
