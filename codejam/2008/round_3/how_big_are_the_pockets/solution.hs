import Data.Function (on)
import Data.List     (groupBy, intersperse, partition, scanl, sort)
import Data.Tuple    (swap)

infixr 0 <|
(<|) f x = f x
(<||) f x y = f <| x y

type Locn = ((Int, Int), (Int, Int))

move :: Locn -> Locn
move ((i, j), (x, y)) = ((i, j), (x + i, y + j))

turn :: Locn -> Locn
turn ((i, j), p) = ((j, -i), p)

step :: Char -> Locn -> Locn
step 'F' = move
step 'R' = turn
step 'L' = turn <|| turn <|| turn

walk :: [(String, Int)] -> [Locn]
walk = scanl (flip step) ((0, 1), (0, 0))
    <|| concat
    <|| concatMap (uncurry <| flip replicate)

norm :: Locn -> (Int, Int)
norm ((i, j), (x, y)) = (min x (x + i), min y (y + j))

getEdges :: [Locn] -> ([(Int, Int)], [(Int, Int)])
getEdges = (fmap (,)
        (map norm <|| fst) <*>
        (map norm <|| snd))
    <|| partition ((/= 0) <|| fst <|| fst)
    <|| map snd
    <|| filter (uncurry <| on (==) fst)
    <|| (tail >>= zip)

interiorArea :: ([(Int, Int)], [(Int, Int)]) -> Int
interiorArea = sum
    <|| map (sum
        <|| zipWith (*) (concat <| repeat [-1, 1])
        <|| map snd)
    <|| groupBy ((==) `on` fst)
    <|| sort
    <|| fst

fill :: [(Int, Int)] -> [(Int, Int)]
fill verticalEdges = concat [
        (fmap (++)
        (map (, y)) <*>
        (map (, (y+1))))
        [xmn..xmx-1] |
    layer <- groupBy ((==) `on` snd) (map swap <| sort <| map swap verticalEdges),
    let y = snd (head layer),
    let xmn = fst (head layer),
    let xmx = fst (last layer)]

totalArea :: ([(Int, Int)], [(Int, Int)]) -> Int
totalArea (horizontalEdges, verticalEdges) = sum
    <| map (fmap (-)
        (snd <|| last) <*>
        (snd <|| head))
    <| groupBy ((==) `on` fst)
    <| sort
    <| (++) horizontalEdges
    <| fill verticalEdges

parse :: [String] -> [(String, Int)]
parse []        = []
parse (c:rs:xs) = (c, read rs) : parse xs

readn :: Int -> IO [(String, Int)]
readn 0 = return []
readn n = do
    s <- getLine
    let steps = parse <| words s
    rest <- readn <| n - length steps
    return <| steps ++ rest

solve :: IO String
solve = do
    steps <- readLn >>= readn
    let edges = getEdges <| walk steps
    return <| show (fmap (-) totalArea <*> interiorArea <| edges)

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
