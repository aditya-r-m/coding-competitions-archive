import Prelude hiding (lookup)
import Data.List (intersperse)
import Data.Map (Map, empty, insertWith, fromList, toList, lookup)
import Data.Maybe (fromJust)
import Data.Function (on)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

m3 :: Int -> Int
m3 = (`mod` 3)

nc3 :: Int -> Int
nc3 n = div (n * (n - 1) * (n - 2)) 6

pointGroupCount :: Map (Int, Int) Int -> Int
pointGroupCount = sum <|| map (nc3 <|| snd) <|| toList

horizontalGroupCount :: Map (Int, Int) Int -> Int
horizontalGroupCount sketch = sum [
    product [
        fromJust (lookup (i, j) sketch)
        | j <- [0..2]]
    | i <- [0..2]]

verticalGroupCount :: Map (Int, Int) Int -> Int
verticalGroupCount sketch = sum [
    product [
        fromJust (lookup (i, j) sketch)
        | i <- [0..2]]
    | j <- [0..2]]

diagonalGroupCount :: Map (Int, Int) Int -> Int
diagonalGroupCount sketch = sum [
    product [
        fromJust (lookup (m3 (i + j), j) sketch)
        | j <- [0..2]]
    | i <- [0..2]]

antiDiagonalGroupCount :: Map (Int, Int) Int -> Int
antiDiagonalGroupCount sketch = sum [
    product [
        fromJust (lookup (m3 (i - j), j) sketch)
        | j <- [0..2]]
    | i <- [0..2]]

solveTestCase :: Map (Int, Int) Int -> Int
solveTestCase = sum
    <|| (<*>) [
        pointGroupCount,
        horizontalGroupCount,
        verticalGroupCount,
        diagonalGroupCount,
        antiDiagonalGroupCount]
    <|| pure

incrementCoords :: (Int, Int) -> (Int, Int) -> Int -> (Int, Int) -> (Int, Int)
incrementCoords (a, b) (c, d) m (x, y) = uncurry ((,) `on` (`mod` m)) (a * x + b, c * y + d)

parseTestCase :: [Int] -> Map (Int, Int) Int
parseTestCase [n, a, b, c, d, x, y, m] = foldr
    (uncurry <| insertWith (+))
    (fromList [((i, j), 0) | i <- [0..2], j <- [0..2]])
    <| map ((, 1) <|| uncurry ((,) `on` m3))
    <| take n
    <| iterate (incrementCoords (a, b) (c, d) m) (x, y)

handleTestCase :: IO String
handleTestCase = do
    [n, a, b, c, d, x, y, m] <- fmap (map (read :: String -> Int) <|| words) <| getLine
    return <| show <| solveTestCase <| parseTestCase [n, a, b, c, d, x, y, m]

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate handleTestCase
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
