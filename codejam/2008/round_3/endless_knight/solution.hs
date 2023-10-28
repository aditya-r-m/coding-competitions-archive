import Data.List (intersperse, scanl, sort)
import Prelude   hiding (negate)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x
(<|||) f g x y = f <| g x y

listToPair :: [a] -> (a, a)
listToPair [x, y] = (x, y)

powerSet :: [a] -> [[a]]
powerSet [] = [[]]
powerSet (x:xs) = concat [[ps, x:ps] | ps <- powerSet xs]

p :: Int
p = 10007

divp :: Int -> Int
divp = flip div p

modp :: Int -> Int
modp = flip mod p

negate :: Int -> Int
negate = modp <|| (p -)

extEuclid :: Int -> Int -> Int -> Int -> Int
extEuclid  _ q1  _  1 = q1
extEuclid q0 q1 r0 r1 = extEuclid
    q1 (modp <| q0 - d * q1)
    r1 (modp <| r0 - d * r1)
    where d = div r0 r1

inverse :: Int -> Int
inverse = extEuclid 0 1 p

factorials :: [Int]
factorials = scanl (modp <||| (*)) 1 [1..p]

c' :: Int -> Int -> Int
c' n r
    | n < r = 0
    | otherwise = modp <| (factorials!!n)
        * (inverse <| factorials!!r)
        * (inverse <| factorials!!(n - r))

c :: Int -> Int -> Int
c 0 r = fromEnum <| r == 0
c n r = modp <| c' (modp n) (modp r) * c (divp n) (divp r)

countKnightPaths :: Int -> Int -> Int
countKnightPaths x y
    | n < r || r < 0 || 0 < m = 0
    | otherwise = c n r
    where
    i = 2 * x - y
    m = mod i 3
    r = div i 3 -- 2(x - r) == y + r
    n = x - r

countKnightPathsWithCheckpoints :: [(Int, Int)] -> Int
countKnightPathsWithCheckpoints ( _ : []) = negate 1
countKnightPathsWithCheckpoints ((x0, y0) : (x1, y1) : ls) = negate
    <| countKnightPaths (x1 - x0) (y1 - y0)
     * countKnightPathsWithCheckpoints ((x1, y1) : ls)

solve :: IO String
solve = do
    [x, y, r] <- fmap (map (read :: String -> Int) <|| words) getLine
    rs <- fmap (map (listToPair
                <|| map (read :: String -> Int)
                <|| words))
        <| sequence
        <| replicate r getLine
    return
        <| show
        <| foldr (modp <||| (+)) 0
        <| map (countKnightPathsWithCheckpoints
            <|| map snd
            <|| sort
            <|| map (uncurry (+) >>= (,))
            <|| ((1,1):)
            <|| ((x,y):))
        <| powerSet rs

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
