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
    pub fn new(x__57: i32, y__58: i32) -> Point {
        let x;
        let y;
        x = x__57;
        y = y__58;
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
    pub fn new(value__72: i32, nextSeed__73: i32) -> RandomResult {
        let value;
        let next_seed;
        value = value__72;
        next_seed = nextSeed__73;
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
    pub fn new(width__90: i32, height__91: i32, snake__92: impl temper_core::ToList<Point>, direction__93: Direction, food__94: Point, score__95: i32, status__96: GameStatus, rngSeed__97: i32) -> SnakeGame {
        let snake__92 = snake__92.to_list();
        let width;
        let height;
        let snake;
        let direction;
        let food;
        let score;
        let status;
        let rng_seed;
        width = width__90;
        height = height__91;
        snake = snake__92.clone();
        direction = direction__93.clone();
        food = food__94.clone();
        score = score__95;
        status = status__96.clone();
        rng_seed = rngSeed__97;
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
    pub fn new(point__101: Point, seed__102: i32) -> FoodPlacement {
        let point;
        let seed;
        point = point__101.clone();
        seed = seed__102;
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
pub fn r#move(head__44: Point, body__45: impl temper_core::ToList<Point>, food__46: Point, width__47: i32, height__48: i32) -> Direction {
    let body__45 = body__45.to_list();
    return Direction::new(Right::new());
}
pub fn point_equals(a__61: Point, b__62: Point) -> bool {
    let return__25: bool;
    let mut t___944: i32;
    let mut t___945: i32;
    if Some(a__61.x()) == Some(b__62.x()) {
        t___944 = a__61.y();
        t___945 = b__62.y();
        return__25 = Some(t___944) == Some(t___945);
    } else {
        return__25 = false;
    }
    return return__25;
}
pub fn is_opposite(a__64: Direction, b__65: Direction) -> bool {
    let return__26: bool;
    if temper_core::is::<Up>(a__64.clone()) {
        return__26 = temper_core::is::<Down>(b__65.clone());
    } else {
        if temper_core::is::<Down>(a__64.clone()) {
            return__26 = temper_core::is::<Up>(b__65.clone());
        } else {
            if temper_core::is::<Left>(a__64.clone()) {
                return__26 = temper_core::is::<Right>(b__65.clone());
            } else {
                if temper_core::is::<Right>(a__64.clone()) {
                    return__26 = temper_core::is::<Left>(b__65.clone());
                } else {
                    return__26 = false;
                }
            }
        }
    }
    return return__26;
}
pub fn direction_delta(dir__67: Direction) -> Point {
    let return__27: Point;
    if temper_core::is::<Up>(dir__67.clone()) {
        return__27 = Point::new(0, -1);
    } else {
        if temper_core::is::<Down>(dir__67.clone()) {
            return__27 = Point::new(0, 1);
        } else {
            if temper_core::is::<Left>(dir__67.clone()) {
                return__27 = Point::new(-1, 0);
            } else {
                if temper_core::is::<Right>(dir__67.clone()) {
                    return__27 = Point::new(1, 0);
                } else {
                    return__27 = Point::new(0, 0);
                }
            }
        }
    }
    return return__27.clone();
}
pub fn next_random(seed__74: i32, max__75: i32) -> RandomResult {
    let mut t___587: i32;
    let mut t___589: i32;
    let raw__77: i32 = seed__74.wrapping_mul(1664525).wrapping_add(1013904223);
    let masked__78: i32 = raw__77 & 2147483647;
    let posVal__79: i32;
    if Some(masked__78) < Some(0) {
        posVal__79 = masked__78.wrapping_neg();
    } else {
        posVal__79 = masked__78;
    }
    let mut value__80: i32 = 0;
    if Some(max__75) > Some(0) {
        'ok___947: {
            'orelse___230: {
                t___587 = match temper_core::int_rem(posVal__79, max__75) {
                    Ok(x) => x,
                    _ => break 'orelse___230
                };
                t___589 = t___587;
                break 'ok___947;
            }
            t___589 = 0;
        }
        value__80 = t___589;
    }
    return RandomResult::new(value__80, masked__78);
}
fn placeFood__42(snake__103: impl temper_core::ToList<Point>, width__104: i32, height__105: i32, seed__106: i32) -> FoodPlacement {
    let snake__103 = snake__103.to_list();
    let return__36: FoodPlacement;
    let mut t___911: i32;
    let mut t___922: Point;
    let mut t___552: i32;
    let mut t___554: i32;
    let mut t___556: i32;
    let mut t___558: i32;
    'fn__107: {
        let totalCells__108: i32 = width__104.wrapping_mul(height__105);
        let mut currentSeed__109: i32 = seed__106;
        let mut attempt__110: i32 = 0;
        'loop___960: while Some(attempt__110) < Some(totalCells__108) {
            let result__111: RandomResult = next_random(currentSeed__109, totalCells__108);
            t___911 = result__111.next_seed();
            currentSeed__109 = t___911;
            let mut px__112: i32 = 0;
            let mut py__113: i32 = 0;
            if Some(width__104) > Some(0) {
                'ok___948: {
                    'orelse___231: {
                        t___552 = match temper_core::int_rem(result__111.value(), width__104) {
                            Ok(x) => x,
                            _ => break 'orelse___231
                        };
                        t___554 = t___552;
                        break 'ok___948;
                    }
                    t___554 = 0;
                }
                px__112 = t___554;
                'ok___949: {
                    'orelse___232: {
                        t___556 = match temper_core::int_div(result__111.value(), width__104) {
                            Ok(x) => x,
                            _ => break 'orelse___232
                        };
                        t___558 = t___556;
                        break 'ok___949;
                    }
                    t___558 = 0;
                }
                py__113 = t___558;
            }
            let candidate__114: Point = Point::new(px__112, py__113);
            let mut occupied__115: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___1 {
                candidate__114: Point, occupied__115: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___1 {
                fn fn__910(& self, seg__116: Point) {
                    if point_equals(seg__116.clone(), self.candidate__114.clone()) {
                        {
                            * self.occupied__115.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___1 {
                candidate__114: candidate__114.clone(), occupied__115: occupied__115.clone()
            };
            let fn__910 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | seg__116: Point | closure_group.fn__910(seg__116))
            };
            temper_core::listed::list_for_each( & snake__103, & ( * fn__910.clone()));
            if ! temper_core::read_locked( & occupied__115) {
                return__36 = FoodPlacement::new(candidate__114.clone(), currentSeed__109);
                break 'fn__107;
            }
            attempt__110 = attempt__110.wrapping_add(1);
        }
        let mut y__117: i32 = 0;
        'loop___961: while Some(y__117) < Some(height__105) {
            let mut x__118: i32 = 0;
            'loop___962: while Some(x__118) < Some(width__104) {
                let candidate__119: Point = Point::new(x__118, y__117);
                let mut free__120: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
                #[derive(Clone)]
                struct ClosureGroup___2 {
                    candidate__119: Point, free__120: std::sync::Arc<std::sync::RwLock<bool>>
                }
                impl ClosureGroup___2 {
                    fn fn__909(& self, seg__121: Point) {
                        if point_equals(seg__121.clone(), self.candidate__119.clone()) {
                            {
                                * self.free__120.write().unwrap() = false;
                            }
                        }
                    }
                }
                let closure_group = ClosureGroup___2 {
                    candidate__119: candidate__119.clone(), free__120: free__120.clone()
                };
                let fn__909 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | seg__121: Point | closure_group.fn__909(seg__121))
                };
                temper_core::listed::list_for_each( & snake__103, & ( * fn__909.clone()));
                if temper_core::read_locked( & free__120) {
                    return__36 = FoodPlacement::new(candidate__119.clone(), currentSeed__109);
                    break 'fn__107;
                }
                x__118 = x__118.wrapping_add(1);
            }
            y__117 = y__117.wrapping_add(1);
        }
        t___922 = Point::new(0, 0);
        return__36 = FoodPlacement::new(t___922.clone(), currentSeed__109);
    }
    return return__36.clone();
}
pub fn new_game(width__122: i32, height__123: i32, seed__124: i32) -> SnakeGame {
    let mut t___535: i32;
    let mut t___537: i32;
    let mut t___538: i32;
    let mut t___540: i32;
    let mut centerX__126: i32 = 0;
    let mut centerY__127: i32 = 0;
    if Some(width__122) > Some(0) {
        t___535 = width__122.wrapping_div(2);
        t___537 = t___535;
        centerX__126 = t___537;
    }
    if Some(height__123) > Some(0) {
        t___538 = height__123.wrapping_div(2);
        t___540 = t___538;
        centerY__127 = t___540;
    }
    let snake__128: temper_core::List<Point> = std::sync::Arc::new(vec![Point::new(centerX__126, centerY__127), Point::new(centerX__126.wrapping_sub(1), centerY__127), Point::new(centerX__126.wrapping_sub(2), centerY__127)]);
    let foodResult__129: FoodPlacement = placeFood__42(snake__128.clone(), width__122, height__123, seed__124);
    let mut t___904: Right = Right::new();
    let mut t___905: Point = foodResult__129.point();
    let mut t___906: Playing = Playing::new();
    let mut t___907: i32 = foodResult__129.seed();
    return SnakeGame::new(width__122, height__123, snake__128.clone(), Direction::new(t___904.clone()), t___905.clone(), 0, GameStatus::new(t___906.clone()), t___907);
}
pub fn change_direction(game__130: SnakeGame, dir__131: Direction) -> SnakeGame {
    let return__38: SnakeGame;
    let mut t___892: i32;
    let mut t___893: i32;
    let mut t___894: temper_core::List<Point>;
    let mut t___895: Point;
    let mut t___896: i32;
    let mut t___897: GameStatus;
    let mut t___898: i32;
    if is_opposite(game__130.direction(), dir__131.clone()) {
        return__38 = game__130.clone();
    } else {
        t___892 = game__130.width();
        t___893 = game__130.height();
        t___894 = game__130.snake();
        t___895 = game__130.food();
        t___896 = game__130.score();
        t___897 = game__130.status();
        t___898 = game__130.rng_seed();
        return__38 = SnakeGame::new(t___892, t___893, t___894.clone(), dir__131.clone(), t___895.clone(), t___896, t___897.clone(), t___898);
    }
    return return__38.clone();
}
pub fn tick(game__133: SnakeGame) -> SnakeGame {
    let return__39: SnakeGame;
    let mut t___832: i32;
    let mut t___833: i32;
    let mut t___834: i32;
    let mut t___835: i32;
    let mut t___836: temper_core::List<Point>;
    let mut t___837: Direction;
    let mut t___838: Point;
    let mut t___839: i32;
    let mut t___840: GameOver;
    let mut t___841: i32;
    let mut t___845: i32;
    let mut t___847: i32;
    let mut t___848: temper_core::List<Point>;
    let mut t___849: Point;
    let mut t___851: i32;
    let mut t___852: i32;
    let mut t___853: temper_core::List<Point>;
    let mut t___854: Direction;
    let mut t___855: Point;
    let mut t___856: i32;
    let mut t___857: GameOver;
    let mut t___858: i32;
    let mut t___863: i32;
    let mut t___865: i32;
    let mut t___866: temper_core::List<Point>;
    let mut t___867: Point;
    let mut t___872: i32;
    let mut t___873: i32;
    let mut t___874: i32;
    let mut t___876: i32;
    let mut t___877: i32;
    let mut t___878: Direction;
    let mut t___879: Point;
    let mut t___880: Playing;
    let mut t___881: i32;
    let mut t___883: i32;
    let mut t___884: i32;
    let mut t___885: Direction;
    let mut t___886: Point;
    let mut t___887: i32;
    let mut t___888: GameStatus;
    let mut t___889: i32;
    let mut t___462: bool;
    let mut t___463: bool;
    let mut t___464: bool;
    'fn__134: {
        if temper_core::is::<GameOver>(game__133.status()) {
            return__39 = game__133.clone();
            break 'fn__134;
        }
        let delta__135: Point = direction_delta(game__133.direction());
        let head__136: Point = temper_core::ListedTrait::get_or( & game__133.snake(), 0, Point::new(0, 0));
        let newHead__137: Point = Point::new(head__136.x().wrapping_add(delta__135.x()), head__136.y().wrapping_add(delta__135.y()));
        if Some(newHead__137.x()) < Some(0) {
            t___464 = true;
        } else {
            if Some(newHead__137.x()) >= Some(game__133.width()) {
                t___463 = true;
            } else {
                if Some(newHead__137.y()) < Some(0) {
                    t___462 = true;
                } else {
                    t___832 = newHead__137.y();
                    t___833 = game__133.height();
                    t___462 = Some(t___832) >= Some(t___833);
                }
                t___463 = t___462;
            }
            t___464 = t___463;
        }
        if t___464 {
            t___834 = game__133.width();
            t___835 = game__133.height();
            t___836 = game__133.snake();
            t___837 = game__133.direction();
            t___838 = game__133.food();
            t___839 = game__133.score();
            t___840 = GameOver::new();
            t___841 = game__133.rng_seed();
            return__39 = SnakeGame::new(t___834, t___835, t___836.clone(), t___837.clone(), t___838.clone(), t___839, GameStatus::new(t___840.clone()), t___841);
            break 'fn__134;
        }
        let eating__138: bool = point_equals(newHead__137.clone(), game__133.food());
        let checkLength__139: i32;
        if eating__138 {
            t___845 = temper_core::ListedTrait::len( & game__133.snake());
            checkLength__139 = t___845;
        } else {
            t___847 = temper_core::ListedTrait::len( & game__133.snake());
            checkLength__139 = t___847.wrapping_sub(1);
        }
        let mut i__140: i32 = 0;
        'loop___963: while Some(i__140) < Some(checkLength__139) {
            t___848 = game__133.snake();
            t___849 = Point::new(-1, -1);
            if point_equals(newHead__137.clone(), temper_core::ListedTrait::get_or( & t___848, i__140, t___849.clone())) {
                t___851 = game__133.width();
                t___852 = game__133.height();
                t___853 = game__133.snake();
                t___854 = game__133.direction();
                t___855 = game__133.food();
                t___856 = game__133.score();
                t___857 = GameOver::new();
                t___858 = game__133.rng_seed();
                return__39 = SnakeGame::new(t___851, t___852, t___853.clone(), t___854.clone(), t___855.clone(), t___856, GameStatus::new(t___857.clone()), t___858);
                break 'fn__134;
            }
            i__140 = i__140.wrapping_add(1);
        }
        let newSnakeBuilder__141: temper_core::ListBuilder<Point> = temper_core::listed::new_builder();
        temper_core::listed::add( & newSnakeBuilder__141, newHead__137.clone(), None);
        let keepLength__142: i32;
        if eating__138 {
            t___863 = temper_core::ListedTrait::len( & game__133.snake());
            keepLength__142 = t___863;
        } else {
            t___865 = temper_core::ListedTrait::len( & game__133.snake());
            keepLength__142 = t___865.wrapping_sub(1);
        }
        let mut i__143: i32 = 0;
        'loop___964: while Some(i__143) < Some(keepLength__142) {
            t___866 = game__133.snake();
            t___867 = Point::new(0, 0);
            temper_core::listed::add( & newSnakeBuilder__141, temper_core::ListedTrait::get_or( & t___866, i__143, t___867.clone()), None);
            i__143 = i__143.wrapping_add(1);
        }
        let newSnake__144: temper_core::List<Point> = temper_core::ListedTrait::to_list( & newSnakeBuilder__141);
        if eating__138 {
            let newScore__145: i32 = game__133.score().wrapping_add(1);
            t___872 = game__133.width();
            t___873 = game__133.height();
            t___874 = game__133.rng_seed();
            let foodResult__146: FoodPlacement = placeFood__42(newSnake__144.clone(), t___872, t___873, t___874);
            t___876 = game__133.width();
            t___877 = game__133.height();
            t___878 = game__133.direction();
            t___879 = foodResult__146.point();
            t___880 = Playing::new();
            t___881 = foodResult__146.seed();
            return__39 = SnakeGame::new(t___876, t___877, newSnake__144.clone(), t___878.clone(), t___879.clone(), newScore__145, GameStatus::new(t___880.clone()), t___881);
        } else {
            t___883 = game__133.width();
            t___884 = game__133.height();
            t___885 = game__133.direction();
            t___886 = game__133.food();
            t___887 = game__133.score();
            t___888 = game__133.status();
            t___889 = game__133.rng_seed();
            return__39 = SnakeGame::new(t___883, t___884, newSnake__144.clone(), t___885.clone(), t___886.clone(), t___887, t___888.clone(), t___889);
        }
    }
    return return__39.clone();
}
fn cellChar__43(game__156: SnakeGame, p__157: Point) -> std::sync::Arc<String> {
    let return__41: std::sync::Arc<String>;
    let mut t___811: i32;
    let mut t___812: temper_core::List<Point>;
    let mut t___813: Point;
    let mut t___814: Point;
    let mut t___815: Point;
    'fn__158: {
        let head__159: Point = temper_core::ListedTrait::get_or( & game__156.snake(), 0, Point::new(-1, -1));
        if point_equals(p__157.clone(), head__159.clone()) {
            return__41 = std::sync::Arc::new("@".to_string());
            break 'fn__158;
        }
        let mut i__160: i32 = 1;
        'loop___965: loop {
            t___811 = temper_core::ListedTrait::len( & game__156.snake());
            if ! (Some(i__160) < Some(t___811)) {
                break;
            }
            t___812 = game__156.snake();
            t___813 = Point::new(-1, -1);
            t___814 = temper_core::ListedTrait::get_or( & t___812, i__160, t___813.clone());
            if point_equals(p__157.clone(), t___814.clone()) {
                return__41 = std::sync::Arc::new("o".to_string());
                break 'fn__158;
            }
            i__160 = i__160.wrapping_add(1);
        }
        t___815 = game__156.food();
        if point_equals(p__157.clone(), t___815.clone()) {
            return__41 = std::sync::Arc::new("*".to_string());
            break 'fn__158;
        }
        return__41 = std::sync::Arc::new(" ".to_string());
    }
    return return__41.clone();
}
pub fn render(game__147: SnakeGame) -> std::sync::Arc<String> {
    let mut t___786: i32;
    let mut t___789: i32;
    let mut t___791: i32;
    let mut t___797: i32;
    let sb__149: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    temper_core::string::builder::append( & sb__149, "\x1b[2J\x1b[H");
    temper_core::string::builder::append( & sb__149, "#");
    let mut x__150: i32 = 0;
    'loop___966: loop {
        t___786 = game__147.width();
        if ! (Some(x__150) < Some(t___786)) {
            break;
        }
        temper_core::string::builder::append( & sb__149, "#");
        x__150 = x__150.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__149, "#\x0a");
    let mut y__151: i32 = 0;
    'loop___967: loop {
        t___789 = game__147.height();
        if ! (Some(y__151) < Some(t___789)) {
            break;
        }
        temper_core::string::builder::append( & sb__149, "#");
        let mut x__152: i32 = 0;
        'loop___968: loop {
            t___791 = game__147.width();
            if ! (Some(x__152) < Some(t___791)) {
                break;
            }
            let p__153: Point = Point::new(x__152, y__151);
            temper_core::string::builder::append( & sb__149, cellChar__43(game__147.clone(), p__153.clone()));
            x__152 = x__152.wrapping_add(1);
        }
        temper_core::string::builder::append( & sb__149, "#\x0a");
        y__151 = y__151.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__149, "#");
    let mut x__154: i32 = 0;
    'loop___969: loop {
        t___797 = game__147.width();
        if ! (Some(x__154) < Some(t___797)) {
            break;
        }
        temper_core::string::builder::append( & sb__149, "#");
        x__154 = x__154.wrapping_add(1);
    }
    temper_core::string::builder::append( & sb__149, "#\x0a");
    let statusText__155: std::sync::Arc<String>;
    let mut t___800: GameStatus = game__147.status();
    if temper_core::is::<Playing>(t___800.clone()) {
        statusText__155 = std::sync::Arc::new("Playing".to_string());
    } else {
        if temper_core::is::<GameOver>(t___800.clone()) {
            statusText__155 = std::sync::Arc::new("GAME OVER".to_string());
        } else {
            statusText__155 = std::sync::Arc::new("".to_string());
        }
    }
    temper_core::string::builder::append( & sb__149, std::sync::Arc::new(format!("Score: {}  {}\x0a", game__147.score(), statusText__155.clone())));
    return temper_core::string::builder::to_string( & sb__149);
}
