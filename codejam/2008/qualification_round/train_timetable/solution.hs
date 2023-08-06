import Data.List (sort, scanl, intersperse)

infixr 0 <|
(<|) :: (a -> b) -> a -> b
(<|) f x = f x

(<||) :: (b -> c) -> (a -> b) -> (a -> c)
(<||) f g x = f <| g x

clockToMinutes :: String -> Int
clockToMinutes c = 60 * read (take 2 c) + read (drop 3 c)

listToPair :: [a] -> (a, a)
listToPair [x, y] = (x, y)

intPairToString :: (Int, Int) -> String
intPairToString (x, y) = show x ++ " " ++ show y

findMaxDepartures :: Int -> [(Int, Int)] -> [(Int, Int)] -> (Int, Int)
findMaxDepartures turnaroundTime scheduleAtoB scheduleBtoA = (
    foldl max 0 <| scanl (+) 0 <| map snd <| sort departureEventsA,
    foldl max 0 <| scanl (+) 0 <| map snd <| sort departureEventsB)
    where
    departureEventsA = map ((, 1) <|| fst) scheduleAtoB ++
        map ((, -1) <|| (turnaroundTime +) <|| snd) scheduleBtoA
    departureEventsB = map ((, 1) <|| fst) scheduleBtoA ++
        map ((, -1) <|| (turnaroundTime +) <|| snd) scheduleAtoB

solveTestCase :: IO String
solveTestCase = do
    turnaroundTime <- (readLn :: IO Int)
    line <- getLine
    let [scheduleLengthAtoB, scheduleLengthBtoA] = map read <| words line
    scheduleAtoBRaw <- sequence <| replicate scheduleLengthAtoB getLine
    let scheduleAtoB = map (listToPair <|| map clockToMinutes <|| words) scheduleAtoBRaw
    scheduleBtoARaw <- sequence <| replicate scheduleLengthBtoA getLine
    let scheduleBtoA = map (listToPair <|| map clockToMinutes <|| words) scheduleBtoARaw
    return <| intPairToString <| findMaxDepartures turnaroundTime scheduleAtoB scheduleBtoA

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solveTestCase
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
