{-# OPTIONS_GHC -Wno-missing-methods #-}
import Prelude hiding ((^))
import Data.List (intersperse)

infixr 0 <|
(<|) f x = f x
(<||) f g x = f <| g x

data I = I Int
instance Num I where
    (+) (I a) (I b) = I (mod (a + b) 1000)
    (*) (I a) (I b) = I (mod (a * b) 1000)
    (-) (I a) (I b) = I (mod (a - b) 1000)
instance Show I where
    show (I i) = reverse <| take 3 <| (++ repeat '0') <| reverse <| show i

type M22I = ((I, I), (I, I))
instance Num M22I where
    (*) ((a00, a01), (a10, a11)) ((b00, b01), (b10, b11)) = (
        (a00 * b00 + a01 * b10, a00 * b01 + a01 * b11),
        (a10 * b00 + a11 * b10, a10 * b01 + a11 * b11))

(^) :: M22I -> Int -> M22I
(^) m 0 = ((I 1, I 0), (I 0, I 1))
(^) m p
    | even p = (m * m)^(div p 2)
    | otherwise = m * (m^(p - 1))

solve :: IO String
solve = (readLn :: IO Int) >>= return
    <|| show
    <|| (flip (-) <| I 1)
    <|| (I 2 *)
    <|| fst <|| fst
    <|| (((I 3, I 5), (I 1, I 3)) ^)

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
