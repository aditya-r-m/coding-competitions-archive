import Data.Function (on)
import Data.Maybe (fromJust)
import Data.List (intersperse, foldl')

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x
(<|||) f g x y = f <| g x y

data Node a = Node {
    capacity :: Int,
    size :: Int,
    value :: Maybe a,
    leftNode :: Maybe (Node a),
    rightNode :: Maybe (Node a)
}

leftSubtreeCapacity :: Node a -> Int
leftSubtreeCapacity node = div (capacity node) 2

rightSubtreeCapacity :: Node a -> Int
rightSubtreeCapacity node = capacity node - leftSubtreeCapacity node

remainingCapacity :: Node a -> Int
remainingCapacity node = capacity node - size node

remainingLeftSubtreeCapacity :: Node a -> Int
remainingLeftSubtreeCapacity node = leftSubtreeCapacity node - safeSize (leftNode node)
    where
    safeSize Nothing = 0
    safeSize (Just (Node { size = s })) = s

create :: Int -> Node a
create c = Node {
    capacity = c,
    size = 0,
    value = Nothing,
    leftNode = Nothing,
    rightNode = Nothing
}

createOrInsert :: Int -> Int -> a -> Maybe (Node a) -> Node a
createOrInsert _ i v (Just node) = insert i v node
createOrInsert c i v Nothing = insert i v <| create c

insert :: Int -> a -> Node a -> Node a
insert i v node
    | capacity node == 1 = Node {
        capacity = 1,
        size = 1,
        value = Just v,
        leftNode = Nothing,
        rightNode = Nothing
    }
    | i <= remainingLeftSubtreeCapacity node = Node {
        capacity = capacity node,
        size = size node + 1,
        value = Nothing,
        leftNode = Just (
            createOrInsert
            (leftSubtreeCapacity node)
            i
            v
            (leftNode node)
        ),
        rightNode = rightNode node
    }
    | i <= remainingCapacity node = Node {
        capacity = capacity node,
        size = size node + 1,
        value = Nothing,
        leftNode = leftNode node,
        rightNode = Just (
            createOrInsert
            (rightSubtreeCapacity node)
            (i - remainingLeftSubtreeCapacity node)
            v
            (rightNode node)
        )
    }

get :: Int -> Maybe (Node a) -> Maybe a
get _ Nothing = Nothing
get i (Just node)
    | capacity node == 1 = value node
    | i <= leftSubtreeCapacity node = get i <| leftNode node
    | i <= capacity node = get (i - leftSubtreeCapacity node) <| rightNode node

insertionSequence :: Int -> [Int]
insertionSequence c = map ((1 +) <|| snd) <| iterate nxt <| (c, 0)
    where nxt (i, p) = (i - 1, mod (p + (1 + c - i)) (i - 1))

solveTestCase :: IO String
solveTestCase = do
    c <- readLn :: IO Int
    qs <- fmap (tail <|| map (read :: String -> Int) <|| words) getLine
    return
        <| concat
        <| intersperse " "
        <| map show
        <| (flip map qs <|| (flip <| fromJust <||| get) <|| Just)
        <| foldl' (flip <| uncurry insert) (create c)
        <| zip (insertionSequence c) [1..c]

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solveTestCase
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
