import Control.Monad (forM_)
import Data.Maybe (mapMaybe)
import System.Environment (getArgs, getProgName)

main :: IO ()
main = getArgs >>= processArgs

processArgs :: [String] -> IO ()
processArgs [filePath] = readFile filePath >>= processTilemap . parseCSV
processArgs _ = do
  progName <- getProgName
  putStrLn $ "Usage: " ++ progName ++ " [wall_tilemap]"

type Tilemap = [[Int]]

parseCSV :: String -> Tilemap
parseCSV = map (read . \l -> "[" ++ l ++ "]") . lines

processTilemap :: Tilemap -> IO ()
processTilemap tilemap = forM_ colliders print
  where
    height = length tilemap
    width = length . head $ tilemap
    positions = [(x, y) | x <- [0 .. width - 1], y <- [0 .. height - 1]]
    colliders = concat . mapMaybe (getCollider tilemap) $ positions

type Position = (Int, Int)

data Character = None

instance Show Character where
  show None = "None"

data SolidColliderData = SolidColliderData
  { whitelisted :: Character,
    position :: (Float, Float),
    size :: (Float, Float)
  }

instance Show SolidColliderData where
  show (SolidColliderData w p s) =
    "SolidColliderData {whitelisted: "
      ++ show w
      ++ ", corner_position: "
      ++ showVec p
      ++ ", size: "
      ++ showVec s
      ++ "},"
    where
      showVec (x, y) = "Vec2::new(" ++ show x ++ " * TILE_SIZE, " ++ show y ++ " * TILE_SIZE)"

getCollider :: Tilemap -> Position -> Maybe [SolidColliderData]
getCollider tilemap pos@(x, y) =
  case tileID of
    -- unfinished & incorrect colliders
    11 ->
      Just
        [ SolidColliderData None (x' - 0.1, y' - 0.1) (1.0, 0.8),
          SolidColliderData None (x' - 0.1, y' + 0.4) (0.8, 0.2)
        ]
    12 -> Just [SolidColliderData None (x' + 0.1, y' - 0.1) (0.8, 0.8)]
    13 -> Just [SolidColliderData None (x' - 0.1, y' + 0.1) (0.8, 0.8)]
    14 -> Just [SolidColliderData None (x' + 0.1, y' + 0.1) (0.8, 0.8)]
    15 -> Just [SolidColliderData None (x' - 0.1, y' - 0.1) (0.8, 0.8)]
    16 -> Just [SolidColliderData None (x' + 0.1, y' - 0.1) (0.8, 0.8)]
    17 -> Just [SolidColliderData None (x' - 0.1, y') (0.8, 1.0)]
    18 -> Just [SolidColliderData None (x', y' + 0.1) (1.0, 0.8)]
    19 -> Just [SolidColliderData None (x', y' - 0.1) (1.0, 0.8)]
    20 -> Just [SolidColliderData None (x' + 0.1, y') (0.8, 1.0)]
    _ -> Nothing
  where
    x' = fromIntegral x
    y' = fromIntegral y
    floatPos = (x', y')
    Just tileID = index2D tilemap pos

index2D :: [[a]] -> Position -> Maybe a
index2D list (x, y)
  | x >= width || x < 0 || y >= height || y < 0 = Nothing
  | otherwise = Just $ list !! y !! x
  where
    height = length list
    width = length . head $ list