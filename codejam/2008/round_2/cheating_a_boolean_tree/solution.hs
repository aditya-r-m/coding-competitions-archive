import Prelude hiding (null)
import Data.List (intersperse)
import Data.Sequence (Seq, null, empty, index, fromList, viewr, ViewR(..), insertAt)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

data Operator = Or | And deriving (Enum)

type Cost = Maybe Int

minCost :: Cost -> Cost -> Cost
minCost Nothing b = b
minCost a Nothing = a
minCost (Just x) (Just y) = Just <| min x y

addCosts :: Cost -> Cost -> Cost
addCosts Nothing _ = Nothing
addCosts _ Nothing = Nothing
addCosts (Just x) (Just y) = Just <| x + y

data ResultCosts = ResultCosts Cost Cost

getResultCost :: Bool -> ResultCosts -> Cost
getResultCost False (ResultCosts r _) = r
getResultCost True (ResultCosts _ r) = r

getLeafCosts :: Bool -> ResultCosts
getLeafCosts False = ResultCosts (Just 0) Nothing
getLeafCosts True = ResultCosts Nothing (Just 0)

type Node = (Operator, Bool)

getNodeCosts :: Node -> ResultCosts -> ResultCosts -> ResultCosts
getNodeCosts (Or, switchable) (ResultCosts x0 x1) (ResultCosts y0 y1) = ResultCosts
    (minCost (addCosts x0 y0)
        <| if not switchable then Nothing else addCosts (Just 1) (minCost x0 y0))
    (minCost x1 y1)
getNodeCosts (And, switchable) (ResultCosts x0 x1) (ResultCosts y0 y1) = ResultCosts
    (minCost x0 y0)
    (minCost (addCosts x1 y1)
        <| if not switchable then Nothing else addCosts (Just 1) (minCost x1 y1))

next :: (Seq Node, Seq ResultCosts) -> (Seq Node, Seq ResultCosts)
next (nodeSeq, costSeq) = case viewr nodeSeq of
    (nxtNodeSeq :> node) -> case viewr costSeq of
        (midCostSeq :> costr) -> case viewr midCostSeq of
            (nxtCostSeq :> costl) -> (
                nxtNodeSeq,
                insertAt 0 (getNodeCosts node costl costr) nxtCostSeq)

getRootCosts :: [Node] -> [Bool] -> ResultCosts
getRootCosts nodes leaves = flip index 0
    <| snd <| head
    <| dropWhile (not <|| null <|| fst)
    <| iterate next (fromList nodes, fromList <| map getLeafCosts leaves)

parseNode :: String -> Node
parseNode = (fmap (,) (toEnum <|| head) <*> (toEnum <|| last))
    <|| map read
    <|| words

parseBool :: String -> Bool
parseBool = toEnum <|| read

formatCost :: Cost -> String
formatCost Nothing = "IMPOSSIBLE"
formatCost (Just c) = show c

solve :: IO String
solve = do
    line <- getLine
    let m = read <| head <| words line
    let v = parseBool <| last <| words line
    nodeLines <- sequence <| replicate (div m 2) getLine
    leafLines <- sequence <| replicate (div (m+1) 2) getLine
    let nodes = map parseNode nodeLines
    let leaves = map parseBool leafLines
    return
        <| formatCost
        <| getResultCost v
        <| getRootCosts nodes leaves

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
