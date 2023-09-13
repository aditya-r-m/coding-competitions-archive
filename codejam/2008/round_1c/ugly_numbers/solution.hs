import Data.List (intersperse, sort, groupBy, foldl')
import Data.Function (on)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x
(<|||) f g x y = f <| g x y

bases :: [Integer]
bases = [2,3,5,7]

getKey :: Bool -> Integer -> [Integer]
getKey False = flip map bases <|| mod
getKey True = getKey False <|| negate

addKeys :: [Integer] -> [Integer] -> [Integer]
addKeys = zipWith (flip mod) bases <||| zipWith (+)

type Cache = [(([Integer], (Bool, Integer)), Integer)]

nxt :: Cache -> Integer -> Cache
nxt cache x = map (fmap (,) (fst <|| head) <*> (sum <|| map snd))
    <| groupBy ((==) `on` fst)
    <| sort
    <| concat [[
        ((addKeys kp (getKey cneg cval), (False, x)), s),
        ((addKeys kp (getKey cneg cval), (True, x)), s),
        ((kp, (cneg, 10 * cval + x)), s)
    ] | ((kp, (cneg, cval)), s) <- cache]

uglyNumbers :: [Integer] -> Integer
uglyNumbers (x:xs) = sum
    <| map snd
    <| filter (elem 0 <|| (fmap addKeys fst <*> (uncurry getKey <|| snd)) <|| fst)
    <| foldl' nxt [((getKey False 0, (False, x)), 1)] xs

solve :: IO String
solve = getLine >>= return <|| show <|| uglyNumbers <|| map (read <|| (:[]))

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
