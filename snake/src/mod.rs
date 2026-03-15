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
    pub fn new(x__110: i32, y__111: i32) -> Point {
        let x;
        let y;
        x = x__110;
        y = y__111;
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
    pub fn new(value__125: i32, nextSeed__126: i32) -> RandomResult {
        let value;
        let next_seed;
        value = value__125;
        next_seed = nextSeed__126;
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
    pub fn new(width__143: i32, height__144: i32, snake__145: impl temper_core::ToList<Point>, direction__146: Direction, food__147: Point, score__148: i32, status__149: GameStatus, rngSeed__150: i32) -> SnakeGame {
        let snake__145 = snake__145.to_list();
        let width;
        let height;
        let snake;
        let direction;
        let food;
        let score;
        let status;
        let rng_seed;
        width = width__143;
        height = height__144;
        snake = snake__145.clone();
        direction = direction__146.clone();
        food = food__147.clone();
        score = score__148;
        status = status__149.clone();
        rng_seed = rngSeed__150;
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
    pub fn new(point__154: Point, seed__155: i32) -> FoodPlacement {
        let point;
        let seed;
        point = point__154.clone();
        seed = seed__155;
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
    pub fn new(id__222: i32, segments__223: impl temper_core::ToList<Point>, direction__224: Direction, score__225: i32, status__226: PlayerStatus) -> PlayerSnake {
        let segments__223 = segments__223.to_list();
        let id;
        let segments;
        let direction;
        let score;
        let status;
        id = id__222;
        segments = segments__223.clone();
        direction = direction__224.clone();
        score = score__225;
        status = status__226.clone();
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
    pub fn new(width__234: i32, height__235: i32, snakes__236: impl temper_core::ToList<PlayerSnake>, food__237: Point, rngSeed__238: i32, tickCount__239: i32) -> MultiSnakeGame {
        let snakes__236 = snakes__236.to_list();
        let width;
        let height;
        let snakes;
        let food;
        let rng_seed;
        let tick_count;
        width = width__234;
        height = height__235;
        snakes = snakes__236.clone();
        food = food__237.clone();
        rng_seed = rngSeed__238;
        tick_count = tickCount__239;
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
    pub fn new(point__259: Point, direction__260: Direction) -> SpawnInfo {
        let point;
        let direction;
        point = point__259.clone();
        direction = direction__260.clone();
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
pub fn r#move(head__97: Point, body__98: impl temper_core::ToList<Point>, food__99: Point, width__100: i32, height__101: i32) -> Direction {
    let body__98 = body__98.to_list();
    return Direction::new(Right::new());
}
pub fn point_equals(a__114: Point, b__115: Point) -> bool {
    let return__49: bool;
    let mut t___2788: i32;
    let mut t___2789: i32;
    if Some(a__114.x()) == Some(b__115.x()) {
        t___2788 = a__114.y();
        t___2789 = b__115.y();
        return__49 = Some(t___2788) == Some(t___2789);
    } else {
        return__49 = false;
    }
    return return__49;
}
pub fn is_opposite(a__117: Direction, b__118: Direction) -> bool {
    let return__50: bool;
    if temper_core::is::<Up>(a__117.clone()) {
        return__50 = temper_core::is::<Down>(b__118.clone());
    } else {
        if temper_core::is::<Down>(a__117.clone()) {
            return__50 = temper_core::is::<Up>(b__118.clone());
        } else {
            if temper_core::is::<Left>(a__117.clone()) {
                return__50 = temper_core::is::<Right>(b__118.clone());
            } else {
                if temper_core::is::<Right>(a__117.clone()) {
                    return__50 = temper_core::is::<Left>(b__118.clone());
                } else {
                    return__50 = false;
                }
            }
        }
    }
    return return__50;
}
pub fn direction_delta(dir__120: Direction) -> Point {
    let return__51: Point;
    if temper_core::is::<Up>(dir__120.clone()) {
        return__51 = Point::new(0, -1);
    } else {
        if temper_core::is::<Down>(dir__120.clone()) {
            return__51 = Point::new(0, 1);
        } else {
            if temper_core::is::<Left>(dir__120.clone()) {
                return__51 = Point::new(-1, 0);
            } else {
                if temper_core::is::<Right>(dir__120.clone()) {
                    return__51 = Point::new(1, 0);
                } else {
                    return__51 = Point::new(0, 0);
                }
            }
        }
    }
    return return__51.clone();
}
pub fn next_random(seed__127: i32, max__128: i32) -> RandomResult {
    let mut t___1684: i32;
    let mut t___1686: i32;
    let raw__130: i32 = seed__127.wrapping_mul(1664525).wrapping_add(1013904223);
    let masked__131: i32 = raw__130 & 2147483647;
    let posVal__132: i32;
    if Some(masked__131) < Some(0) {
        posVal__132 = masked__131.wrapping_neg();
    } else {
        posVal__132 = masked__131;
    }
    let mut value__133: i32 = 0;
    if Some(max__128) > Some(0) {
        'ok___2791: {
            'orelse___514: {
                t___1684 = match temper_core::int_rem(posVal__132, max__128) {
                    Ok(x) => x,
                    _ => break 'orelse___514
                };
                t___1686 = t___1684;
                break 'ok___2791;
            }
            t___1686 = 0;
        }
        value__133 = t___1686;
    }
    return RandomResult::new(value__133, masked__131);
}
fn placeFood__92(snake__156: impl temper_core::ToList<Point>, width__157: i32, height__158: i32, seed__159: i32) -> FoodPlacement {
    let snake__156 = snake__156.to_list();
    let return__60: FoodPlacement;
    let mut t___2755: i32;
    let mut t___2766: Point;
    let mut t___1649: i32;
    let mut t___1651: i32;
    let mut t___1653: i32;
    let mut t___1655: i32;
    'fn__160: {
        let totalCells__161: i32 = width__157.wrapping_mul(height__158);
        let mut currentSeed__162: i32 = seed__159;
        let mut attempt__163: i32 = 0;
        'loop___2804: while Some(attempt__163) < Some(totalCells__161) {
            let result__164: RandomResult = next_random(currentSeed__162, totalCells__161);
            t___2755 = result__164.next_seed();
            currentSeed__162 = t___2755;
            let mut px__165: i32 = 0;
            let mut py__166: i32 = 0;
            if Some(width__157) > Some(0) {
                'ok___2792: {
                    'orelse___515: {
                        t___1649 = match temper_core::int_rem(result__164.value(), width__157) {
                            Ok(x) => x,
                            _ => break 'orelse___515
                        };
                        t___1651 = t___1649;
                        break 'ok___2792;
                    }
                    t___1651 = 0;
                }
                px__165 = t___1651;
                'ok___2793: {
                    'orelse___516: {
                        t___1653 = match temper_core::int_div(result__164.value(), width__157) {
                            Ok(x) => x,
                            _ => break 'orelse___516
                        };
                        t___1655 = t___1653;
                        break 'ok___2793;
                    }
                    t___1655 = 0;
                }
                py__166 = t___1655;
            }
            let candidate__167: Point = Point::new(px__165, py__166);
            let mut occupied__168: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___1 {
                candidate__167: Point, occupied__168: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___1 {
                fn fn__2754(& self, seg__169: Point) {
                    if point_equals(seg__169.clone(), self.candidate__167.clone()) {
                        {
                            * self.occupied__168.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___1 {
                candidate__167: candidate__167.clone(), occupied__168: occupied__168.clone()
            };
            let fn__2754 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | seg__169: Point | closure_group.fn__2754(seg__169))
            };
            temper_core::listed::list_for_each( & snake__156, & ( * fn__2754.clone()));
            if ! temper_core::read_locked( & occupied__168) {
                return__60 = FoodPlacement::new(candidate__167.clone(), currentSeed__162);
                break 'fn__160;
            }
            attempt__163 = attempt__163.wrapping_add(1);
        }
        let mut y__170: i32 = 0;
        'loop___2805: while Some(y__170) < Some(height__158) {
            let mut x__171: i32 = 0;
            'loop___2806: while Some(x__171) < Some(width__157) {
                let candidate__172: Point = Point::new(x__171, y__170);
                let mut free__173: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
                #[derive(Clone)]
                struct ClosureGroup___2 {
                    candidate__172: Point, free__173: std::sync::Arc<std::sync::RwLock<bool>>
                }
                impl ClosureGroup___2 {
                    fn fn__2753(& self, seg__174: Point) {
                        if point_equals(seg__174.clone(), self.candidate__172.clone()) {
                            {
                                * self.free__173.write().unwrap() = false;
                            }
                        }
                    }
                }
                let closure_group = ClosureGroup___2 {
                    candidate__172: candidate__172.clone(), free__173: free__173.clone()
                };
                let fn__2753 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | seg__174: Point | closure_group.fn__2753(seg__174))
                };
                temper_core::listed::list_for_each( & snake__156, & ( * fn__2753.clone()));
                if temper_core::read_locked( & free__173) {
                    return__60 = FoodPlacement::new(candidate__172.clone(), currentSeed__162);
                    break 'fn__160;
                }
                x__171 = x__171.wrapping_add(1);
            }
            y__170 = y__170.wrapping_add(1);
        }
        t___2766 = Point::new(0, 0);
        return__60 = FoodPlacement::new(t___2766.clone(), currentSeed__162);
    }
    return return__60.clone();
}
pub fn new_game(width__175: i32, height__176: i32, seed__177: i32) -> SnakeGame {
    let mut t___1632: i32;
    let mut t___1634: i32;
    let mut t___1635: i32;
    let mut t___1637: i32;
    let mut centerX__179: i32 = 0;
    let mut centerY__180: i32 = 0;
    if Some(width__175) > Some(0) {
        t___1632 = width__175.wrapping_div(2);
        t___1634 = t___1632;
        centerX__179 = t___1634;
    }
    if Some(height__176) > Some(0) {
        t___1635 = height__176.wrapping_div(2);
        t___1637 = t___1635;
        centerY__180 = t___1637;
    }
    let snake__181: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(centerX__179, centerY__180), Point::new(centerX__179.wrapping_sub(1), centerY__180), Point::new(centerX__179.wrapping_sub(2), centerY__180)]);
    let foodResult__182: FoodPlacement = placeFood__92(snake__181.clone(), width__175, height__176, seed__177);
    let mut t___2748: Right = Right::new();
    let mut t___2749: Point = foodResult__182.point();
    let mut t___2750: Playing = Playing::new();
    let mut t___2751: i32 = foodResult__182.seed();
    return SnakeGame::new(width__175, height__176, snake__181.clone(), Direction::new(t___2748.clone()), t___2749.clone(), 0, GameStatus::new(t___2750.clone()), t___2751);
}
pub fn change_direction(game__183: SnakeGame, dir__184: Direction) -> SnakeGame {
    let return__62: SnakeGame;
    let mut t___2736: i32;
    let mut t___2737: i32;
    let mut t___2738: temper_core::List<Point>;
    let mut t___2739: Point;
    let mut t___2740: i32;
    let mut t___2741: GameStatus;
    let mut t___2742: i32;
    if is_opposite(game__183.direction(), dir__184.clone()) {
        return__62 = game__183.clone();
    } else {
        t___2736 = game__183.width();
        t___2737 = game__183.height();
        t___2738 = game__183.snake();
        t___2739 = game__183.food();
        t___2740 = game__183.score();
        t___2741 = game__183.status();
        t___2742 = game__183.rng_seed();
        return__62 = SnakeGame::new(t___2736, t___2737, t___2738.clone(), dir__184.clone(), t___2739.clone(), t___2740, t___2741.clone(), t___2742);
    }
    return return__62.clone();
}
pub fn tick(game__186: SnakeGame) -> SnakeGame {
    let return__63: SnakeGame;
    let mut t___2676: i32;
    let mut t___2677: i32;
    let mut t___2678: i32;
    let mut t___2679: i32;
    let mut t___2680: temper_core::List<Point>;
    let mut t___2681: Direction;
    let mut t___2682: Point;
    let mut t___2683: i32;
    let mut t___2684: GameOver;
    let mut t___2685: i32;
    let mut t___2689: i32;
    let mut t___2691: i32;
    let mut t___2692: temper_core::List<Point>;
    let mut t___2693: Point;
    let mut t___2695: i32;
    let mut t___2696: i32;
    let mut t___2697: temper_core::List<Point>;
    let mut t___2698: Direction;
    let mut t___2699: Point;
    let mut t___2700: i32;
    let mut t___2701: GameOver;
    let mut t___2702: i32;
    let mut t___2707: i32;
    let mut t___2709: i32;
    let mut t___2710: temper_core::List<Point>;
    let mut t___2711: Point;
    let mut t___2716: i32;
    let mut t___2717: i32;
    let mut t___2718: i32;
    let mut t___2720: i32;
    let mut t___2721: i32;
    let mut t___2722: Direction;
    let mut t___2723: Point;
    let mut t___2724: Playing;
    let mut t___2725: i32;
    let mut t___2727: i32;
    let mut t___2728: i32;
    let mut t___2729: Direction;
    let mut t___2730: Point;
    let mut t___2731: i32;
    let mut t___2732: GameStatus;
    let mut t___2733: i32;
    let mut t___1559: bool;
    let mut t___1560: bool;
    let mut t___1561: bool;
    'fn__187: {
        if temper_core::is::<GameOver>(game__186.status()) {
            return__63 = game__186.clone();
            break 'fn__187;
        }
        let delta__188: Point = direction_delta(game__186.direction());
        let head__189: Point = temper_core::ListedTrait::get_or( & game__186.snake(), 0, Point::new(0, 0));
        let newHead__190: Point = Point::new(head__189.x().wrapping_add(delta__188.x()), head__189.y().wrapping_add(delta__188.y()));
        if Some(newHead__190.x()) < Some(0) {
            t___1561 = true;
        } else {
            if Some(newHead__190.x()) >= Some(game__186.width()) {
                t___1560 = true;
            } else {
                if Some(newHead__190.y()) < Some(0) {
                    t___1559 = true;
                } else {
                    t___2676 = newHead__190.y();
                    t___2677 = game__186.height();
                    t___1559 = Some(t___2676) >= Some(t___2677);
                }
                t___1560 = t___1559;
            }
            t___1561 = t___1560;
        }
        if t___1561 {
            t___2678 = game__186.width();
            t___2679 = game__186.height();
            t___2680 = game__186.snake();
            t___2681 = game__186.direction();
            t___2682 = game__186.food();
            t___2683 = game__186.score();
            t___2684 = GameOver::new();
            t___2685 = game__186.rng_seed();
            return__63 = SnakeGame::new(t___2678, t___2679, t___2680.clone(), t___2681.clone(), t___2682.clone(), t___2683, GameStatus::new(t___2684.clone()), t___2685);
            break 'fn__187;
        }
        let eating__191: bool = point_equals(newHead__190.clone(), game__186.food());
        let checkLength__192: i32;
        if eating__191 {
            t___2689 = temper_core::ListedTrait::len( & game__186.snake());
            checkLength__192 = t___2689;
        } else {
            t___2691 = temper_core::ListedTrait::len( & game__186.snake());
            checkLength__192 = t___2691.wrapping_sub(1);
        }
        let mut i__193: i32 = 0;
        'loop___2807: while Some(i__193) < Some(checkLength__192) {
            t___2692 = game__186.snake();
            t___2693 = Point::new(-1, -1);
            if point_equals(newHead__190.clone(), temper_core::ListedTrait::get_or( & t___2692, i__193, t___2693.clone())) {
                t___2695 = game__186.width();
                t___2696 = game__186.height();
                t___2697 = game__186.snake();
                t___2698 = game__186.direction();
                t___2699 = game__186.food();
                t___2700 = game__186.score();
                t___2701 = GameOver::new();
                t___2702 = game__186.rng_seed();
                return__63 = SnakeGame::new(t___2695, t___2696, t___2697.clone(), t___2698.clone(), t___2699.clone(), t___2700, GameStatus::new(t___2701.clone()), t___2702);
                break 'fn__187;
            }
            i__193 = i__193.wrapping_add(1);
        }
        let newSnakeBuilder__194: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
        temper_core::listed::add( & newSnakeBuilder__194, newHead__190.clone(), None);
        let keepLength__195: i32;
        if eating__191 {
            t___2707 = temper_core::ListedTrait::len( & game__186.snake());
            keepLength__195 = t___2707;
        } else {
            t___2709 = temper_core::ListedTrait::len( & game__186.snake());
            keepLength__195 = t___2709.wrapping_sub(1);
        }
        let mut i__196: i32 = 0;
        'loop___2808: while Some(i__196) < Some(keepLength__195) {
            t___2710 = game__186.snake();
            t___2711 = Point::new(0, 0);
            temper_core::listed::add( & newSnakeBuilder__194, temper_core::ListedTrait::get_or( & t___2710, i__196, t___2711.clone()), None);
            i__196 = i__196.wrapping_add(1);
        }
        let newSnake__197: temper_core::List<Point> = temper_core::ListedTrait::to_list( & newSnakeBuilder__194);
        if eating__191 {
            let newScore__198: i32 = game__186.score().wrapping_add(1);
            t___2716 = game__186.width();
            t___2717 = game__186.height();
            t___2718 = game__186.rng_seed();
            let foodResult__199: FoodPlacement = placeFood__92(newSnake__197.clone(), t___2716, t___2717, t___2718);
            t___2720 = game__186.width();
            t___2721 = game__186.height();
            t___2722 = game__186.direction();
            t___2723 = foodResult__199.point();
            t___2724 = Playing::new();
            t___2725 = foodResult__199.seed();
            return__63 = SnakeGame::new(t___2720, t___2721, newSnake__197.clone(), t___2722.clone(), t___2723.clone(), newScore__198, GameStatus::new(t___2724.clone()), t___2725);
        } else {
            t___2727 = game__186.width();
            t___2728 = game__186.height();
            t___2729 = game__186.direction();
            t___2730 = game__186.food();
            t___2731 = game__186.score();
            t___2732 = game__186.status();
            t___2733 = game__186.rng_seed();
            return__63 = SnakeGame::new(t___2727, t___2728, newSnake__197.clone(), t___2729.clone(), t___2730.clone(), t___2731, t___2732.clone(), t___2733);
        }
    }
    return return__63.clone();
}
fn cellChar__93(game__209: SnakeGame, p__210: Point) -> std::sync::Arc<String> {
    let return__65: std::sync::Arc<String>;
    let mut t___2655: i32;
    let mut t___2656: temper_core::List<Point>;
    let mut t___2657: Point;
    let mut t___2658: Point;
    let mut t___2659: Point;
    'fn__211: {
        let head__212: Point = temper_core::ListedTrait::get_or( & game__209.snake(), 0, Point::new(-1, -1));
        if point_equals(p__210.clone(), head__212.clone()) {
            return__65 = std::sync::Arc::new("@".to_string());
            break 'fn__211;
        }
        let mut i__213: i32 = 1;
        'loop___2809: loop {
            t___2655 = temper_core::ListedTrait::len( & game__209.snake());
            if ! (Some(i__213) < Some(t___2655)) {
                break;
            }
            t___2656 = game__209.snake();
            t___2657 = Point::new(-1, -1);
            t___2658 = temper_core::ListedTrait::get_or( & t___2656, i__213, t___2657.clone());
            if point_equals(p__210.clone(), t___2658.clone()) {
                return__65 = std::sync::Arc::new("o".to_string());
                break 'fn__211;
            }
            i__213 = i__213.wrapping_add(1);
        }
        t___2659 = game__209.food();
        if point_equals(p__210.clone(), t___2659.clone()) {
            return__65 = std::sync::Arc::new("*".to_string());
            break 'fn__211;
        }
        return__65 = std::sync::Arc::new(" ".to_string());
    }
    return return__65.clone();
}
pub fn render(game__200: SnakeGame) -> std::sync::Arc<String> {
    let mut t___2630: i32;
    let mut t___2633: i32;
    let mut t___2635: i32;
    let mut t___2641: i32;
    let sb__202: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    temper_core::string::builder::append( & sb__202, "\x1b[2J\x1b[H");
    temper_core::string::builder::append( & sb__202, "#");
    let mut x__203: i32 = 0;
    'loop___2810: loop {
        t___2630 = game__200.width();
        if ! (Some(x__203) < Some(t___2630)) {
            break;
        }
        temper_core::string::builder::append( & sb__202, "#");
        x__203 = x__203.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__202, "#\x0d\x0a");
    let mut y__204: i32 = 0;
    'loop___2811: loop {
        t___2633 = game__200.height();
        if ! (Some(y__204) < Some(t___2633)) {
            break;
        }
        temper_core::string::builder::append( & sb__202, "#");
        let mut x__205: i32 = 0;
        'loop___2812: loop {
            t___2635 = game__200.width();
            if ! (Some(x__205) < Some(t___2635)) {
                break;
            }
            let p__206: Point = Point::new(x__205, y__204);
            temper_core::string::builder::append( & sb__202, cellChar__93(game__200.clone(), p__206.clone()));
            x__205 = x__205.wrapping_add(1);
        }
        temper_core::string::builder::append( & sb__202, "#\x0d\x0a");
        y__204 = y__204.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__202, "#");
    let mut x__207: i32 = 0;
    'loop___2813: loop {
        t___2641 = game__200.width();
        if ! (Some(x__207) < Some(t___2641)) {
            break;
        }
        temper_core::string::builder::append( & sb__202, "#");
        x__207 = x__207.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__202, "#\x0d\x0a");
    let statusText__208: std::sync::Arc<String>;
    let mut t___2644: GameStatus = game__200.status();
    if temper_core::is::<Playing>(t___2644.clone()) {
        statusText__208 = std::sync::Arc::new("Playing".to_string());
    } else {
        if temper_core::is::<GameOver>(t___2644.clone()) {
            statusText__208 = std::sync::Arc::new("GAME OVER".to_string());
        } else {
            statusText__208 = std::sync::Arc::new("".to_string());
        }
    }
    temper_core::string::builder::append( & sb__202, std::sync::Arc::new(format!("Score: {}  {}\x0d\x0a", game__200.score(), statusText__208.clone())));
    return temper_core::string::builder::to_string( & sb__202);
}
fn spawnPosition__94(width__261: i32, height__262: i32, index__263: i32, total__264: i32) -> SpawnInfo {
    let return__79: SpawnInfo;
    let mut t___2615: Point;
    let mut t___2616: Right;
    let mut t___2618: Point;
    let mut t___2619: Left;
    let mut t___2621: Point;
    let mut t___2622: Down;
    let mut t___2624: Point;
    let mut t___2625: Up;
    let mut t___1458: i32;
    let mut t___1460: i32;
    let mut t___1461: i32;
    let mut t___1463: i32;
    let mut t___1464: i32;
    let mut t___1466: i32;
    let mut t___1467: i32;
    let mut t___1469: i32;
    let mut t___1470: i32;
    let mut t___1472: i32;
    'fn__265: {
        let mut cx__266: i32 = 0;
        let mut cy__267: i32 = 0;
        if Some(width__261) > Some(0) {
            t___1458 = width__261.wrapping_div(2);
            t___1460 = t___1458;
            cx__266 = t___1460;
        }
        if Some(height__262) > Some(0) {
            t___1461 = height__262.wrapping_div(2);
            t___1463 = t___1461;
            cy__267 = t___1463;
        }
        let mut qx__268: i32 = 0;
        let mut qy__269: i32 = 0;
        if Some(width__261) > Some(0) {
            t___1464 = width__261.wrapping_div(4);
            t___1466 = t___1464;
            qx__268 = t___1466;
        }
        if Some(height__262) > Some(0) {
            t___1467 = height__262.wrapping_div(4);
            t___1469 = t___1467;
            qy__269 = t___1469;
        }
        let mut slot__270: i32 = 0;
        if Some(total__264) > Some(0) {
            t___1470 = index__263.wrapping_rem(4);
            t___1472 = t___1470;
            slot__270 = t___1472;
        }
        if Some(slot__270) == Some(0) {
            t___2615 = Point::new(qx__268, cy__267);
            t___2616 = Right::new();
            return__79 = SpawnInfo::new(t___2615.clone(), Direction::new(t___2616.clone()));
            break 'fn__265;
        }
        if Some(slot__270) == Some(1) {
            t___2618 = Point::new(width__261.wrapping_sub(qx__268).wrapping_sub(1), cy__267);
            t___2619 = Left::new();
            return__79 = SpawnInfo::new(t___2618.clone(), Direction::new(t___2619.clone()));
            break 'fn__265;
        }
        if Some(slot__270) == Some(2) {
            t___2621 = Point::new(cx__266, qy__269);
            t___2622 = Down::new();
            return__79 = SpawnInfo::new(t___2621.clone(), Direction::new(t___2622.clone()));
            break 'fn__265;
        }
        t___2624 = Point::new(cx__266, height__262.wrapping_sub(qy__269).wrapping_sub(1));
        t___2625 = Up::new();
        return__79 = SpawnInfo::new(t___2624.clone(), Direction::new(t___2625.clone()));
    }
    return return__79.clone();
}
fn collectAllSegments__95(snakes__271: impl temper_core::ToList<PlayerSnake>) -> temper_core::List<Point> {
    let snakes__271 = snakes__271.to_list();
    let mut t___2602: i32;
    let mut t___2606: PlayerSnake;
    let mut t___2609: i32;
    let mut t___2610: temper_core::List<Point>;
    let mut t___2611: Point;
    let builder__273: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
    let mut i__274: i32 = 0;
    'loop___2814: loop {
        t___2602 = temper_core::ListedTrait::len( & snakes__271);
        if ! (Some(i__274) < Some(t___2602)) {
            break;
        }
        t___2606 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__275: PlayerSnake = temper_core::ListedTrait::get_or( & snakes__271, i__274, t___2606.clone());
        let mut j__276: i32 = 0;
        'loop___2815: loop {
            t___2609 = temper_core::ListedTrait::len( & snake__275.segments());
            if ! (Some(j__276) < Some(t___2609)) {
                break;
            }
            t___2610 = snake__275.segments();
            t___2611 = Point::new(0, 0);
            temper_core::listed::add( & builder__273, temper_core::ListedTrait::get_or( & t___2610, j__276, t___2611.clone()), None);
            j__276 = j__276.wrapping_add(1);
        }
        i__274 = i__274.wrapping_add(1);
    }
    return temper_core::ListedTrait::to_list( & builder__273);
}
pub fn new_multi_game(width__240: i32, height__241: i32, numPlayers__242: i32, seed__243: i32) -> MultiSnakeGame {
    let mut t___2591: Alive;
    let snakeBuilder__245: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut currentSeed__246: i32 = seed__243;
    let mut i__247: i32 = 0;
    'loop___2816: while Some(i__247) < Some(numPlayers__242) {
        let spawn__248: SpawnInfo = spawnPosition__94(width__240, height__241, i__247, numPlayers__242);
        let dir__249: Direction = spawn__248.direction();
        let startX__250: i32 = spawn__248.point().x();
        let startY__251: i32 = spawn__248.point().y();
        let delta__252: Point = direction_delta(dir__249.clone());
        let segments__253: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(startX__250, startY__251), Point::new(startX__250.wrapping_sub(delta__252.x()), startY__251.wrapping_sub(delta__252.y())), Point::new(startX__250.wrapping_sub(delta__252.x().wrapping_mul(2)), startY__251.wrapping_sub(delta__252.y().wrapping_mul(2)))]);
        t___2591 = Alive::new();
        temper_core::listed::add( & snakeBuilder__245, PlayerSnake::new(i__247, segments__253.clone(), dir__249.clone(), 0, PlayerStatus::new(t___2591.clone())), None);
        i__247 = i__247.wrapping_add(1);
    }
    let mut t___2594: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & snakeBuilder__245);
    let allSegments__254: temper_core::List<Point> = collectAllSegments__95(t___2594.clone());
    let foodResult__255: FoodPlacement = placeFood__92(allSegments__254.clone(), width__240, height__241, currentSeed__246);
    let mut t___2597: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & snakeBuilder__245);
    let mut t___2598: Point = foodResult__255.point();
    let mut t___2599: i32 = foodResult__255.seed();
    return MultiSnakeGame::new(width__240, height__241, t___2597.clone(), t___2598.clone(), t___2599, 0);
}
pub fn multi_tick(game__277: MultiSnakeGame, directions__278: impl temper_core::ToList<Direction>) -> MultiSnakeGame {
    let directions__278 = directions__278.to_list();
    let mut t___2420: i32;
    let mut t___2421: temper_core::List<PlayerSnake>;
    let mut t___2425: PlayerSnake;
    let mut t___2427: Direction;
    let mut t___2435: i32;
    let mut t___2436: temper_core::List<PlayerSnake>;
    let mut t___2440: PlayerSnake;
    let mut t___2444: temper_core::List<Direction>;
    let mut t___2445: Right;
    let mut t___2463: i32;
    let mut t___2464: temper_core::List<PlayerSnake>;
    let mut t___2468: PlayerSnake;
    let mut t___2473: Point;
    let mut t___2479: i32;
    let mut t___2480: i32;
    let mut t___2482: i32;
    let mut t___2483: temper_core::List<Point>;
    let mut t___2484: Point;
    let mut t___2487: i32;
    let mut t___2488: temper_core::List<PlayerSnake>;
    let mut t___2492: PlayerSnake;
    let mut t___2497: i32;
    let mut t___2498: temper_core::List<Point>;
    let mut t___2499: Point;
    let mut t___2502: i32;
    let mut t___2503: temper_core::List<PlayerSnake>;
    let mut t___2507: PlayerSnake;
    let mut t___2511: Point;
    let mut t___2516: i32;
    let mut t___2518: Point;
    let mut t___2523: i32;
    let mut t___2524: temper_core::List<PlayerSnake>;
    let mut t___2528: PlayerSnake;
    let mut t___2541: Point;
    let mut t___2543: Direction;
    let mut t___2546: i32;
    let mut t___2548: i32;
    let mut t___2551: temper_core::List<Point>;
    let mut t___2552: Point;
    let mut t___2555: i32;
    let mut t___2556: i32;
    let mut t___2566: i32;
    let mut t___2567: i32;
    let mut t___2568: i32;
    let mut t___2570: Point;
    let mut t___2571: i32;
    let mut t___1320: bool;
    let mut t___1321: bool;
    let mut t___1322: bool;
    let mut t___1392: i32;
    let mut t___1400: i32;
    let newDirs__280: temper_core::ListBuilder<Direction> = temper_core::listed::new_builder();
    let mut i__281: i32 = 0;
    'loop___2817: loop {
        t___2420 = temper_core::ListedTrait::len( & game__277.snakes());
        if ! (Some(i__281) < Some(t___2420)) {
            break;
        }
        t___2421 = game__277.snakes();
        t___2425 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__282: PlayerSnake = temper_core::ListedTrait::get_or( & t___2421, i__281, t___2425.clone());
        t___2427 = snake__282.direction();
        let inputDir__283: Direction = temper_core::ListedTrait::get_or( & directions__278, i__281, t___2427.clone());
        if is_opposite(snake__282.direction(), inputDir__283.clone()) {
            temper_core::listed::add( & newDirs__280, snake__282.direction(), None);
        } else {
            temper_core::listed::add( & newDirs__280, inputDir__283.clone(), None);
        }
        i__281 = i__281.wrapping_add(1);
    }
    let newHeads__284: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
    let mut i__285: i32 = 0;
    'loop___2818: loop {
        t___2435 = temper_core::ListedTrait::len( & game__277.snakes());
        if ! (Some(i__285) < Some(t___2435)) {
            break;
        }
        t___2436 = game__277.snakes();
        t___2440 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__286: PlayerSnake = temper_core::ListedTrait::get_or( & t___2436, i__285, t___2440.clone());
        if temper_core::is::<Alive>(snake__286.status()) {
            t___2444 = temper_core::ListedTrait::to_list( & newDirs__280);
            t___2445 = Right::new();
            let dir__287: Direction = temper_core::ListedTrait::get_or( & t___2444, i__285, Direction::new(t___2445.clone()));
            let delta__288: Point = direction_delta(dir__287.clone());
            let head__289: Point = temper_core::ListedTrait::get_or( & snake__286.segments(), 0, Point::new(0, 0));
            temper_core::listed::add( & newHeads__284, Point::new(head__289.x().wrapping_add(delta__288.x()), head__289.y().wrapping_add(delta__288.y())), None);
        } else {
            temper_core::listed::add( & newHeads__284, Point::new(-1, -1), None);
        }
        i__285 = i__285.wrapping_add(1);
    }
    let headsList__290: temper_core::List<Point> = temper_core::ListedTrait::to_list( & newHeads__284);
    let dirsList__291: temper_core::List<Direction> = temper_core::ListedTrait::to_list( & newDirs__280);
    let aliveBuilder__292: temper_core::ListBuilder<bool> = temper_core::listed::new_builder();
    let mut i__293: i32 = 0;
    'loop___2819: loop {
        t___2463 = temper_core::ListedTrait::len( & game__277.snakes());
        if ! (Some(i__293) < Some(t___2463)) {
            break;
        }
        t___2464 = game__277.snakes();
        t___2468 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__294: PlayerSnake = temper_core::ListedTrait::get_or( & t___2464, i__293, t___2468.clone());
        if ! temper_core::is::<Alive>(snake__294.status()) {
            temper_core::listed::add( & aliveBuilder__292, false, None);
        } else {
            t___2473 = Point::new(-1, -1);
            let nh__295: Point = temper_core::ListedTrait::get_or( & headsList__290, i__293, t___2473.clone());
            let mut dead__296: bool = false;
            if Some(nh__295.x()) < Some(0) {
                t___1322 = true;
            } else {
                if Some(nh__295.x()) >= Some(game__277.width()) {
                    t___1321 = true;
                } else {
                    if Some(nh__295.y()) < Some(0) {
                        t___1320 = true;
                    } else {
                        t___2479 = nh__295.y();
                        t___2480 = game__277.height();
                        t___1320 = Some(t___2479) >= Some(t___2480);
                    }
                    t___1321 = t___1320;
                }
                t___1322 = t___1321;
            }
            if t___1322 {
                dead__296 = true;
            }
            if ! dead__296 {
                let mut s__297: i32 = 0;
                'loop___2820: loop {
                    t___2482 = temper_core::ListedTrait::len( & snake__294.segments());
                    if ! (Some(s__297) < Some(t___2482.wrapping_sub(1))) {
                        break;
                    }
                    t___2483 = snake__294.segments();
                    t___2484 = Point::new(-2, -2);
                    if point_equals(nh__295.clone(), temper_core::ListedTrait::get_or( & t___2483, s__297, t___2484.clone())) {
                        dead__296 = true;
                    }
                    s__297 = s__297.wrapping_add(1);
                }
            }
            if ! dead__296 {
                let mut j__298: i32 = 0;
                'loop___2821: loop {
                    t___2487 = temper_core::ListedTrait::len( & game__277.snakes());
                    if ! (Some(j__298) < Some(t___2487)) {
                        break;
                    }
                    if Some(j__298) != Some(i__293) {
                        t___2488 = game__277.snakes();
                        t___2492 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
                        let other__299: PlayerSnake = temper_core::ListedTrait::get_or( & t___2488, j__298, t___2492.clone());
                        if temper_core::is::<Alive>(other__299.status()) {
                            let mut s__300: i32 = 0;
                            'loop___2822: loop {
                                t___2497 = temper_core::ListedTrait::len( & other__299.segments());
                                if ! (Some(s__300) < Some(t___2497.wrapping_sub(1))) {
                                    break;
                                }
                                t___2498 = other__299.segments();
                                t___2499 = Point::new(-2, -2);
                                if point_equals(nh__295.clone(), temper_core::ListedTrait::get_or( & t___2498, s__300, t___2499.clone())) {
                                    dead__296 = true;
                                }
                                s__300 = s__300.wrapping_add(1);
                            }
                        }
                    }
                    j__298 = j__298.wrapping_add(1);
                }
            }
            if ! dead__296 {
                let mut j__301: i32 = 0;
                'loop___2823: loop {
                    t___2502 = temper_core::ListedTrait::len( & game__277.snakes());
                    if ! (Some(j__301) < Some(t___2502)) {
                        break;
                    }
                    if Some(j__301) != Some(i__293) {
                        t___2503 = game__277.snakes();
                        t___2507 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
                        let otherSnake__302: PlayerSnake = temper_core::ListedTrait::get_or( & t___2503, j__301, t___2507.clone());
                        if temper_core::is::<Alive>(otherSnake__302.status()) {
                            t___2511 = Point::new(-3, -3);
                            let otherHead__303: Point = temper_core::ListedTrait::get_or( & headsList__290, j__301, t___2511.clone());
                            if point_equals(nh__295.clone(), otherHead__303.clone()) {
                                dead__296 = true;
                            }
                        }
                    }
                    j__301 = j__301.wrapping_add(1);
                }
            }
            temper_core::listed::add( & aliveBuilder__292, ! dead__296, None);
        }
        i__293 = i__293.wrapping_add(1);
    }
    let aliveList__304: temper_core::List<bool> = temper_core::ListedTrait::to_list( & aliveBuilder__292);
    let mut eaterIndex__305: i32 = -1;
    let mut i__306: i32 = 0;
    'loop___2824: loop {
        t___2516 = temper_core::ListedTrait::len( & game__277.snakes());
        if ! (Some(i__306) < Some(t___2516)) {
            break;
        }
        if temper_core::ListedTrait::get_or( & aliveList__304, i__306, false) {
            t___2518 = Point::new(-1, -1);
            let nh__307: Point = temper_core::ListedTrait::get_or( & headsList__290, i__306, t___2518.clone());
            if point_equals(nh__307.clone(), game__277.food()) {
                eaterIndex__305 = i__306;
            }
        }
        i__306 = i__306.wrapping_add(1);
    }
    let resultSnakes__308: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__309: i32 = 0;
    'loop___2825: loop {
        t___2523 = temper_core::ListedTrait::len( & game__277.snakes());
        if ! (Some(i__309) < Some(t___2523)) {
            break;
        }
        t___2524 = game__277.snakes();
        t___2528 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__310: PlayerSnake = temper_core::ListedTrait::get_or( & t___2524, i__309, t___2528.clone());
        if ! temper_core::is::<Alive>(snake__310.status()) {
            temper_core::listed::add( & resultSnakes__308, snake__310.clone(), None);
        } else {
            if ! temper_core::ListedTrait::get_or( & aliveList__304, i__309, false) {
                temper_core::listed::add( & resultSnakes__308, PlayerSnake::new(snake__310.id(), snake__310.segments(), snake__310.direction(), snake__310.score(), PlayerStatus::new(Dead::new())), None);
            } else {
                t___2541 = Point::new(0, 0);
                let nh__311: Point = temper_core::ListedTrait::get_or( & headsList__290, i__309, t___2541.clone());
                t___2543 = snake__310.direction();
                let dir__312: Direction = temper_core::ListedTrait::get_or( & dirsList__291, i__309, t___2543.clone());
                let isEating__313: bool = Some(i__309) == Some(eaterIndex__305);
                if isEating__313 {
                    t___2546 = temper_core::ListedTrait::len( & snake__310.segments());
                    t___1392 = t___2546;
                } else {
                    t___2548 = temper_core::ListedTrait::len( & snake__310.segments());
                    t___1392 = t___2548.wrapping_sub(1);
                }
                let keepLen__314: i32 = t___1392;
                let newSegs__315: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
                temper_core::listed::add( & newSegs__315, nh__311.clone(), None);
                let mut s__316: i32 = 0;
                'loop___2826: while Some(s__316) < Some(keepLen__314) {
                    t___2551 = snake__310.segments();
                    t___2552 = Point::new(0, 0);
                    temper_core::listed::add( & newSegs__315, temper_core::ListedTrait::get_or( & t___2551, s__316, t___2552.clone()), None);
                    s__316 = s__316.wrapping_add(1);
                }
                if isEating__313 {
                    t___2555 = snake__310.score();
                    t___1400 = t___2555.wrapping_add(1);
                } else {
                    t___2556 = snake__310.score();
                    t___1400 = t___2556;
                }
                let newScore__317: i32 = t___1400;
                temper_core::listed::add( & resultSnakes__308, PlayerSnake::new(snake__310.id(), temper_core::ListedTrait::to_list( & newSegs__315), dir__312.clone(), newScore__317, PlayerStatus::new(Alive::new())), None);
            }
        }
        i__309 = i__309.wrapping_add(1);
    }
    let resultSnakesList__318: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & resultSnakes__308);
    let mut t___2563: Point = game__277.food();
    let mut newFood__319: Point = t___2563.clone();
    let mut t___2564: i32 = game__277.rng_seed();
    let mut newSeed__320: i32 = t___2564;
    if Some(eaterIndex__305) >= Some(0) {
        let allSegs__321: temper_core::List<Point> = collectAllSegments__95(resultSnakesList__318.clone());
        t___2566 = game__277.width();
        t___2567 = game__277.height();
        t___2568 = game__277.rng_seed();
        let foodResult__322: FoodPlacement = placeFood__92(allSegs__321.clone(), t___2566, t___2567, t___2568);
        t___2570 = foodResult__322.point();
        newFood__319 = t___2570.clone();
        t___2571 = foodResult__322.seed();
        newSeed__320 = t___2571;
    }
    let mut t___2572: i32 = game__277.width();
    let mut t___2573: i32 = game__277.height();
    let mut t___2574: i32 = game__277.tick_count();
    return MultiSnakeGame::new(t___2572, t___2573, resultSnakesList__318.clone(), newFood__319.clone(), newSeed__320, t___2574.wrapping_add(1));
}
pub fn change_player_direction(game__323: MultiSnakeGame, playerId__324: i32, dir__325: Direction) -> MultiSnakeGame {
    let mut t___2393: i32;
    let mut t___2394: temper_core::List<PlayerSnake>;
    let mut t___2398: PlayerSnake;
    let mut t___2403: Direction;
    let mut t___2404: i32;
    let mut t___2405: temper_core::List<Point>;
    let mut t___2406: i32;
    let mut t___2407: PlayerStatus;
    let mut t___1245: bool;
    let mut t___1246: bool;
    let newSnakes__327: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__328: i32 = 0;
    'loop___2827: loop {
        t___2393 = temper_core::ListedTrait::len( & game__323.snakes());
        if ! (Some(i__328) < Some(t___2393)) {
            break;
        }
        t___2394 = game__323.snakes();
        t___2398 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__329: PlayerSnake = temper_core::ListedTrait::get_or( & t___2394, i__328, t___2398.clone());
        if Some(snake__329.id()) == Some(playerId__324) {
            if temper_core::is::<Alive>(snake__329.status()) {
                t___2403 = snake__329.direction();
                t___1245 = ! is_opposite(t___2403.clone(), dir__325.clone());
            } else {
                t___1245 = false;
            }
            t___1246 = t___1245;
        } else {
            t___1246 = false;
        }
        if t___1246 {
            t___2404 = snake__329.id();
            t___2405 = snake__329.segments();
            t___2406 = snake__329.score();
            t___2407 = snake__329.status();
            temper_core::listed::add( & newSnakes__327, PlayerSnake::new(t___2404, t___2405.clone(), dir__325.clone(), t___2406, t___2407.clone()), None);
        } else {
            temper_core::listed::add( & newSnakes__327, snake__329.clone(), None);
        }
        i__328 = i__328.wrapping_add(1);
    }
    return MultiSnakeGame::new(game__323.width(), game__323.height(), temper_core::ListedTrait::to_list( & newSnakes__327), game__323.food(), game__323.rng_seed(), game__323.tick_count());
}
pub fn is_multi_game_over(game__330: MultiSnakeGame) -> bool {
    let return__83: bool;
    let mut t___2378: i32;
    let mut t___2379: temper_core::List<PlayerSnake>;
    let mut t___2383: PlayerSnake;
    let mut aliveCount__332: i32 = 0;
    let mut i__333: i32 = 0;
    'loop___2828: loop {
        t___2378 = temper_core::ListedTrait::len( & game__330.snakes());
        if ! (Some(i__333) < Some(t___2378)) {
            break;
        }
        t___2379 = game__330.snakes();
        t___2383 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__334: PlayerSnake = temper_core::ListedTrait::get_or( & t___2379, i__333, t___2383.clone());
        if temper_core::is::<Alive>(snake__334.status()) {
            aliveCount__332 = aliveCount__332.wrapping_add(1);
        }
        i__333 = i__333.wrapping_add(1);
    }
    if Some(temper_core::ListedTrait::len( & game__330.snakes())) == Some(0) {
        return__83 = false;
    } else {
        if Some(temper_core::ListedTrait::len( & game__330.snakes())) == Some(1) {
            return__83 = Some(aliveCount__332) == Some(0);
        } else {
            return__83 = Some(aliveCount__332) <= Some(1);
        }
    }
    return return__83;
}
pub fn add_player(game__335: MultiSnakeGame, seed__336: i32) -> MultiSnakeGame {
    let mut t___2356: i32;
    let mut t___2357: temper_core::List<PlayerSnake>;
    let mut t___2361: PlayerSnake;
    let newId__338: i32 = temper_core::ListedTrait::len( & game__335.snakes());
    let spawn__339: SpawnInfo = spawnPosition__94(game__335.width(), game__335.height(), newId__338, newId__338.wrapping_add(1));
    let dir__340: Direction = spawn__339.direction();
    let delta__341: Point = direction_delta(dir__340.clone());
    let startX__342: i32 = spawn__339.point().x();
    let startY__343: i32 = spawn__339.point().y();
    let segments__344: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(startX__342, startY__343), Point::new(startX__342.wrapping_sub(delta__341.x()), startY__343.wrapping_sub(delta__341.y())), Point::new(startX__342.wrapping_sub(delta__341.x().wrapping_mul(2)), startY__343.wrapping_sub(delta__341.y().wrapping_mul(2)))]);
    let newSnake__345: PlayerSnake = PlayerSnake::new(newId__338, segments__344.clone(), dir__340.clone(), 0, PlayerStatus::new(Alive::new()));
    let builder__346: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__347: i32 = 0;
    'loop___2829: loop {
        t___2356 = temper_core::ListedTrait::len( & game__335.snakes());
        if ! (Some(i__347) < Some(t___2356)) {
            break;
        }
        t___2357 = game__335.snakes();
        t___2361 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        temper_core::listed::add( & builder__346, temper_core::ListedTrait::get_or( & t___2357, i__347, t___2361.clone()), None);
        i__347 = i__347.wrapping_add(1);
    }
    temper_core::listed::add( & builder__346, newSnake__345.clone(), None);
    let mut t___2365: temper_core::List<PlayerSnake> = temper_core::ListedTrait::to_list( & builder__346);
    let allSegs__348: temper_core::List<Point> = collectAllSegments__95(t___2365.clone());
    let mut t___2367: i32 = game__335.width();
    let mut t___2368: i32 = game__335.height();
    let foodResult__349: FoodPlacement = placeFood__92(allSegs__348.clone(), t___2367, t___2368, seed__336);
    return MultiSnakeGame::new(game__335.width(), game__335.height(), temper_core::ListedTrait::to_list( & builder__346), foodResult__349.point(), foodResult__349.seed(), game__335.tick_count());
}
pub fn remove_player(game__350: MultiSnakeGame, playerId__351: i32) -> MultiSnakeGame {
    let mut t___2318: i32;
    let mut t___2319: temper_core::List<PlayerSnake>;
    let mut t___2323: PlayerSnake;
    let builder__353: temper_core::ListBuilder<PlayerSnake> = temper_core::listed::new_builder();
    let mut i__354: i32 = 0;
    'loop___2830: loop {
        t___2318 = temper_core::ListedTrait::len( & game__350.snakes());
        if ! (Some(i__354) < Some(t___2318)) {
            break;
        }
        t___2319 = game__350.snakes();
        t___2323 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__355: PlayerSnake = temper_core::ListedTrait::get_or( & t___2319, i__354, t___2323.clone());
        if Some(snake__355.id()) != Some(playerId__351) {
            temper_core::listed::add( & builder__353, snake__355.clone(), None);
        }
        i__354 = i__354.wrapping_add(1);
    }
    return MultiSnakeGame::new(game__350.width(), game__350.height(), temper_core::ListedTrait::to_list( & builder__353), game__350.food(), game__350.rng_seed(), game__350.tick_count());
}
pub fn player_head_char(id__368: i32) -> std::sync::Arc<String> {
    let return__87: std::sync::Arc<String>;
    if Some(id__368) == Some(0) {
        return__87 = std::sync::Arc::new("@".to_string());
    } else {
        if Some(id__368) == Some(1) {
            return__87 = std::sync::Arc::new("#".to_string());
        } else {
            if Some(id__368) == Some(2) {
                return__87 = std::sync::Arc::new("$".to_string());
            } else {
                if Some(id__368) == Some(3) {
                    return__87 = std::sync::Arc::new("%".to_string());
                } else {
                    return__87 = std::sync::Arc::new("&".to_string());
                }
            }
        }
    }
    return return__87.clone();
}
pub fn player_body_char(id__370: i32) -> std::sync::Arc<String> {
    let return__88: std::sync::Arc<String>;
    if Some(id__370) == Some(0) {
        return__88 = std::sync::Arc::new("o".to_string());
    } else {
        if Some(id__370) == Some(1) {
            return__88 = std::sync::Arc::new("+".to_string());
        } else {
            if Some(id__370) == Some(2) {
                return__88 = std::sync::Arc::new("~".to_string());
            } else {
                if Some(id__370) == Some(3) {
                    return__88 = std::sync::Arc::new("=".to_string());
                } else {
                    return__88 = std::sync::Arc::new(".".to_string());
                }
            }
        }
    }
    return return__88.clone();
}
fn multiCellChar__96(game__372: MultiSnakeGame, p__373: Point) -> std::sync::Arc<String> {
    let return__89: std::sync::Arc<String>;
    let mut t___2288: i32;
    let mut t___2289: temper_core::List<PlayerSnake>;
    let mut t___2293: PlayerSnake;
    let mut t___2300: i32;
    let mut t___2302: i32;
    let mut t___2303: temper_core::List<PlayerSnake>;
    let mut t___2307: PlayerSnake;
    let mut t___2310: i32;
    let mut t___2311: temper_core::List<Point>;
    let mut t___2312: Point;
    let mut t___2313: Point;
    let mut t___2314: i32;
    let mut t___2315: Point;
    'fn__374: {
        let mut i__375: i32 = 0;
        'loop___2831: loop {
            t___2288 = temper_core::ListedTrait::len( & game__372.snakes());
            if ! (Some(i__375) < Some(t___2288)) {
                break;
            }
            t___2289 = game__372.snakes();
            t___2293 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
            let snake__376: PlayerSnake = temper_core::ListedTrait::get_or( & t___2289, i__375, t___2293.clone());
            if Some(temper_core::ListedTrait::len( & snake__376.segments())) > Some(0) {
                let head__377: Point = temper_core::ListedTrait::get_or( & snake__376.segments(), 0, Point::new(-1, -1));
                if point_equals(p__373.clone(), head__377.clone()) {
                    t___2300 = snake__376.id();
                    return__89 = player_head_char(t___2300);
                    break 'fn__374;
                }
            }
            i__375 = i__375.wrapping_add(1);
        }
        let mut i__378: i32 = 0;
        'loop___2832: loop {
            t___2302 = temper_core::ListedTrait::len( & game__372.snakes());
            if ! (Some(i__378) < Some(t___2302)) {
                break;
            }
            t___2303 = game__372.snakes();
            t___2307 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
            let snake__379: PlayerSnake = temper_core::ListedTrait::get_or( & t___2303, i__378, t___2307.clone());
            let mut j__380: i32 = 1;
            'loop___2833: loop {
                t___2310 = temper_core::ListedTrait::len( & snake__379.segments());
                if ! (Some(j__380) < Some(t___2310)) {
                    break;
                }
                t___2311 = snake__379.segments();
                t___2312 = Point::new(-1, -1);
                t___2313 = temper_core::ListedTrait::get_or( & t___2311, j__380, t___2312.clone());
                if point_equals(p__373.clone(), t___2313.clone()) {
                    t___2314 = snake__379.id();
                    return__89 = player_body_char(t___2314);
                    break 'fn__374;
                }
                j__380 = j__380.wrapping_add(1);
            }
            i__378 = i__378.wrapping_add(1);
        }
        t___2315 = game__372.food();
        if point_equals(p__373.clone(), t___2315.clone()) {
            return__89 = std::sync::Arc::new("*".to_string());
            break 'fn__374;
        }
        return__89 = std::sync::Arc::new(" ".to_string());
    }
    return return__89.clone();
}
pub fn multi_render(game__356: MultiSnakeGame) -> std::sync::Arc<String> {
    let mut t___2255: i32;
    let mut t___2258: i32;
    let mut t___2260: i32;
    let mut t___2266: i32;
    let mut t___2270: i32;
    let mut t___2271: temper_core::List<PlayerSnake>;
    let mut t___2275: PlayerSnake;
    let mut t___2277: PlayerStatus;
    let mut t___1116: std::sync::Arc<String>;
    let sb__358: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    temper_core::string::builder::append( & sb__358, "\x1b[2J\x1b[H");
    temper_core::string::builder::append( & sb__358, "#");
    let mut x__359: i32 = 0;
    'loop___2834: loop {
        t___2255 = game__356.width();
        if ! (Some(x__359) < Some(t___2255)) {
            break;
        }
        temper_core::string::builder::append( & sb__358, "#");
        x__359 = x__359.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__358, "#\x0d\x0a");
    let mut y__360: i32 = 0;
    'loop___2835: loop {
        t___2258 = game__356.height();
        if ! (Some(y__360) < Some(t___2258)) {
            break;
        }
        temper_core::string::builder::append( & sb__358, "#");
        let mut x__361: i32 = 0;
        'loop___2836: loop {
            t___2260 = game__356.width();
            if ! (Some(x__361) < Some(t___2260)) {
                break;
            }
            let p__362: Point = Point::new(x__361, y__360);
            temper_core::string::builder::append( & sb__358, multiCellChar__96(game__356.clone(), p__362.clone()));
            x__361 = x__361.wrapping_add(1);
        }
        temper_core::string::builder::append( & sb__358, "#\x0d\x0a");
        y__360 = y__360.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__358, "#");
    let mut x__363: i32 = 0;
    'loop___2837: loop {
        t___2266 = game__356.width();
        if ! (Some(x__363) < Some(t___2266)) {
            break;
        }
        temper_core::string::builder::append( & sb__358, "#");
        x__363 = x__363.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__358, "#\x0d\x0a");
    let mut i__364: i32 = 0;
    'loop___2838: loop {
        t___2270 = temper_core::ListedTrait::len( & game__356.snakes());
        if ! (Some(i__364) < Some(t___2270)) {
            break;
        }
        t___2271 = game__356.snakes();
        t___2275 = PlayerSnake::new(0, [], Direction::new(Right::new()), 0, PlayerStatus::new(Dead::new()));
        let snake__365: PlayerSnake = temper_core::ListedTrait::get_or( & t___2271, i__364, t___2275.clone());
        t___2277 = snake__365.status();
        if temper_core::is::<Alive>(t___2277.clone()) {
            t___1116 = std::sync::Arc::new("Playing".to_string());
        } else {
            if temper_core::is::<Dead>(t___2277.clone()) {
                t___1116 = std::sync::Arc::new("DEAD".to_string());
            } else {
                t___1116 = std::sync::Arc::new("".to_string());
            }
        }
        let statusText__366: std::sync::Arc<String> = t___1116.clone();
        let symbol__367: std::sync::Arc<String> = player_head_char(snake__365.id());
        temper_core::string::builder::append( & sb__358, std::sync::Arc::new(format!("P{} {}: {}  {}\x0d\x0a", snake__365.id(), symbol__367.clone(), snake__365.score(), statusText__366.clone())));
        i__364 = i__364.wrapping_add(1);
    }
    return temper_core::string::builder::to_string( & sb__358);
}
pub fn direction_to_string(dir__381: Direction) -> std::sync::Arc<String> {
    let return__90: std::sync::Arc<String>;
    if temper_core::is::<Up>(dir__381.clone()) {
        return__90 = std::sync::Arc::new("up".to_string());
    } else {
        if temper_core::is::<Down>(dir__381.clone()) {
            return__90 = std::sync::Arc::new("down".to_string());
        } else {
            if temper_core::is::<Left>(dir__381.clone()) {
                return__90 = std::sync::Arc::new("left".to_string());
            } else {
                if temper_core::is::<Right>(dir__381.clone()) {
                    return__90 = std::sync::Arc::new("right".to_string());
                } else {
                    return__90 = std::sync::Arc::new("right".to_string());
                }
            }
        }
    }
    return return__90.clone();
}
pub fn string_to_direction(s__383: impl temper_core::ToArcString) -> Option<Direction> {
    let s__383 = s__383.to_arc_string();
    let return__91: Option<Direction>;
    'fn__384: {
        if Some(s__383.as_str()) == Some("up") {
            return__91 = Some(Direction::new(Up::new()));
            break 'fn__384;
        }
        if Some(s__383.as_str()) == Some("down") {
            return__91 = Some(Direction::new(Down::new()));
            break 'fn__384;
        }
        if Some(s__383.as_str()) == Some("left") {
            return__91 = Some(Direction::new(Left::new()));
            break 'fn__384;
        }
        if Some(s__383.as_str()) == Some("right") {
            return__91 = Some(Direction::new(Right::new()));
            break 'fn__384;
        }
        return__91 = None;
    }
    return return__91.clone();
}
