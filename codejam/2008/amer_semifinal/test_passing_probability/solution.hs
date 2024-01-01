import Data.List (intersperse, sort)
import Numeric   (showFFloat)

infixr 0 <|
(<|) f = f
(<||) f g x = f <| g x

solve :: IO String
solve = do
    (m:q:_) <- fmap (map (read :: String -> Int) <|| words) getLine
    qs <- fmap (map (map (read :: String -> Float) <|| words))
        <| sequence
        <| replicate q getLine
    return
        <| flip (showFFloat (Just 9)) ""
        <| sum
        <| foldr (((take m <|| reverse <|| sort) <||) <|| ((<*>) <|| fmap (*))) [1.0] qs

main :: IO ()
main = (readLn :: IO Int)
    >>= sequence <|| flip replicate solve
    >>= putStrLn
        <|| concat
        <|| intersperse "\n"
        <|| map (uncurry (++))
        <|| zip (map (("Case #" ++) <|| (++ ": ") <|| show) [1..])
