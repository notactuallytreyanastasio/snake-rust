#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
pub enum DirectionEnum {
    Right(Right), Up(Up), Down(Down), Left(Left)
}
pub trait DirectionTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> DirectionEnum;
    fn clone_boxed(& self) -> Direction;
}
#[derive(Clone)]
pub struct Direction(std::sync::Arc<dyn DirectionTrait>);
impl Direction {
    pub fn new(selfish: impl DirectionTrait + 'static) -> Direction {
        Direction(std::sync::Arc::new(selfish))
    }
}
impl DirectionTrait for Direction {
    fn as_enum(& self) -> DirectionEnum {
        DirectionTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> Direction {
        DirectionTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(Direction);
impl std::ops::Deref for Direction {
    type Target = dyn DirectionTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct UpStruct {}
#[derive(Clone)]
pub struct Up(std::sync::Arc<UpStruct>);
impl Up {
    pub fn new() -> Up {
        let selfish = Up(std::sync::Arc::new(UpStruct {}));
        return selfish;
    }
}
impl DirectionTrait for Up {
    fn as_enum(& self) -> DirectionEnum {
        DirectionEnum::Up(self.clone())
    }
    fn clone_boxed(& self) -> Direction {
        Direction::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Up, [Direction]);
struct DownStruct {}
#[derive(Clone)]
pub struct Down(std::sync::Arc<DownStruct>);
impl Down {
    pub fn new() -> Down {
        let selfish = Down(std::sync::Arc::new(DownStruct {}));
        return selfish;
    }
}
impl DirectionTrait for Down {
    fn as_enum(& self) -> DirectionEnum {
        DirectionEnum::Down(self.clone())
    }
    fn clone_boxed(& self) -> Direction {
        Direction::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Down, [Direction]);
struct LeftStruct {}
#[derive(Clone)]
pub struct Left(std::sync::Arc<LeftStruct>);
impl Left {
    pub fn new() -> Left {
        let selfish = Left(std::sync::Arc::new(LeftStruct {}));
        return selfish;
    }
}
impl DirectionTrait for Left {
    fn as_enum(& self) -> DirectionEnum {
        DirectionEnum::Left(self.clone())
    }
    fn clone_boxed(& self) -> Direction {
        Direction::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Left, [Direction]);
struct RightStruct {}
#[derive(Clone)]
pub struct Right(std::sync::Arc<RightStruct>);
impl Right {
    pub fn new() -> Right {
        let selfish = Right(std::sync::Arc::new(RightStruct {}));
        return selfish;
    }
}
impl DirectionTrait for Right {
    fn as_enum(& self) -> DirectionEnum {
        DirectionEnum::Right(self.clone())
    }
    fn clone_boxed(& self) -> Direction {
        Direction::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Right, [Direction]);
struct PointStruct {
    x: i32, y: i32
}
#[derive(Clone)]
pub struct Point(std::sync::Arc<PointStruct>);
#[derive(Clone)]
pub struct PointBuilder {
    pub x: i32, pub y: i32
}
impl PointBuilder {
    pub fn build(self) -> Point {
        Point::new(self.x, self.y)
    }
}
impl Point {
    pub fn new(x__111: i32, y__112: i32) -> Point {
        let x;
        let y;
        x = x__111;
        y = y__112;
        let selfish = Point(std::sync::Arc::new(PointStruct {
                    x, y
        }));
        return selfish;
    }
    pub fn x(& self) -> i32 {
        return self.0.x;
    }
    pub fn y(& self) -> i32 {
        return self.0.y;
    }
}
temper_core::impl_any_value_trait!(Point, []);
pub enum GameStatusEnum {
    Playing(Playing), GameOver(GameOver)
}
pub trait GameStatusTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> GameStatusEnum;
    fn clone_boxed(& self) -> GameStatus;
}
#[derive(Clone)]
pub struct GameStatus(std::sync::Arc<dyn GameStatusTrait>);
impl GameStatus {
    pub fn new(selfish: impl GameStatusTrait + 'static) -> GameStatus {
        GameStatus(std::sync::Arc::new(selfish))
    }
}
impl GameStatusTrait for GameStatus {
    fn as_enum(& self) -> GameStatusEnum {
        GameStatusTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> GameStatus {
        GameStatusTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(GameStatus);
impl std::ops::Deref for GameStatus {
    type Target = dyn GameStatusTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct PlayingStruct {}
#[derive(Clone)]
pub struct Playing(std::sync::Arc<PlayingStruct>);
impl Playing {
    pub fn new() -> Playing {
        let selfish = Playing(std::sync::Arc::new(PlayingStruct {}));
        return selfish;
    }
}
impl GameStatusTrait for Playing {
    fn as_enum(& self) -> GameStatusEnum {
        GameStatusEnum::Playing(self.clone())
    }
    fn clone_boxed(& self) -> GameStatus {
        GameStatus::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Playing, [GameStatus]);
struct GameOverStruct {}
#[derive(Clone)]
pub struct GameOver(std::sync::Arc<GameOverStruct>);
impl GameOver {
    pub fn new() -> GameOver {
        let selfish = GameOver(std::sync::Arc::new(GameOverStruct {}));
        return selfish;
    }
}
impl GameStatusTrait for GameOver {
    fn as_enum(& self) -> GameStatusEnum {
        GameStatusEnum::GameOver(self.clone())
    }
    fn clone_boxed(& self) -> GameStatus {
        GameStatus::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(GameOver, [GameStatus]);
struct RandomResultStruct {
    value: i32, next_seed: i32
}
#[derive(Clone)]
pub struct RandomResult(std::sync::Arc<RandomResultStruct>);
#[derive(Clone)]
pub struct RandomResultBuilder {
    pub value: i32, pub next_seed: i32
}
impl RandomResultBuilder {
    pub fn build(self) -> RandomResult {
        RandomResult::new(self.value, self.next_seed)
    }
}
impl RandomResult {
    pub fn new(value__126: i32, nextSeed__127: i32) -> RandomResult {
        let value;
        let next_seed;
        value = value__126;
        next_seed = nextSeed__127;
        let selfish = RandomResult(std::sync::Arc::new(RandomResultStruct {
                    value, next_seed
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
    pub fn next_seed(& self) -> i32 {
        return self.0.next_seed;
    }
}
temper_core::impl_any_value_trait!(RandomResult, []);
struct SnakeGameStruct {
    width: i32, height: i32, snake: temper_core::List<Point>, direction: Direction, food: Point, score: i32, status: GameStatus, rng_seed: i32
}
#[derive(Clone)]
pub struct SnakeGame(std::sync::Arc<SnakeGameStruct>);
#[derive(Clone)]
pub struct SnakeGameBuilder {
    pub width: i32, pub height: i32, pub snake: temper_core::List<Point>, pub direction: Direction, pub food: Point, pub score: i32, pub status: GameStatus, pub rng_seed: i32
}
impl SnakeGameBuilder {
    pub fn build(self) -> SnakeGame {
        SnakeGame::new(self.width, self.height, self.snake, self.direction, self.food, self.score, self.status, self.rng_seed)
    }
}
impl SnakeGame {
    pub fn new(width__144: i32, height__145: i32, snake__146: impl temper_core::ToList<Point>, direction__147: Direction, food__148: Point, score__149: i32, status__150: GameStatus, rngSeed__151: i32) -> SnakeGame {
        let snake__146 = snake__146.to_list();
        let width;
        let height;
        let snake;
        let direction;
        let food;
        let score;
        let status;
        let rng_seed;
        width = width__144;
        height = height__145;
        snake = snake__146.clone();
        direction = direction__147.clone();
        food = food__148.clone();
        score = score__149;
        status = status__150.clone();
        rng_seed = rngSeed__151;
        let selfish = SnakeGame(std::sync::Arc::new(SnakeGameStruct {
                    width, height, snake, direction, food, score, status, rng_seed
        }));
        return selfish;
    }
    pub fn width(& self) -> i32 {
        return self.0.width;
    }
    pub fn height(& self) -> i32 {
        return self.0.height;
    }
    pub fn snake(& self) -> temper_core::List<Point> {
        return self.0.snake.clone();
    }
    pub fn direction(& self) -> Direction {
        return self.0.direction.clone();
    }
    pub fn food(& self) -> Point {
        return self.0.food.clone();
    }
    pub fn score(& self) -> i32 {
        return self.0.score;
    }
    pub fn status(& self) -> GameStatus {
        return self.0.status.clone();
    }
    pub fn rng_seed(& self) -> i32 {
        return self.0.rng_seed;
    }
}
temper_core::impl_any_value_trait!(SnakeGame, []);
struct FoodPlacementStruct {
    point: Point, seed: i32
}
#[derive(Clone)]
pub (crate) struct FoodPlacement(std::sync::Arc<FoodPlacementStruct>);
impl FoodPlacement {
    pub fn new(point__155: Point, seed__156: i32) -> FoodPlacement {
        let point;
        let seed;
        point = point__155.clone();
        seed = seed__156;
        let selfish = FoodPlacement(std::sync::Arc::new(FoodPlacementStruct {
                    point, seed
        }));
        return selfish;
    }
    pub fn point(& self) -> Point {
        return self.0.point.clone();
    }
    pub fn seed(& self) -> i32 {
        return self.0.seed;
    }
}
temper_core::impl_any_value_trait!(FoodPlacement, []);
pub enum PlayerStatusEnum {
    Alive(Alive), Dead(Dead)
}
pub trait PlayerStatusTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> PlayerStatusEnum;
    fn clone_boxed(& self) -> PlayerStatus;
}
#[derive(Clone)]
pub struct PlayerStatus(std::sync::Arc<dyn PlayerStatusTrait>);
impl PlayerStatus {
    pub fn new(selfish: impl PlayerStatusTrait + 'static) -> PlayerStatus {
        PlayerStatus(std::sync::Arc::new(selfish))
    }
}
impl PlayerStatusTrait for PlayerStatus {
    fn as_enum(& self) -> PlayerStatusEnum {
        PlayerStatusTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> PlayerStatus {
        PlayerStatusTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(PlayerStatus);
impl std::ops::Deref for PlayerStatus {
    type Target = dyn PlayerStatusTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct AliveStruct {}
#[derive(Clone)]
pub struct Alive(std::sync::Arc<AliveStruct>);
impl Alive {
    pub fn new() -> Alive {
        let selfish = Alive(std::sync::Arc::new(AliveStruct {}));
        return selfish;
    }
}
impl PlayerStatusTrait for Alive {
    fn as_enum(& self) -> PlayerStatusEnum {
        PlayerStatusEnum::Alive(self.clone())
    }
    fn clone_boxed(& self) -> PlayerStatus {
        PlayerStatus::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Alive, [PlayerStatus]);
struct DeadStruct {}
#[derive(Clone)]
pub struct Dead(std::sync::Arc<DeadStruct>);
impl Dead {
    pub fn new() -> Dead {
        let selfish = Dead(std::sync::Arc::new(DeadStruct {}));
        return selfish;
    }
}
impl PlayerStatusTrait for Dead {
    fn as_enum(& self) -> PlayerStatusEnum {
        PlayerStatusEnum::Dead(self.clone())
    }
    fn clone_boxed(& self) -> PlayerStatus {
        PlayerStatus::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Dead, [PlayerStatus]);
struct PlayerSnakeStruct {
    id: i32, segments: temper_core::List<Point>, direction: Direction, score: i32, status: PlayerStatus
}
#[derive(Clone)]
pub struct PlayerSnake(std::sync::Arc<PlayerSnakeStruct>);
#[derive(Clone)]
pub struct PlayerSnakeBuilder {
    pub id: i32, pub segments: temper_core::List<Point>, pub direction: Direction, pub score: i32, pub status: PlayerStatus
}
impl PlayerSnakeBuilder {
    pub fn build(self) -> PlayerSnake {
        PlayerSnake::new(self.id, self.segments, self.direction, self.score, self.status)
    }
}
impl PlayerSnake {
    pub fn new(id__223: i32, segments__224: impl temper_core::ToList<Point>, direction__225: Direction, score__226: i32, status__227: PlayerStatus) -> PlayerSnake {
        let segments__224 = segments__224.to_list();
        let id;
        let segments;
        let direction;
        let score;
        let status;
        id = id__223;
        segments = segments__224.clone();
        direction = direction__225.clone();
        score = score__226;
        status = status__227.clone();
        let selfish = PlayerSnake(std::sync::Arc::new(PlayerSnakeStruct {
                    id, segments, direction, score, status
        }));
        return selfish;
    }
    pub fn id(& self) -> i32 {
        return self.0.id;
    }
    pub fn segments(& self) -> temper_core::List<Point> {
        return self.0.segments.clone();
    }
    pub fn direction(& self) -> Direction {
        return self.0.direction.clone();
    }
    pub fn score(& self) -> i32 {
        return self.0.score;
    }
    pub fn status(& self) -> PlayerStatus {
        return self.0.status.clone();
    }
}
temper_core::impl_any_value_trait!(PlayerSnake, []);
struct MultiSnakeGameStruct {
    width: i32, height: i32, snakes: temper_core::List<PlayerSnake>, food: Point, rng_seed: i32, tick_count: i32
}
#[derive(Clone)]
pub struct MultiSnakeGame(std::sync::Arc<MultiSnakeGameStruct>);
#[derive(Clone)]
pub struct MultiSnakeGameBuilder {
    pub width: i32, pub height: i32, pub snakes: temper_core::List<PlayerSnake>, pub food: Point, pub rng_seed: i32, pub tick_count: i32
}
impl MultiSnakeGameBuilder {
    pub fn build(self) -> MultiSnakeGame {
        MultiSnakeGame::new(self.width, self.height, self.snakes, self.food, self.rng_seed, self.tick_count)
    }
}
impl MultiSnakeGame {
    pub fn new(width__235: i32, height__236: i32, snakes__237: impl temper_core::ToList<PlayerSnake>, food__238: Point, rngSeed__239: i32, tickCount__240: i32) -> MultiSnakeGame {
        let snakes__237 = snakes__237.to_list();
        let width;
        let height;
        let snakes;
        let food;
        let rng_seed;
        let tick_count;
        width = width__235;
        height = height__236;
        snakes = snakes__237.clone();
        food = food__238.clone();
        rng_seed = rngSeed__239;
        tick_count = tickCount__240;
        let selfish = MultiSnakeGame(std::sync::Arc::new(MultiSnakeGameStruct {
                    width, height, snakes, food, rng_seed, tick_count
        }));
        return selfish;
    }
    pub fn width(& self) -> i32 {
        return self.0.width;
    }
    pub fn height(& self) -> i32 {
        return self.0.height;
    }
    pub fn snakes(& self) -> temper_core::List<PlayerSnake> {
        return self.0.snakes.clone();
    }
    pub fn food(& self) -> Point {
        return self.0.food.clone();
    }
    pub fn rng_seed(& self) -> i32 {
        return self.0.rng_seed;
    }
    pub fn tick_count(& self) -> i32 {
        return self.0.tick_count;
    }
}
temper_core::impl_any_value_trait!(MultiSnakeGame, []);
struct SpawnInfoStruct {
    point: Point, direction: Direction
}
#[derive(Clone)]
pub (crate) struct SpawnInfo(std::sync::Arc<SpawnInfoStruct>);
impl SpawnInfo {
    pub fn new(point__260: Point, direction__261: Direction) -> SpawnInfo {
        let point;
        let direction;
        point = point__260.clone();
        direction = direction__261.clone();
        let selfish = SpawnInfo(std::sync::Arc::new(SpawnInfoStruct {
                    point, direction
        }));
        return selfish;
    }
    pub fn point(& self) -> Point {
        return self.0.point.clone();
    }
    pub fn direction(& self) -> Direction {
        return self.0.direction.clone();
    }
}
temper_core::impl_any_value_trait!(SpawnInfo, []);
pub fn r#move(head__98: Point, body__99: impl temper_core::ToList<Point>, food__100: Point, width__101: i32, height__102: i32) -> Direction {
    let body__99 = body__99.to_list();
    return Direction::new(Right::new());
}
pub fn point_equals(a__115: Point, b__116: Point) -> bool {
    let return__50: bool;
    let mut t___2803: i32;
    let mut t___2804: i32;
    if Some(a__115.x()) == Some(b__116.x()) {
        t___2803 = a__115.y();
        t___2804 = b__116.y();
        return__50 = Some(t___2803) == Some(t___2804);
    } else {
        return__50 = false;
    }
    return return__50;
}
pub fn is_opposite(a__118: Direction, b__119: Direction) -> bool {
    let return__51: bool;
    if temper_core::is::<Up>(a__118.clone()) {
        return__51 = temper_core::is::<Down>(b__119.clone());
    } else {
        if temper_core::is::<Down>(a__118.clone()) {
            return__51 = temper_core::is::<Up>(b__119.clone());
        } else {
            if temper_core::is::<Left>(a__118.clone()) {
                return__51 = temper_core::is::<Right>(b__119.clone());
            } else {
                if temper_core::is::<Right>(a__118.clone()) {
                    return__51 = temper_core::is::<Left>(b__119.clone());
                } else {
                    return__51 = false;
                }
            }
        }
    }
    return return__51;
}
pub fn direction_delta(dir__121: Direction) -> Point {
    let return__52: Point;
    if temper_core::is::<Up>(dir__121.clone()) {
        return__52 = Point::new(0, -1);
    } else {
        if temper_core::is::<Down>(dir__121.clone()) {
            return__52 = Point::new(0, 1);
        } else {
            if temper_core::is::<Left>(dir__121.clone()) {
                return__52 = Point::new(-1, 0);
            } else {
                if temper_core::is::<Right>(dir__121.clone()) {
                    return__52 = Point::new(1, 0);
                } else {
                    return__52 = Point::new(0, 0);
                }
            }
        }
    }
    return return__52.clone();
}
pub fn next_random(seed__128: i32, max__129: i32) -> RandomResult {
    let mut t___1687: i32;
    let mut t___1689: i32;
    let raw__131: i32 = seed__128.wrapping_mul(1664525).wrapping_add(1013904223);
    let masked__132: i32 = raw__131 & 2147483647;
    let posVal__133: i32;
    if Some(masked__132) < Some(0) {
        posVal__133 = masked__132.wrapping_neg();
    } else {
        posVal__133 = masked__132;
    }
    let mut value__134: i32 = 0;
    if Some(max__129) > Some(0) {
        'ok___2806: {
            'orelse___519: {
                t___1687 = match temper_core::int_rem(posVal__133, max__129) {
                    Ok(x) => x,
                    _ => break 'orelse___519
                };
                t___1689 = t___1687;
                break 'ok___2806;
            }
            t___1689 = 0;
        }
        value__134 = t___1689;
    }
    return RandomResult::new(value__134, masked__132);
}
fn placeFood__93(snake__157: impl temper_core::ToList<Point>, width__158: i32, height__159: i32, seed__160: i32) -> FoodPlacement {
    let snake__157 = snake__157.to_list();
    let return__61: FoodPlacement;
    let mut t___2770: i32;
    let mut t___2781: Point;
    let mut t___1652: i32;
    let mut t___1654: i32;
    let mut t___1656: i32;
    let mut t___1658: i32;
    'fn__161: {
        let totalCells__162: i32 = width__158.wrapping_mul(height__159);
        let mut currentSeed__163: i32 = seed__160;
        let mut attempt__164: i32 = 0;
        'loop___2819: while Some(attempt__164) < Some(totalCells__162) {
            let result__165: RandomResult = next_random(currentSeed__163, totalCells__162);
            t___2770 = result__165.next_seed();
            currentSeed__163 = t___2770;
            let mut px__166: i32 = 0;
            let mut py__167: i32 = 0;
            if Some(width__158) > Some(0) {
                'ok___2807: {
                    'orelse___520: {
                        t___1652 = match temper_core::int_rem(result__165.value(), width__158) {
                            Ok(x) => x,
                            _ => break 'orelse___520
                        };
                        t___1654 = t___1652;
                        break 'ok___2807;
                    }
                    t___1654 = 0;
                }
                px__166 = t___1654;
                'ok___2808: {
                    'orelse___521: {
                        t___1656 = match temper_core::int_div(result__165.value(), width__158) {
                            Ok(x) => x,
                            _ => break 'orelse___521
                        };
                        t___1658 = t___1656;
                        break 'ok___2808;
                    }
                    t___1658 = 0;
                }
                py__167 = t___1658;
            }
            let candidate__168: Point = Point::new(px__166, py__167);
            let mut occupied__169: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___1 {
                candidate__168: Point, occupied__169: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___1 {
                fn fn__2769(& self, seg__170: Point) {
                    if point_equals(seg__170.clone(), self.candidate__168.clone()) {
                        {
                            * self.occupied__169.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___1 {
                candidate__168: candidate__168.clone(), occupied__169: occupied__169.clone()
            };
            let fn__2769 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | seg__170: Point | closure_group.fn__2769(seg__170))
            };
            temper_core::listed::list_for_each( & snake__157, & ( * fn__2769.clone()));
            if ! temper_core::read_locked( & occupied__169) {
                return__61 = FoodPlacement::new(candidate__168.clone(), currentSeed__163);
                break 'fn__161;
            }
            attempt__164 = attempt__164.wrapping_add(1);
        }
        let mut y__171: i32 = 0;
        'loop___2820: while Some(y__171) < Some(height__159) {
            let mut x__172: i32 = 0;
            'loop___2821: while Some(x__172) < Some(width__158) {
                let candidate__173: Point = Point::new(x__172, y__171);
                let mut free__174: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
                #[derive(Clone)]
                struct ClosureGroup___2 {
                    candidate__173: Point, free__174: std::sync::Arc<std::sync::RwLock<bool>>
                }
                impl ClosureGroup___2 {
                    fn fn__2768(& self, seg__175: Point) {
                        if point_equals(seg__175.clone(), self.candidate__173.clone()) {
                            {
                                * self.free__174.write().unwrap() = false;
                            }
                        }
                    }
                }
                let closure_group = ClosureGroup___2 {
                    candidate__173: candidate__173.clone(), free__174: free__174.clone()
                };
                let fn__2768 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | seg__175: Point | closure_group.fn__2768(seg__175))
                };
                temper_core::listed::list_for_each( & snake__157, & ( * fn__2768.clone()));
                if temper_core::read_locked( & free__174) {
                    return__61 = FoodPlacement::new(candidate__173.clone(), currentSeed__163);
                    break 'fn__161;
                }
                x__172 = x__172.wrapping_add(1);
            }
            y__171 = y__171.wrapping_add(1);
        }
        t___2781 = Point::new(0, 0);
        return__61 = FoodPlacement::new(t___2781.clone(), currentSeed__163);
    }
    return return__61.clone();
}
pub fn new_game(width__176: i32, height__177: i32, seed__178: i32) -> SnakeGame {
    let mut t___1635: i32;
    let mut t___1637: i32;
    let mut t___1638: i32;
    let mut t___1640: i32;
    let mut centerX__180: i32 = 0;
    let mut centerY__181: i32 = 0;
    if Some(width__176) > Some(0) {
        t___1635 = width__176.wrapping_div(2);
        t___1637 = t___1635;
        centerX__180 = t___1637;
    }
    if Some(height__177) > Some(0) {
        t___1638 = height__177.wrapping_div(2);
        t___1640 = t___1638;
        centerY__181 = t___1640;
    }
    let snake__182: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(centerX__180, centerY__181), Point::new(centerX__180.wrapping_sub(1), centerY__181), Point::new(centerX__180.wrapping_sub(2), centerY__181)]);
    let foodResult__183: FoodPlacement = placeFood__93(snake__182.clone(), width__176, height__177, seed__178);
    let mut t___2763: Right = Right::new();
    let mut t___2764: Point = foodResult__183.point();
    let mut t___2765: Playing = Playing::new();
    let mut t___2766: i32 = foodResult__183.seed();
    return SnakeGame::new(width__176, height__177, snake__182.clone(), Direction::new(t___2763.clone()), t___2764.clone(), 0, GameStatus::new(t___2765.clone()), t___2766);
}
pub fn change_direction(game__184: SnakeGame, dir__185: Direction) -> SnakeGame {
    let return__63: SnakeGame;
    let mut t___2751: i32;
    let mut t___2752: i32;
    let mut t___2753: temper_core::List<Point>;
    let mut t___2754: Point;
    let mut t___2755: i32;
    let mut t___2756: GameStatus;
    let mut t___2757: i32;
    if is_opposite(game__184.direction(), dir__185.clone()) {
        return__63 = game__184.clone();
    } else {
        t___2751 = game__184.width();
        t___2752 = game__184.height();
        t___2753 = game__184.snake();
        t___2754 = game__184.food();
        t___2755 = game__184.score();
        t___2756 = game__184.status();
        t___2757 = game__184.rng_seed();
        return__63 = SnakeGame::new(t___2751, t___2752, t___2753.clone(), dir__185.clone(), t___2754.clone(), t___2755, t___2756.clone(), t___2757);
    }
    return return__63.clone();
}
pub fn tick(game__187: SnakeGame) -> SnakeGame {
    let return__64: SnakeGame;
    let mut t___2691: i32;
    let mut t___2692: i32;
    let mut t___2693: i32;
    let mut t___2694: i32;
    let mut t___2695: temper_core::List<Point>;
    let mut t___2696: Direction;
    let mut t___2697: Point;
    let mut t___2698: i32;
    let mut t___2699: GameOver;
    let mut t___2700: i32;
    let mut t___2704: i32;
    let mut t___2706: i32;
    let mut t___2707: temper_core::List<Point>;
    let mut t___2708: Point;
    let mut t___2710: i32;
    let mut t___2711: i32;
    let mut t___2712: temper_core::List<Point>;
    let mut t___2713: Direction;
    let mut t___2714: Point;
    let mut t___2715: i32;
    let mut t___2716: GameOver;
    let mut t___2717: i32;
    let mut t___2722: i32;
    let mut t___2724: i32;
    let mut t___2725: temper_core::List<Point>;
    let mut t___2726: Point;
    let mut t___2731: i32;
    let mut t___2732: i32;
    let mut t___2733: i32;
    let mut t___2735: i32;
    let mut t___2736: i32;
    let mut t___2737: Direction;
    let mut t___2738: Point;
    let mut t___2739: Playing;
    let mut t___2740: i32;
    let mut t___2742: i32;
    let mut t___2743: i32;
    let mut t___2744: Direction;
    let mut t___2745: Point;
    let mut t___2746: i32;
    let mut t___2747: GameStatus;
    let mut t___2748: i32;
    let mut t___1562: bool;
    let mut t___1563: bool;
    let mut t___1564: bool;
    'fn__188: {
        if temper_core::is::<GameOver>(game__187.status()) {
            return__64 = game__187.clone();
            break 'fn__188;
        }
        let delta__189: Point = direction_delta(game__187.direction());
        let head__190: Point = temper_core::ListedTrait::get_or( & game__187.snake(), 0, Point::new(0, 0));
        let newHead__191: Point = Point::new(head__190.x().wrapping_add(delta__189.x()), head__190.y().wrapping_add(delta__189.y()));
        if Some(newHead__191.x()) < Some(0) {
            t___1564 = true;
        } else {
            if Some(newHead__191.x()) >= Some(game__187.width()) {
                t___1563 = true;
            } else {
                if Some(newHead__191.y()) < Some(0) {
                    t___1562 = true;
                } else {
                    t___2691 = newHead__191.y();
                    t___2692 = game__187.height();
                    t___1562 = Some(t___2691) >= Some(t___2692);
                }
                t___1563 = t___1562;
            }
            t___1564 = t___1563;
        }
        if t___1564 {
            t___2693 = game__187.width();
            t___2694 = game__187.height();
            t___2695 = game__187.snake();
            t___2696 = game__187.direction();
            t___2697 = game__187.food();
            t___2698 = game__187.score();
            t___2699 = GameOver::new();
            t___2700 = game__187.rng_seed();
            return__64 = SnakeGame::new(t___2693, t___2694, t___2695.clone(), t___2696.clone(), t___2697.clone(), t___2698, GameStatus::new(t___2699.clone()), t___2700);
            break 'fn__188;
        }
        let eating__192: bool = point_equals(newHead__191.clone(), game__187.food());
        let checkLength__193: i32;
        if eating__192 {
            t___2704 = temper_core::ListedTrait::len( & game__187.snake());
            checkLength__193 = t___2704;
        } else {
            t___2706 = temper_core::ListedTrait::len( & game__187.snake());
            checkLength__193 = t___2706.wrapping_sub(1);
        }
        let mut i__194: i32 = 0;
        'loop___2822: while Some(i__194) < Some(checkLength__193) {
            t___2707 = game__187.snake();
            t___2708 = Point::new(-1, -1);
            if point_equals(newHead__191.clone(), temper_core::ListedTrait::get_or( & t___2707, i__194, t___2708.clone())) {
                t___2710 = game__187.width();
                t___2711 = game__187.height();
                t___2712 = game__187.snake();
                t___2713 = game__187.direction();
                t___2714 = game__187.food();
                t___2715 = game__187.score();
                t___2716 = GameOver::new();
                t___2717 = game__187.rng_seed();
                return__64 = SnakeGame::new(t___2710, t___2711, t___2712.clone(), t___2713.clone(), t___2714.clone(), t___2715, GameStatus::new(t___2716.clone()), t___2717);
                break 'fn__188;
            }
            i__194 = i__194.wrapping_add(1);
        }
        let newSnakeBuilder__195: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
        temper_core::listed::add( & newSnakeBuilder__195, newHead__191.clone(), None);
        let keepLength__196: i32;
        if eating__192 {
            t___2722 = temper_core::ListedTrait::len( & game__187.snake());
            keepLength__196 = t___2722;
        } else {
            t___2724 = temper_core::ListedTrait::len( & game__187.snake());
            keepLength__196 = t___2724.wrapping_sub(1);
        }
        let mut i__197: i32 = 0;
        'loop___2823: while Some(i__197) < Some(keepLength__196) {
            t___2725 = game__187.snake();
            t___2726 = Point::new(0, 0);
            temper_core::listed::add( & newSnakeBuilder__195, temper_core::ListedTrait::get_or( & t___2725, i__197, t___2726.clone()), None);
            i__197 = i__197.wrapping_add(1);
        }
        let newSnake__198: temper_core::List<Point> = temper_core::ListedTrait::to_list( & newSnakeBuilder__195);
        if eating__192 {
            let newScore__199: i32 = game__187.score().wrapping_add(1);
            t___2731 = game__187.width();
            t___2732 = game__187.height();
            t___2733 = game__187.rng_seed();
            let foodResult__200: FoodPlacement = placeFood__93(newSnake__198.clone(), t___2731, t___2732, t___2733);
            t___2735 = game__187.width();
            t___2736 = game__187.height();
            t___2737 = game__187.direction();
            t___2738 = foodResult__200.point();
            t___2739 = Playing::new();
            t___2740 = foodResult__200.seed();
            return__64 = SnakeGame::new(t___2735, t___2736, newSnake__198.clone(), t___2737.clone(), t___2738.clone(), newScore__199, GameStatus::new(t___2739.clone()), t___2740);
        } else {
            t___2742 = game__187.width();
            t___2743 = game__187.height();
            t___2744 = game__187.direction();
            t___2745 = game__187.food();
            t___2746 = game__187.score();
            t___2747 = game__187.status();
            t___2748 = game__187.rng_seed();
            return__64 = SnakeGame::new(t___2742, t___2743, newSnake__198.clone(), t___2744.clone(), t___2745.clone(), t___2746, t___2747.clone(), t___2748);
        }
    }
    return return__64.clone();
}
fn cellChar__94(game__210: SnakeGame, p__211: Point) -> std::sync::Arc<String> {
    let return__66: std::sync::Arc<String>;
    let mut t___2670: i32;
    let mut t___2671: temper_core::List<Point>;
    let mut t___2672: Point;
    let mut t___2673: Point;
    let mut t___2674: Point;
    'fn__212: {
        let head__213: Point = temper_core::ListedTrait::get_or( & game__210.snake(), 0, Point::new(-1, -1));
        if point_equals(p__211.clone(), head__213.clone()) {
            return__66 = std::sync::Arc::new("@".to_string());
            break 'fn__212;
        }
        let mut i__214: i32 = 1;
        'loop___2824: loop {
            t___2670 = temper_core::ListedTrait::len( & game__210.snake());
            if ! (Some(i__214) < Some(t___2670)) {
                break;
            }
            t___2671 = game__210.snake();
            t___2672 = Point::new(-1, -1);
            t___2673 = temper_core::ListedTrait::get_or( & t___2671, i__214, t___2672.clone());
            if point_equals(p__211.clone(), t___2673.clone()) {
                return__66 = std::sync::Arc::new("o".to_string());
                break 'fn__212;
            }
            i__214 = i__214.wrapping_add(1);
        }
        t___2674 = game__210.food();
        if point_equals(p__211.clone(), t___2674.clone()) {
            return__66 = std::sync::Arc::new("*".to_string());
            break 'fn__212;
        }
        return__66 = std::sync::Arc::new(" ".to_string());
    }
    return return__66.clone();
}
pub fn render(game__201: SnakeGame) -> std::sync::Arc<String> {
    let mut t___2645: i32;
    let mut t___2648: i32;
    let mut t___2650: i32;
    let mut t___2656: i32;
    let sb__203: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    temper_core::string::builder::append( & sb__203, "\x1b[2J\x1b[H");
    temper_core::string::builder::append( & sb__203, "#");
    let mut x__204: i32 = 0;
    'loop___2825: loop {
        t___2645 = game__201.width();
        if ! (Some(x__204) < Some(t___2645)) {
            break;
        }
        temper_core::string::builder::append( & sb__203, "#");
        x__204 = x__204.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__203, "#\x0d\x0a");
    let mut y__205: i32 = 0;
    'loop___2826: loop {
        t___2648 = game__201.height();
        if ! (Some(y__205) < Some(t___2648)) {
            break;
        }
        temper_core::string::builder::append( & sb__203, "#");
        let mut x__206: i32 = 0;
        'loop___2827: loop {
            t___2650 = game__201.width();
            if ! (Some(x__206) < Some(t___2650)) {
                break;
            }
            let p__207: Point = Point::new(x__206, y__205);
            temper_core::string::builder::append( & sb__203, cellChar__94(game__201.clone(), p__207.clone()));
            x__206 = x__206.wrapping_add(1);
        }
        temper_core::string::builder::append( & sb__203, "#\x0d\x0a");
        y__205 = y__205.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__203, "#");
    let mut x__208: i32 = 0;
    'loop___2828: loop {
        t___2656 = game__201.width();
        if ! (Some(x__208) < Some(t___2656)) {
            break;
        }
        temper_core::string::builder::append( & sb__203, "#");
        x__208 = x__208.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__203, "#\x0d\x0a");
    let statusText__209: std::sync::Arc<String>;
    let mut t___2659: GameStatus = game__201.status();
    if temper_core::is::<Playing>(t___2659.clone()) {
        statusText__209 = std::sync::Arc::new("Playing".to_string());
    } else {
        if temper_core::is::<GameOver>(t___2659.clone()) {
            statusText__209 = std::sync::Arc::new("GAME OVER".to_string());
        } else {
            statusText__209 = std::sync::Arc::new("".to_string());
        }
    }
    temper_core::string::builder::append( & sb__203, std::sync::Arc::new(format!("Score: {}  {}\x0d\x0a", game__201.score(), statusText__209.clone())));
    return temper_core::string::builder::to_string( & sb__203);
}
fn spawnPosition__95(width__262: i32, height__263: i32, index__264: i32, seed__265: i32) -> SpawnInfo {
    let return__80: SpawnInfo;
    let mut t___2624: Point;
    let mut t___2625: Right;
    let mut t___2631: Direction;
    let mut t___2633: Direction;
    let mut t___2635: Direction;
    let mut t___2637: Direction;
    let mut t___2639: Direction;
    let mut t___2640: Point;
    let mut t___1463: bool;
    let mut t___1464: i32;
    let mut t___1466: i32;
    let mut t___1467: i32;
    let mut t___1469: i32;
    'fn__266: {
        let buf__267: i32 = 5;
        let safeW__268: i32 = width__262.wrapping_sub(10);
        let safeH__269: i32 = height__263.wrapping_sub(10);
        if Some(safeW__268) < Some(1) {
            t___1463 = true;
        } else {
            t___1463 = Some(safeH__269) < Some(1);
        }
        if t___1463 {
            t___1464 = width__262.wrapping_div(2);
            t___1466 = t___1464;
            t___1467 = height__263.wrapping_div(2);
            t___1469 = t___1467;
            t___2624 = Point::new(t___1466, t___1469);
            t___2625 = Right::new();
            return__80 = SpawnInfo::new(t___2624.clone(), Direction::new(t___2625.clone()));
            break 'fn__266;
        }
        let r1__270: RandomResult = next_random(seed__265.wrapping_mul(7).wrapping_add(index__264.wrapping_mul(131)).wrapping_add(37), safeW__268);
        let r2__271: RandomResult = next_random(r1__270.next_seed(), safeH__269);
        let x__272: i32 = (5 as i32).wrapping_add(r1__270.value());
        let y__273: i32 = (5 as i32).wrapping_add(r2__271.value());
        let r3__274: RandomResult = next_random(r2__271.next_seed(), 4);
        t___2631 = Direction::new(Right::new());
        let mut dir__275: Direction = t___2631.clone();
        if Some(r3__274.value()) == Some(0) {
            t___2633 = Direction::new(Right::new());
            dir__275 = t___2633.clone();
        }
        if Some(r3__274.value()) == Some(1) {
            t___2635 = Direction::new(Left::new());
            dir__275 = t___2635.clone();
        }
        if Some(r3__274.value()) == Some(2) {
            t___2637 = Direction::new(Down::new());
            dir__275 = t___2637.clone();
        }
        if Some(r3__274.value()) == Some(3) {
            t___2639 = Direction::new(Up::new());
            dir__275 = t___2639.clone();
        }
        t___2640 = Point::new(x__272, y__273);
        return__80 = SpawnInfo::new(t___2640.clone(), dir__275.clone());
    }
    return return__80.clone();
}
fn collectAllSegments__96(snakes__276: impl temper_core::ToList<PlayerSnake>) -> temper_core::List<Point> {
    let snakes__276 = snakes__276.to_list();
    let mut t___2611: i32;
    let mut t___2615: PlayerSnake;
    let mut t___2618: i32;
    let mut t___2619: temper_core::List<Point>;
    let mut t___2620: Point;
    let builder__278: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
    let mut i__279: i32 = 0;
    'loop___2829: loop {
        t___2611 = temper_core::ListedTrait::len( & snakes__276);
        if ! (Some(i__279) < Some(t___2611)) {
            break;
        }
        t___2615 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__280: PlayerSnake = temper_core::ListedTrait::get_or( & snakes__276, i__279, t___2615.clone());
        let mut j__281: i32 = 0;
        'loop___2830: loop {
            t___2618 = temper_core::ListedTrait::len( & snake__280.segments());
            if ! (Some(j__281) < Some(t___2618)) {
                break;
            }
            t___2619 = snake__280.segments();
            t___2620 = Point::new(0, 0);
            temper_core::listed::add( & builder__278, temper_core::ListedTrait::get_or( & t___2619, j__281, t___2620.clone()), None);
            j__281 = j__281.wrapping_add(1);
        }
        i__279 = i__279.wrapping_add(1);
    }
    return temper_core::ListedTrait::to_list( & builder__278);
}
pub fn new_multi_game(width__241: i32, height__242: i32, numPlayers__243: i32, seed__244: i32) -> MultiSnakeGame {
    let mut t___2600: Alive;
    let snakeBuilder__246: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut currentSeed__247: i32 = seed__244;
    let mut i__248: i32 = 0;
    'loop___2831: while Some(i__248) < Some(numPlayers__243) {
        let spawn__249: SpawnInfo = spawnPosition__95(width__241, height__242, i__248, currentSeed__247);
        let dir__250: Direction = spawn__249.direction();
        let startX__251: i32 = spawn__249.point().x();
        let startY__252: i32 = spawn__249.point().y();
        let delta__253: Point = direction_delta(dir__250.clone());
        let segments__254: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(startX__251, startY__252), Point::new(startX__251.wrapping_sub(delta__253.x()), startY__252.wrapping_sub(delta__253.y())), Point::new(startX__251.wrapping_sub(delta__253.x().wrapping_mul(2)), startY__252.wrapping_sub(delta__253.y().wrapping_mul(2)))]);
        t___2600 = Alive::new();
        temper_core::listed::add( & snakeBuilder__246, PlayerSnake::new(i__248, segments__254.clone(), dir__250.clone(), 0, PlayerStatus::new(t___2600.clone())), None);
        i__248 = i__248.wrapping_add(1);
    }
    let mut t___2603: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & snakeBuilder__246);
    let allSegments__255: temper_core::List<Point> = collectAllSegments__96(t___2603.clone());
    let foodResult__256: FoodPlacement = placeFood__93(allSegments__255.clone(), width__241, height__242, currentSeed__247);
    let mut t___2606: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & snakeBuilder__246);
    let mut t___2607: Point = foodResult__256.point();
    let mut t___2608: i32 = foodResult__256.seed();
    return MultiSnakeGame::new(width__241, height__242, t___2606.clone(), t___2607.clone(), t___2608, 0);
}
pub fn multi_tick(game__282: MultiSnakeGame, directions__283: impl temper_core::ToList<Direction>) -> MultiSnakeGame {
    let directions__283 = directions__283.to_list();
    let mut t___2429: i32;
    let mut t___2430: temper_core::List<PlayerSnake>;
    let mut t___2434: PlayerSnake;
    let mut t___2436: Direction;
    let mut t___2444: i32;
    let mut t___2445: temper_core::List<PlayerSnake>;
    let mut t___2449: PlayerSnake;
    let mut t___2453: temper_core::List<Direction>;
    let mut t___2454: Right;
    let mut t___2472: i32;
    let mut t___2473: temper_core::List<PlayerSnake>;
    let mut t___2477: PlayerSnake;
    let mut t___2482: Point;
    let mut t___2488: i32;
    let mut t___2489: i32;
    let mut t___2491: i32;
    let mut t___2492: temper_core::List<Point>;
    let mut t___2493: Point;
    let mut t___2496: i32;
    let mut t___2497: temper_core::List<PlayerSnake>;
    let mut t___2501: PlayerSnake;
    let mut t___2506: i32;
    let mut t___2507: temper_core::List<Point>;
    let mut t___2508: Point;
    let mut t___2511: i32;
    let mut t___2512: temper_core::List<PlayerSnake>;
    let mut t___2516: PlayerSnake;
    let mut t___2520: Point;
    let mut t___2525: i32;
    let mut t___2527: Point;
    let mut t___2532: i32;
    let mut t___2533: temper_core::List<PlayerSnake>;
    let mut t___2537: PlayerSnake;
    let mut t___2550: Point;
    let mut t___2552: Direction;
    let mut t___2555: i32;
    let mut t___2557: i32;
    let mut t___2560: temper_core::List<Point>;
    let mut t___2561: Point;
    let mut t___2564: i32;
    let mut t___2565: i32;
    let mut t___2575: i32;
    let mut t___2576: i32;
    let mut t___2577: i32;
    let mut t___2579: Point;
    let mut t___2580: i32;
    let mut t___1325: bool;
    let mut t___1326: bool;
    let mut t___1327: bool;
    let mut t___1397: i32;
    let mut t___1405: i32;
    let newDirs__285: temper_core::ListBuilder<Direction> = temper_core::listed::new_builder();
    let mut i__286: i32 = 0;
    'loop___2832: loop {
        t___2429 = temper_core::ListedTrait::len( & game__282.snakes());
        if ! (Some(i__286) < Some(t___2429)) {
            break;
        }
        t___2430 = game__282.snakes();
        t___2434 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__287: PlayerSnake = temper_core::ListedTrait::get_or( & t___2430, i__286, t___2434.clone());
        t___2436 = snake__287.direction();
        let inputDir__288: Direction = temper_core::ListedTrait::get_or( & directions__283, i__286, t___2436.clone());
        if is_opposite(snake__287.direction(), inputDir__288.clone()) {
            temper_core::listed::add( & newDirs__285, snake__287.direction(), None);
        } else {
            temper_core::listed::add( & newDirs__285, inputDir__288.clone(), None);
        }
        i__286 = i__286.wrapping_add(1);
    }
    let newHeads__289: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
    let mut i__290: i32 = 0;
    'loop___2833: loop {
        t___2444 = temper_core::ListedTrait::len( & game__282.snakes());
        if ! (Some(i__290) < Some(t___2444)) {
            break;
        }
        t___2445 = game__282.snakes();
        t___2449 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__291: PlayerSnake = temper_core::ListedTrait::get_or( & t___2445, i__290, t___2449.clone());
        if temper_core::is::<Alive>(snake__291.status()) {
            t___2453 = temper_core::ListedTrait::to_list( & newDirs__285);
            t___2454 = Right::new();
            let dir__292: Direction = temper_core::ListedTrait::get_or( & t___2453, i__290, Direction::new(t___2454.clone()));
            let delta__293: Point = direction_delta(dir__292.clone());
            let head__294: Point = temper_core::ListedTrait::get_or( & snake__291.segments(), 0, Point::new(0, 0));
            temper_core::listed::add( & newHeads__289, Point::new(head__294.x().wrapping_add(delta__293.x()), head__294.y().wrapping_add(delta__293.y())), None);
        } else {
            temper_core::listed::add( & newHeads__289, Point::new(-1, -1), None);
        }
        i__290 = i__290.wrapping_add(1);
    }
    let headsList__295: temper_core::List<Point> = temper_core::ListedTrait::to_list( & newHeads__289);
    let dirsList__296: temper_core::List<Direction> = temper_core::ListedTrait::to_list( & newDirs__285);
    let aliveBuilder__297: temper_core::ListBuilder<bool> = temper_core::listed::new_builder();
    let mut i__298: i32 = 0;
    'loop___2834: loop {
        t___2472 = temper_core::ListedTrait::len( & game__282.snakes());
        if ! (Some(i__298) < Some(t___2472)) {
            break;
        }
        t___2473 = game__282.snakes();
        t___2477 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__299: PlayerSnake = temper_core::ListedTrait::get_or( & t___2473, i__298, t___2477.clone());
        if ! temper_core::is::<Alive>(snake__299.status()) {
            temper_core::listed::add( & aliveBuilder__297, false, None);
        } else {
            t___2482 = Point::new(-1, -1);
            let nh__300: Point = temper_core::ListedTrait::get_or( & headsList__295, i__298, t___2482.clone());
            let mut dead__301: bool = false;
            if Some(nh__300.x()) < Some(0) {
                t___1327 = true;
            } else {
                if Some(nh__300.x()) >= Some(game__282.width()) {
                    t___1326 = true;
                } else {
                    if Some(nh__300.y()) < Some(0) {
                        t___1325 = true;
                    } else {
                        t___2488 = nh__300.y();
                        t___2489 = game__282.height();
                        t___1325 = Some(t___2488) >= Some(t___2489);
                    }
                    t___1326 = t___1325;
                }
                t___1327 = t___1326;
            }
            if t___1327 {
                dead__301 = true;
            }
            if ! dead__301 {
                let mut s__302: i32 = 0;
                'loop___2835: loop {
                    t___2491 = temper_core::ListedTrait::len( & snake__299.segments());
                    if ! (Some(s__302) < Some(t___2491.wrapping_sub(1))) {
                        break;
                    }
                    t___2492 = snake__299.segments();
                    t___2493 = Point::new(-2, -2);
                    if point_equals(nh__300.clone(), temper_core::ListedTrait::get_or( & t___2492, s__302, t___2493.clone())) {
                        dead__301 = true;
                    }
                    s__302 = s__302.wrapping_add(1);
                }
            }
            if ! dead__301 {
                let mut j__303: i32 = 0;
                'loop___2836: loop {
                    t___2496 = temper_core::ListedTrait::len( & game__282.snakes());
                    if ! (Some(j__303) < Some(t___2496)) {
                        break;
                    }
                    if Some(j__303) != Some(i__298) {
                        t___2497 = game__282.snakes();
                        t___2501 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
                        let other__304: PlayerSnake = temper_core::ListedTrait::get_or( & t___2497, j__303, t___2501.clone());
                        if temper_core::is::<Alive>(other__304.status()) {
                            let mut s__305: i32 = 0;
                            'loop___2837: loop {
                                t___2506 = temper_core::ListedTrait::len( & other__304.segments());
                                if ! (Some(s__305) < Some(t___2506.wrapping_sub(1))) {
                                    break;
                                }
                                t___2507 = other__304.segments();
                                t___2508 = Point::new(-2, -2);
                                if point_equals(nh__300.clone(), temper_core::ListedTrait::get_or( & t___2507, s__305, t___2508.clone())) {
                                    dead__301 = true;
                                }
                                s__305 = s__305.wrapping_add(1);
                            }
                        }
                    }
                    j__303 = j__303.wrapping_add(1);
                }
            }
            if ! dead__301 {
                let mut j__306: i32 = 0;
                'loop___2838: loop {
                    t___2511 = temper_core::ListedTrait::len( & game__282.snakes());
                    if ! (Some(j__306) < Some(t___2511)) {
                        break;
                    }
                    if Some(j__306) != Some(i__298) {
                        t___2512 = game__282.snakes();
                        t___2516 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
                        let otherSnake__307: PlayerSnake = temper_core::ListedTrait::get_or( & t___2512, j__306, t___2516.clone());
                        if temper_core::is::<Alive>(otherSnake__307.status()) {
                            t___2520 = Point::new(-3, -3);
                            let otherHead__308: Point = temper_core::ListedTrait::get_or( & headsList__295, j__306, t___2520.clone());
                            if point_equals(nh__300.clone(), otherHead__308.clone()) {
                                dead__301 = true;
                            }
                        }
                    }
                    j__306 = j__306.wrapping_add(1);
                }
            }
            temper_core::listed::add( & aliveBuilder__297, ! dead__301, None);
        }
        i__298 = i__298.wrapping_add(1);
    }
    let aliveList__309: temper_core::List<bool> = temper_core::ListedTrait::to_list( & aliveBuilder__297);
    let mut eaterIndex__310: i32 = -1;
    let mut i__311: i32 = 0;
    'loop___2839: loop {
        t___2525 = temper_core::ListedTrait::len( & game__282.snakes());
        if ! (Some(i__311) < Some(t___2525)) {
            break;
        }
        if temper_core::ListedTrait::get_or( & aliveList__309, i__311, false) {
            t___2527 = Point::new(-1, -1);
            let nh__312: Point = temper_core::ListedTrait::get_or( & headsList__295, i__311, t___2527.clone());
            if point_equals(nh__312.clone(), game__282.food()) {
                eaterIndex__310 = i__311;
            }
        }
        i__311 = i__311.wrapping_add(1);
    }
    let resultSnakes__313: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__314: i32 = 0;
    'loop___2840: loop {
        t___2532 = temper_core::ListedTrait::len( & game__282.snakes());
        if ! (Some(i__314) < Some(t___2532)) {
            break;
        }
        t___2533 = game__282.snakes();
        t___2537 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__315: PlayerSnake = temper_core::ListedTrait::get_or( & t___2533, i__314, t___2537.clone());
        if ! temper_core::is::<Alive>(snake__315.status()) {
            temper_core::listed::add( & resultSnakes__313, snake__315.clone(), None);
        } else {
            if ! temper_core::ListedTrait::get_or( & aliveList__309, i__314, false) {
                temper_core::listed::add( & resultSnakes__313, PlayerSnake::new(snake__315.id(), snake__315.segments(), snake__315.direction(), snake__315.score(), PlayerStatus::new(Dead::new())), None);
            } else {
                t___2550 = Point::new(0, 0);
                let nh__316: Point = temper_core::ListedTrait::get_or( & headsList__295, i__314, t___2550.clone());
                t___2552 = snake__315.direction();
                let dir__317: Direction = temper_core::ListedTrait::get_or( & dirsList__296, i__314, t___2552.clone());
                let isEating__318: bool = Some(i__314) == Some(eaterIndex__310);
                if isEating__318 {
                    t___2555 = temper_core::ListedTrait::len( & snake__315.segments());
                    t___1397 = t___2555;
                } else {
                    t___2557 = temper_core::ListedTrait::len( & snake__315.segments());
                    t___1397 = t___2557.wrapping_sub(1);
                }
                let keepLen__319: i32 = t___1397;
                let newSegs__320: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
                temper_core::listed::add( & newSegs__320, nh__316.clone(), None);
                let mut s__321: i32 = 0;
                'loop___2841: while Some(s__321) < Some(keepLen__319) {
                    t___2560 = snake__315.segments();
                    t___2561 = Point::new(0, 0);
                    temper_core::listed::add( & newSegs__320, temper_core::ListedTrait::get_or( & t___2560, s__321, t___2561.clone()), None);
                    s__321 = s__321.wrapping_add(1);
                }
                if isEating__318 {
                    t___2564 = snake__315.score();
                    t___1405 = t___2564.wrapping_add(1);
                } else {
                    t___2565 = snake__315.score();
                    t___1405 = t___2565;
                }
                let newScore__322: i32 = t___1405;
                temper_core::listed::add( & resultSnakes__313, PlayerSnake::new(snake__315.id(), temper_core::ListedTrait::to_list( & newSegs__320), dir__317.clone(), newScore__322, PlayerStatus::new(Alive::new())), None);
            }
        }
        i__314 = i__314.wrapping_add(1);
    }
    let resultSnakesList__323: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & resultSnakes__313);
    let mut t___2572: Point = game__282.food();
    let mut newFood__324: Point = t___2572.clone();
    let mut t___2573: i32 = game__282.rng_seed();
    let mut newSeed__325: i32 = t___2573;
    if Some(eaterIndex__310) >= Some(0) {
        let allSegs__326: temper_core::List<Point> = collectAllSegments__96(resultSnakesList__323.clone());
        t___2575 = game__282.width();
        t___2576 = game__282.height();
        t___2577 = game__282.rng_seed();
        let foodResult__327: FoodPlacement = placeFood__93(allSegs__326.clone(), t___2575, t___2576, t___2577);
        t___2579 = foodResult__327.point();
        newFood__324 = t___2579.clone();
        t___2580 = foodResult__327.seed();
        newSeed__325 = t___2580;
    }
    let mut t___2581: i32 = game__282.width();
    let mut t___2582: i32 = game__282.height();
    let mut t___2583: i32 = game__282.tick_count();
    return MultiSnakeGame::new(t___2581, t___2582, resultSnakesList__323.clone(), newFood__324.clone(), newSeed__325, t___2583.wrapping_add(1));
}
pub fn change_player_direction(game__328: MultiSnakeGame, playerId__329: i32, dir__330: Direction) -> MultiSnakeGame {
    let mut t___2402: i32;
    let mut t___2403: temper_core::List<PlayerSnake>;
    let mut t___2407: PlayerSnake;
    let mut t___2412: Direction;
    let mut t___2413: i32;
    let mut t___2414: temper_core::List<Point>;
    let mut t___2415: i32;
    let mut t___2416: PlayerStatus;
    let mut t___1250: bool;
    let mut t___1251: bool;
    let newSnakes__332: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__333: i32 = 0;
    'loop___2842: loop {
        t___2402 = temper_core::ListedTrait::len( & game__328.snakes());
        if ! (Some(i__333) < Some(t___2402)) {
            break;
        }
        t___2403 = game__328.snakes();
        t___2407 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__334: PlayerSnake = temper_core::ListedTrait::get_or( & t___2403, i__333, t___2407.clone());
        if Some(snake__334.id()) == Some(playerId__329) {
            if temper_core::is::<Alive>(snake__334.status()) {
                t___2412 = snake__334.direction();
                t___1250 = ! is_opposite(t___2412.clone(), dir__330.clone());
            } else {
                t___1250 = false;
            }
            t___1251 = t___1250;
        } else {
            t___1251 = false;
        }
        if t___1251 {
            t___2413 = snake__334.id();
            t___2414 = snake__334.segments();
            t___2415 = snake__334.score();
            t___2416 = snake__334.status();
            temper_core::listed::add( & newSnakes__332, PlayerSnake::new(t___2413, t___2414.clone(), dir__330.clone(), t___2415, t___2416.clone()), None);
        } else {
            temper_core::listed::add( & newSnakes__332, snake__334.clone(), None);
        }
        i__333 = i__333.wrapping_add(1);
    }
    return MultiSnakeGame::new(game__328.width(), game__328.height(), temper_core::ListedTrait::to_list( & newSnakes__332), game__328.food(), game__328.rng_seed(), game__328.tick_count());
}
pub fn is_multi_game_over(game__335: MultiSnakeGame) -> bool {
    let return__84: bool;
    let mut t___2387: i32;
    let mut t___2388: temper_core::List<PlayerSnake>;
    let mut t___2392: PlayerSnake;
    let mut aliveCount__337: i32 = 0;
    let mut i__338: i32 = 0;
    'loop___2843: loop {
        t___2387 = temper_core::ListedTrait::len( & game__335.snakes());
        if ! (Some(i__338) < Some(t___2387)) {
            break;
        }
        t___2388 = game__335.snakes();
        t___2392 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__339: PlayerSnake = temper_core::ListedTrait::get_or( & t___2388, i__338, t___2392.clone());
        if temper_core::is::<Alive>(snake__339.status()) {
            aliveCount__337 = aliveCount__337.wrapping_add(1);
        }
        i__338 = i__338.wrapping_add(1);
    }
    if Some(temper_core::ListedTrait::len( & game__335.snakes())) == Some(0) {
        return__84 = false;
    } else {
        if Some(temper_core::ListedTrait::len( & game__335.snakes())) == Some(1) {
            return__84 = Some(aliveCount__337) == Some(0);
        } else {
            return__84 = Some(aliveCount__337) <= Some(1);
        }
    }
    return return__84;
}
pub fn add_player(game__340: MultiSnakeGame, seed__341: i32) -> MultiSnakeGame {
    let mut t___2365: i32;
    let mut t___2366: temper_core::List<PlayerSnake>;
    let mut t___2370: PlayerSnake;
    let newId__343: i32 = temper_core::ListedTrait::len( & game__340.snakes());
    let spawn__344: SpawnInfo = spawnPosition__95(game__340.width(), game__340.height(), newId__343, seed__341);
    let dir__345: Direction = spawn__344.direction();
    let delta__346: Point = direction_delta(dir__345.clone());
    let startX__347: i32 = spawn__344.point().x();
    let startY__348: i32 = spawn__344.point().y();
    let segments__349: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(startX__347, startY__348), Point::new(startX__347.wrapping_sub(delta__346.x()), startY__348.wrapping_sub(delta__346.y())), Point::new(startX__347.wrapping_sub(delta__346.x().wrapping_mul(2)), startY__348.wrapping_sub(delta__346.y().wrapping_mul(2)))]);
    let newSnake__350: PlayerSnake = PlayerSnake::new(newId__343, segments__349.clone(), dir__345.clone(), 0, PlayerStatus::new(Alive::new()));
    let builder__351: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__352: i32 = 0;
    'loop___2844: loop {
        t___2365 = temper_core::ListedTrait::len( & game__340.snakes());
        if ! (Some(i__352) < Some(t___2365)) {
            break;
        }
        t___2366 = game__340.snakes();
        t___2370 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        temper_core::listed::add( & builder__351, temper_core::ListedTrait::get_or( & t___2366, i__352, t___2370.clone()), None);
        i__352 = i__352.wrapping_add(1);
    }
    temper_core::listed::add( & builder__351, newSnake__350.clone(), None);
    let mut t___2374: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & builder__351);
    let allSegs__353: temper_core::List<Point> = collectAllSegments__96(t___2374.clone());
    let mut t___2376: i32 = game__340.width();
    let mut t___2377: i32 = game__340.height();
    let foodResult__354: FoodPlacement = placeFood__93(allSegs__353.clone(), t___2376, t___2377, seed__341);
    return MultiSnakeGame::new(game__340.width(), game__340.height(), temper_core::ListedTrait::to_list( & builder__351), foodResult__354.point(), foodResult__354.seed(), game__340.tick_count());
}
pub fn remove_player(game__355: MultiSnakeGame, playerId__356: i32) -> MultiSnakeGame {
    let mut t___2327: i32;
    let mut t___2328: temper_core::List<PlayerSnake>;
    let mut t___2332: PlayerSnake;
    let builder__358: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__359: i32 = 0;
    'loop___2845: loop {
        t___2327 = temper_core::ListedTrait::len( & game__355.snakes());
        if ! (Some(i__359) < Some(t___2327)) {
            break;
        }
        t___2328 = game__355.snakes();
        t___2332 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__360: PlayerSnake = temper_core::ListedTrait::get_or( & t___2328, i__359, t___2332.clone());
        if Some(snake__360.id()) != Some(playerId__356) {
            temper_core::listed::add( & builder__358, snake__360.clone(), None);
        }
        i__359 = i__359.wrapping_add(1);
    }
    return MultiSnakeGame::new(game__355.width(), game__355.height(), temper_core::ListedTrait::to_list( & builder__358), game__355.food(), game__355.rng_seed(), game__355.tick_count());
}
pub fn player_head_char(id__373: i32) -> std::sync::Arc<String> {
    let return__88: std::sync::Arc<String>;
    if Some(id__373) == Some(0) {
        return__88 = std::sync::Arc::new("@".to_string());
    } else {
        if Some(id__373) == Some(1) {
            return__88 = std::sync::Arc::new("#".to_string());
        } else {
            if Some(id__373) == Some(2) {
                return__88 = std::sync::Arc::new("$".to_string());
            } else {
                if Some(id__373) == Some(3) {
                    return__88 = std::sync::Arc::new("%".to_string());
                } else {
                    return__88 = std::sync::Arc::new("&".to_string());
                }
            }
        }
    }
    return return__88.clone();
}
pub fn player_body_char(id__375: i32) -> std::sync::Arc<String> {
    let return__89: std::sync::Arc<String>;
    if Some(id__375) == Some(0) {
        return__89 = std::sync::Arc::new("o".to_string());
    } else {
        if Some(id__375) == Some(1) {
            return__89 = std::sync::Arc::new("+".to_string());
        } else {
            if Some(id__375) == Some(2) {
                return__89 = std::sync::Arc::new("~".to_string());
            } else {
                if Some(id__375) == Some(3) {
                    return__89 = std::sync::Arc::new("=".to_string());
                } else {
                    return__89 = std::sync::Arc::new(".".to_string());
                }
            }
        }
    }
    return return__89.clone();
}
fn multiCellChar__97(game__377: MultiSnakeGame, p__378: Point) -> std::sync::Arc<String> {
    let return__90: std::sync::Arc<String>;
    let mut t___2297: i32;
    let mut t___2298: temper_core::List<PlayerSnake>;
    let mut t___2302: PlayerSnake;
    let mut t___2309: i32;
    let mut t___2311: i32;
    let mut t___2312: temper_core::List<PlayerSnake>;
    let mut t___2316: PlayerSnake;
    let mut t___2319: i32;
    let mut t___2320: temper_core::List<Point>;
    let mut t___2321: Point;
    let mut t___2322: Point;
    let mut t___2323: i32;
    let mut t___2324: Point;
    'fn__379: {
        let mut i__380: i32 = 0;
        'loop___2846: loop {
            t___2297 = temper_core::ListedTrait::len( & game__377.snakes());
            if ! (Some(i__380) < Some(t___2297)) {
                break;
            }
            t___2298 = game__377.snakes();
            t___2302 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
            let snake__381: PlayerSnake = temper_core::ListedTrait::get_or( & t___2298, i__380, t___2302.clone());
            if Some(temper_core::ListedTrait::len( & snake__381.segments())) > Some(0) {
                let head__382: Point = temper_core::ListedTrait::get_or( & snake__381.segments(), 0, Point::new(-1, -1));
                if point_equals(p__378.clone(), head__382.clone()) {
                    t___2309 = snake__381.id();
                    return__90 = player_head_char(t___2309);
                    break 'fn__379;
                }
            }
            i__380 = i__380.wrapping_add(1);
        }
        let mut i__383: i32 = 0;
        'loop___2847: loop {
            t___2311 = temper_core::ListedTrait::len( & game__377.snakes());
            if ! (Some(i__383) < Some(t___2311)) {
                break;
            }
            t___2312 = game__377.snakes();
            t___2316 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
            let snake__384: PlayerSnake = temper_core::ListedTrait::get_or( & t___2312, i__383, t___2316.clone());
            let mut j__385: i32 = 1;
            'loop___2848: loop {
                t___2319 = temper_core::ListedTrait::len( & snake__384.segments());
                if ! (Some(j__385) < Some(t___2319)) {
                    break;
                }
                t___2320 = snake__384.segments();
                t___2321 = Point::new(-1, -1);
                t___2322 = temper_core::ListedTrait::get_or( & t___2320, j__385, t___2321.clone());
                if point_equals(p__378.clone(), t___2322.clone()) {
                    t___2323 = snake__384.id();
                    return__90 = player_body_char(t___2323);
                    break 'fn__379;
                }
                j__385 = j__385.wrapping_add(1);
            }
            i__383 = i__383.wrapping_add(1);
        }
        t___2324 = game__377.food();
        if point_equals(p__378.clone(), t___2324.clone()) {
            return__90 = std::sync::Arc::new("*".to_string());
            break 'fn__379;
        }
        return__90 = std::sync::Arc::new(" ".to_string());
    }
    return return__90.clone();
}
pub fn multi_render(game__361: MultiSnakeGame) -> std::sync::Arc<String> {
    let mut t___2264: i32;
    let mut t___2267: i32;
    let mut t___2269: i32;
    let mut t___2275: i32;
    let mut t___2279: i32;
    let mut t___2280: temper_core::List<PlayerSnake>;
    let mut t___2284: PlayerSnake;
    let mut t___2286: PlayerStatus;
    let mut t___1121: std::sync::Arc<String>;
    let sb__363: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    temper_core::string::builder::append( & sb__363, "\x1b[2J\x1b[H");
    temper_core::string::builder::append( & sb__363, "#");
    let mut x__364: i32 = 0;
    'loop___2849: loop {
        t___2264 = game__361.width();
        if ! (Some(x__364) < Some(t___2264)) {
            break;
        }
        temper_core::string::builder::append( & sb__363, "#");
        x__364 = x__364.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__363, "#\x0d\x0a");
    let mut y__365: i32 = 0;
    'loop___2850: loop {
        t___2267 = game__361.height();
        if ! (Some(y__365) < Some(t___2267)) {
            break;
        }
        temper_core::string::builder::append( & sb__363, "#");
        let mut x__366: i32 = 0;
        'loop___2851: loop {
            t___2269 = game__361.width();
            if ! (Some(x__366) < Some(t___2269)) {
                break;
            }
            let p__367: Point = Point::new(x__366, y__365);
            temper_core::string::builder::append( & sb__363, multiCellChar__97(game__361.clone(), p__367.clone()));
            x__366 = x__366.wrapping_add(1);
        }
        temper_core::string::builder::append( & sb__363, "#\x0d\x0a");
        y__365 = y__365.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__363, "#");
    let mut x__368: i32 = 0;
    'loop___2852: loop {
        t___2275 = game__361.width();
        if ! (Some(x__368) < Some(t___2275)) {
            break;
        }
        temper_core::string::builder::append( & sb__363, "#");
        x__368 = x__368.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__363, "#\x0d\x0a");
    let mut i__369: i32 = 0;
    'loop___2853: loop {
        t___2279 = temper_core::ListedTrait::len( & game__361.snakes());
        if ! (Some(i__369) < Some(t___2279)) {
            break;
        }
        t___2280 = game__361.snakes();
        t___2284 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__370: PlayerSnake = temper_core::ListedTrait::get_or( & t___2280, i__369, t___2284.clone());
        t___2286 = snake__370.status();
        if temper_core::is::<Alive>(t___2286.clone()) {
            t___1121 = std::sync::Arc::new("Playing".to_string());
        } else {
            if temper_core::is::<Dead>(t___2286.clone()) {
                t___1121 = std::sync::Arc::new("DEAD".to_string());
            } else {
                t___1121 = std::sync::Arc::new("".to_string());
            }
        }
        let statusText__371: std::sync::Arc<String> = t___1121.clone();
        let symbol__372: std::sync::Arc<String> = player_head_char(snake__370.id());
        temper_core::string::builder::append( & sb__363, std::sync::Arc::new(format!("P{} {}: {}  {}\x0d\x0a", snake__370.id(), symbol__372.clone(), snake__370.score(), statusText__371.clone())));
        i__369 = i__369.wrapping_add(1);
    }
    return temper_core::string::builder::to_string( & sb__363);
}
pub fn direction_to_string(dir__386: Direction) -> std::sync::Arc<String> {
    let return__91: std::sync::Arc<String>;
    if temper_core::is::<Up>(dir__386.clone()) {
        return__91 = std::sync::Arc::new("up".to_string());
    } else {
        if temper_core::is::<Down>(dir__386.clone()) {
            return__91 = std::sync::Arc::new("down".to_string());
        } else {
            if temper_core::is::<Left>(dir__386.clone()) {
                return__91 = std::sync::Arc::new("left".to_string());
            } else {
                if temper_core::is::<Right>(dir__386.clone()) {
                    return__91 = std::sync::Arc::new("right".to_string());
                } else {
                    return__91 = std::sync::Arc::new("right".to_string());
                }
            }
        }
    }
    return return__91.clone();
}
pub fn string_to_direction(s__388: impl temper_core::ToArcString) -> Option<Direction> {
    let s__388 = s__388.to_arc_string();
    let return__92: Option<Direction>;
    'fn__389: {
        if Some(s__388.as_str()) == Some("up") {
            return__92 = Some(Direction::new(Up::new()));
            break 'fn__389;
        }
        if Some(s__388.as_str()) == Some("down") {
            return__92 = Some(Direction::new(Down::new()));
            break 'fn__389;
        }
        if Some(s__388.as_str()) == Some("left") {
            return__92 = Some(Direction::new(Left::new()));
            break 'fn__389;
        }
        if Some(s__388.as_str()) == Some("right") {
            return__92 = Some(Direction::new(Right::new()));
            break 'fn__389;
        }
        return__92 = None;
    }
    return return__92.clone();
}
