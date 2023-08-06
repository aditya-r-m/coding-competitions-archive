import Data.List (intersperse)
import Data.Set (Set, empty, insert, fromList)

infixr 0 <|
(<|) :: (a -> b) -> a -> b
(<|) f x = f x

(<||) :: (b -> c) -> (a -> b) -> (a -> c)
(<||) f g x = f (g x)

foldQuery :: Int -> (Int, Set String) -> String -> (Int, Set String)
foldQuery engineCount (assignmentCount, pendingAssignments) query
    | engineCount == length currentPendingAssignments = (assignmentCount + 1, fromList [query])
    | otherwise = (assignmentCount, currentPendingAssignments)
    where currentPendingAssignments = insert query pendingAssignments

minimumAssignments :: [String] -> [String] -> Int
minimumAssignments engines = fst <|| foldl (foldQuery <| length engines) (0, empty)

solveQueries :: [String] -> IO Int
solveQueries engines = (readLn :: IO Int)
    >>= sequence <|| flip replicate getLine
    >>= return <|| (minimumAssignments engines)

solveTestCase :: IO Int
solveTestCase = (readLn :: IO Int)
    >>= sequence <|| flip replicate getLine
    >>= solveQueries

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solveTestCase
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
        <|| map show
