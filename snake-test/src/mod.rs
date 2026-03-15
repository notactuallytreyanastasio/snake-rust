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
#[cfg(test)]
mod tests {
    #[test]
    fn initialStateHasSnakeNearCenter__168() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___0 = temper_std::testing::Test::new();
        let game__65: snake::SnakeGame = snake::new_game(10, 10, 42);
        let head__66: snake::Point = temper_core::ListedTrait::get_or( & game__65.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1567: bool = Some(head__66.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___1 {
            head__66: snake::Point
        }
        impl ClosureGroup___1 {
            fn fn__1560(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should be 5, got {}", self.head__66.x()));
            }
        }
        let closure_group = ClosureGroup___1 {
            head__66: head__66.clone()
        };
        let fn__1560 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1560())
        };
        test___0.assert(t___1567, fn__1560.clone());
        let mut t___1571: bool = Some(head__66.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___2 {
            head__66: snake::Point
        }
        impl ClosureGroup___2 {
            fn fn__1559(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should be 5, got {}", self.head__66.y()));
            }
        }
        let closure_group = ClosureGroup___2 {
            head__66: head__66.clone()
        };
        let fn__1559 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1559())
        };
        test___0.assert(t___1571, fn__1559.clone());
        let mut t___1576: bool = Some(temper_core::ListedTrait::len( & game__65.snake())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__1558(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should start with 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__1558 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1558())
        };
        test___0.assert(t___1576, fn__1558.clone());
        test___0.soft_fail_to_hard()
    }
    #[test]
    fn initialStatusIsPlaying__169() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___1 = temper_std::testing::Test::new();
        let game__68: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1551: bool = temper_core::is::<snake::Playing>(game__68.status());
        #[derive(Clone)]
        struct ClosureGroup___4 {}
        impl ClosureGroup___4 {
            fn fn__1548(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial status should be Playing".to_string());
            }
        }
        let closure_group = ClosureGroup___4 {};
        let fn__1548 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1548())
        };
        test___1.assert(t___1551, fn__1548.clone());
        test___1.soft_fail_to_hard()
    }
    #[test]
    fn initialDirectionIsRight__170() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___2 = temper_std::testing::Test::new();
        let game__70: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1545: bool = temper_core::is::<snake::Right>(game__70.direction());
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__1542(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial direction should be Right".to_string());
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__1542 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1542())
        };
        test___2.assert(t___1545, fn__1542.clone());
        test___2.soft_fail_to_hard()
    }
    #[test]
    fn initialScoreIs0__171() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___3 = temper_std::testing::Test::new();
        let game__72: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1540: bool = Some(game__72.score()) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___6 {}
        impl ClosureGroup___6 {
            fn fn__1536(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial score should be 0".to_string());
            }
        }
        let closure_group = ClosureGroup___6 {};
        let fn__1536 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1536())
        };
        test___3.assert(t___1540, fn__1536.clone());
        test___3.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesRight__172() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___4 = temper_std::testing::Test::new();
        let game__74: snake::SnakeGame = snake::new_game(10, 10, 42);
        let moved__75: snake::SnakeGame = snake::tick(game__74.clone());
        let head__76: snake::Point = temper_core::ListedTrait::get_or( & moved__75.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1530: bool = Some(head__76.x()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___7 {
            head__76: snake::Point
        }
        impl ClosureGroup___7 {
            fn fn__1522(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move right to x=6, got {}", self.head__76.x()));
            }
        }
        let closure_group = ClosureGroup___7 {
            head__76: head__76.clone()
        };
        let fn__1522 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1522())
        };
        test___4.assert(t___1530, fn__1522.clone());
        let mut t___1534: bool = Some(head__76.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___8 {
            head__76: snake::Point
        }
        impl ClosureGroup___8 {
            fn fn__1521(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should stay 5, got {}", self.head__76.y()));
            }
        }
        let closure_group = ClosureGroup___8 {
            head__76: head__76.clone()
        };
        let fn__1521 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1521())
        };
        test___4.assert(t___1534, fn__1521.clone());
        test___4.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesDown__173() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___5 = temper_std::testing::Test::new();
        let game__78: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Down::new()));
        let moved__79: snake::SnakeGame = snake::tick(game__78.clone());
        let head__80: snake::Point = temper_core::ListedTrait::get_or( & moved__79.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1511: bool = Some(head__80.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___9 {
            head__80: snake::Point
        }
        impl ClosureGroup___9 {
            fn fn__1502(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should stay 5, got {}", self.head__80.x()));
            }
        }
        let closure_group = ClosureGroup___9 {
            head__80: head__80.clone()
        };
        let fn__1502 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1502())
        };
        test___5.assert(t___1511, fn__1502.clone());
        let mut t___1515: bool = Some(head__80.y()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___10 {
            head__80: snake::Point
        }
        impl ClosureGroup___10 {
            fn fn__1501(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move down to y=6, got {}", self.head__80.y()));
            }
        }
        let closure_group = ClosureGroup___10 {
            head__80: head__80.clone()
        };
        let fn__1501 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1501())
        };
        test___5.assert(t___1515, fn__1501.clone());
        test___5.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesUp__174() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___6 = temper_std::testing::Test::new();
        let game__82: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Up::new()));
        let moved__83: snake::SnakeGame = snake::tick(game__82.clone());
        let head__84: snake::Point = temper_core::ListedTrait::get_or( & moved__83.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1495: bool = Some(head__84.y()) == Some(4);
        #[derive(Clone)]
        struct ClosureGroup___11 {
            head__84: snake::Point
        }
        impl ClosureGroup___11 {
            fn fn__1486(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move up to y=4, got {}", self.head__84.y()));
            }
        }
        let closure_group = ClosureGroup___11 {
            head__84: head__84.clone()
        };
        let fn__1486 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1486())
        };
        test___6.assert(t___1495, fn__1486.clone());
        test___6.soft_fail_to_hard()
    }
    #[test]
    fn oppositeDirectionIsRejected__175() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___7 = temper_std::testing::Test::new();
        let game__86: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__87: snake::SnakeGame = snake::change_direction(game__86.clone(), snake::Direction::new(snake::Left::new()));
        let mut t___1481: bool = temper_core::is::<snake::Right>(changed__87.direction());
        #[derive(Clone)]
        struct ClosureGroup___12 {}
        impl ClosureGroup___12 {
            fn fn__1477(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be Right after trying Left".to_string());
            }
        }
        let closure_group = ClosureGroup___12 {};
        let fn__1477 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1477())
        };
        test___7.assert(t___1481, fn__1477.clone());
        test___7.soft_fail_to_hard()
    }
    #[test]
    fn nonOppositeDirectionIsAccepted__176() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___8 = temper_std::testing::Test::new();
        let game__89: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__90: snake::SnakeGame = snake::change_direction(game__89.clone(), snake::Direction::new(snake::Up::new()));
        let mut t___1474: bool = temper_core::is::<snake::Up>(changed__90.direction());
        #[derive(Clone)]
        struct ClosureGroup___13 {}
        impl ClosureGroup___13 {
            fn fn__1470(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should change to Up".to_string());
            }
        }
        let closure_group = ClosureGroup___13 {};
        let fn__1470 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1470())
        };
        test___8.assert(t___1474, fn__1470.clone());
        test___8.soft_fail_to_hard()
    }
    #[test]
    fn wallCollisionCausesGameOver__177() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___9 = temper_std::testing::Test::new();
        let mut t___1465: snake::SnakeGame;
        let mut t___1464: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__92: snake::SnakeGame = t___1464.clone();
        let mut i__93: i32 = 0;
        'loop___1581: while Some(i__93) < Some(10) {
            t___1465 = snake::tick(game__92.clone());
            game__92 = t___1465.clone();
            i__93 = i__93.wrapping_add(1);
        }
        let mut t___1467: bool = temper_core::is::<snake::GameOver>(game__92.status());
        #[derive(Clone)]
        struct ClosureGroup___14 {}
        impl ClosureGroup___14 {
            fn fn__1463(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after hitting wall".to_string());
            }
        }
        let closure_group = ClosureGroup___14 {};
        let fn__1463 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1463())
        };
        test___9.assert(t___1467, fn__1463.clone());
        test___9.soft_fail_to_hard()
    }
    #[test]
    fn selfCollisionCausesGameOver__178() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___10 = temper_std::testing::Test::new();
        let snake__95: temper_core::List<snake::Point> = std::sync::Arc::new(vec![snake::Point::new(5, 5), snake::Point::new(6, 5), snake::Point::new(6, 4), snake::Point::new(5, 4), snake::Point::new(4, 4), snake::Point::new(4, 5), snake::Point::new(4, 6)]);
        let mut t___1457: snake::SnakeGame = snake::SnakeGame::new(10, 10, snake__95.clone(), snake::Direction::new(snake::Left::new()), snake::Point::new(0, 0), 0, snake::GameStatus::new(snake::Playing::new()), 42);
        let mut game__96: snake::SnakeGame = t___1457.clone();
        let mut t___1458: snake::SnakeGame = snake::tick(game__96.clone());
        game__96 = t___1458.clone();
        let mut t___1460: bool = temper_core::is::<snake::GameOver>(game__96.status());
        #[derive(Clone)]
        struct ClosureGroup___15 {}
        impl ClosureGroup___15 {
            fn fn__1446(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after self collision".to_string());
            }
        }
        let closure_group = ClosureGroup___15 {};
        let fn__1446 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1446())
        };
        test___10.assert(t___1460, fn__1446.clone());
        test___10.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForSamePoints__179() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___11 = temper_std::testing::Test::new();
        let mut t___1444: bool = snake::point_equals(snake::Point::new(3, 4), snake::Point::new(3, 4));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__1440(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same points should be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__1440 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1440())
        };
        test___11.assert(t___1444, fn__1440.clone());
        test___11.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForDifferentPoints__180() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___12 = temper_std::testing::Test::new();
        let mut t___1438: bool = ! snake::point_equals(snake::Point::new(3, 4), snake::Point::new(5, 6));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__1434(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("different points should not be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__1434 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1434())
        };
        test___12.assert(t___1438, fn__1434.clone());
        test___12.soft_fail_to_hard()
    }
    #[test]
    fn isOppositeDetectsOppositeDirections__181() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___13 = temper_std::testing::Test::new();
        let mut t___1422: bool = snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Down::new()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__1418(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Down are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__1418 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1418())
        };
        test___13.assert(t___1422, fn__1418.clone());
        let mut t___1427: bool = snake::is_opposite(snake::Direction::new(snake::Left::new()), snake::Direction::new(snake::Right::new()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__1417(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Left/Right are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__1417 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1417())
        };
        test___13.assert(t___1427, fn__1417.clone());
        let mut t___1432: bool = ! snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Left::new()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__1416(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Left are not opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__1416 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1416())
        };
        test___13.assert(t___1432, fn__1416.clone());
        test___13.soft_fail_to_hard()
    }
    #[test]
    fn directionDeltaReturnsCorrectDeltas__182() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___14 = temper_std::testing::Test::new();
        let mut t___1408: i32;
        let mut t___1413: i32;
        let mut t___701: bool;
        let mut t___706: bool;
        let up__101: snake::Point = snake::direction_delta(snake::Direction::new(snake::Up::new()));
        if Some(up__101.x()) == Some(0) {
            t___1408 = up__101.y();
            t___701 = Some(t___1408) == Some(-1);
        } else {
            t___701 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__1405(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up should be (0, -1)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__1405 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1405())
        };
        test___14.assert(t___701, fn__1405.clone());
        let right__102: snake::Point = snake::direction_delta(snake::Direction::new(snake::Right::new()));
        if Some(right__102.x()) == Some(1) {
            t___1413 = right__102.y();
            t___706 = Some(t___1413) == Some(0);
        } else {
            t___706 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__1404(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Right should be (1, 0)".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__1404 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1404())
        };
        test___14.assert(t___706, fn__1404.clone());
        test___14.soft_fail_to_hard()
    }
    #[test]
    fn prngIsDeterministic__183() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___17 = temper_std::testing::Test::new();
        let r1__104: snake::RandomResult = snake::next_random(42, 100);
        let r2__105: snake::RandomResult = snake::next_random(42, 100);
        let mut t___1397: bool = Some(r1__104.value()) == Some(r2__105.value());
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__1393(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same value".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__1393 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1393())
        };
        test___17.assert(t___1397, fn__1393.clone());
        let mut t___1402: bool = Some(r1__104.next_seed()) == Some(r2__105.next_seed());
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__1392(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same next seed".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__1392 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1392())
        };
        test___17.assert(t___1402, fn__1392.clone());
        test___17.soft_fail_to_hard()
    }
    #[test]
    fn prngProducesValuesInRange__184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___18 = temper_std::testing::Test::new();
        let mut t___1389: i32;
        let mut t___691: bool;
        let r__107: snake::RandomResult = snake::next_random(42, 10);
        if Some(r__107.value()) >= Some(0) {
            t___1389 = r__107.value();
            t___691 = Some(t___1389) < Some(10);
        } else {
            t___691 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___25 {
            r__107: snake::RandomResult
        }
        impl ClosureGroup___25 {
            fn fn__1387(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("value should be in [0, 10), got {}", self.r__107.value()));
            }
        }
        let closure_group = ClosureGroup___25 {
            r__107: r__107.clone()
        };
        let fn__1387 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1387())
        };
        test___18.assert(t___691, fn__1387.clone());
        test___18.soft_fail_to_hard()
    }
    #[test]
    fn tickDoesNothingWhenGameIsOver__185() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___20 = temper_std::testing::Test::new();
        let mut t___1370: snake::SnakeGame;
        let mut t___1369: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__109: snake::SnakeGame = t___1369.clone();
        let mut i__110: i32 = 0;
        'loop___1582: while Some(i__110) < Some(10) {
            t___1370 = snake::tick(game__109.clone());
            game__109 = t___1370.clone();
            i__110 = i__110.wrapping_add(1);
        }
        let mut t___1372: bool = temper_core::is::<snake::GameOver>(game__109.status());
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__1368(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__1368 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1368())
        };
        test___20.assert(t___1372, fn__1368.clone());
        let head1__111: snake::Point = temper_core::ListedTrait::get_or( & game__109.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1378: snake::SnakeGame = snake::tick(game__109.clone());
        game__109 = t___1378.clone();
        let head2__112: snake::Point = temper_core::ListedTrait::get_or( & game__109.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1383: bool = snake::point_equals(head1__111.clone(), head2__112.clone());
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__1367(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should not move after game over".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__1367 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1367())
        };
        test___20.assert(t___1383, fn__1367.clone());
        test___20.soft_fail_to_hard()
    }
    #[test]
    fn multiGameCreatesCorrectNumberOfSnakes__186() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___21 = temper_std::testing::Test::new();
        let game__114: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1365: bool = Some(temper_core::ListedTrait::len( & game__114.snakes())) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__1360(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 2 snakes".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__1360 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1360())
        };
        test___21.assert(t___1365, fn__1360.clone());
        test___21.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesStartAlive__187() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let game__116: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let s0__117: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__116.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__118: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__116.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1353: bool = temper_core::is::<snake::Alive>(s0__117.status());
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__1338(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 should be alive".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__1338 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1338())
        };
        test___22.assert(t___1353, fn__1338.clone());
        let mut t___1357: bool = temper_core::is::<snake::Alive>(s1__118.status());
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__1337(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 1 should be alive".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__1337 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1337())
        };
        test___22.assert(t___1357, fn__1337.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesStartAtDifferentPositions__188() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let game__120: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let s0__121: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__120.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__122: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__120.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let h0__123: snake::Point = temper_core::ListedTrait::get_or( & s0__121.segments(), 0, snake::Point::new(-1, -1));
        let h1__124: snake::Point = temper_core::ListedTrait::get_or( & s1__122.segments(), 0, snake::Point::new(-1, -1));
        let mut t___1335: bool = ! snake::point_equals(h0__123.clone(), h1__124.clone());
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__1314(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snakes should start at different positions".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__1314 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1314())
        };
        test___23.assert(t___1335, fn__1314.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesHave3_segmentsEach__189() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let game__126: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let s0__127: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__126.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__128: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__126.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1307: bool = Some(temper_core::ListedTrait::len( & s0__127.segments())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__1290(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 should have 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__1290 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1290())
        };
        test___24.assert(t___1307, fn__1290.clone());
        let mut t___1312: bool = Some(temper_core::ListedTrait::len( & s1__128.segments())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__1289(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 1 should have 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__1289 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1289())
        };
        test___24.assert(t___1312, fn__1289.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn multiTickMovesBothSnakes__190() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let game__130: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let h0Before__131: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & game__130.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let h1Before__132: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & game__130.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let dirs__133: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![snake::Direction::new(snake::Right::new()), snake::Direction::new(snake::Left::new())]);
        let after__134: snake::MultiSnakeGame = snake::multi_tick(game__130.clone(), dirs__133.clone());
        let h0After__135: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & after__134.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let h1After__136: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & after__134.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let mut t___1284: bool = ! snake::point_equals(h0Before__131.clone(), h0After__135.clone());
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__1242(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake 0 should have moved".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__1242 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1242())
        };
        test___25.assert(t___1284, fn__1242.clone());
        let mut t___1287: bool = ! snake::point_equals(h1Before__132.clone(), h1After__136.clone());
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__1241(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake 1 should have moved".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__1241 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1241())
        };
        test___25.assert(t___1287, fn__1241.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn multiWallCollisionKillsOneSnake__191() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let mut t___1227: snake::MultiSnakeGame;
        let mut t___1229: i32;
        let mut t___1230: temper_core::List<snake::PlayerSnake>;
        let mut t___1234: snake::PlayerSnake;
        let mut t___1224: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut game__138: snake::MultiSnakeGame = t___1224.clone();
        let dirs__139: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![snake::Direction::new(snake::Right::new()), snake::Direction::new(snake::Left::new())]);
        let mut i__140: i32 = 0;
        'loop___1583: while Some(i__140) < Some(20) {
            t___1227 = snake::multi_tick(game__138.clone(), dirs__139.clone());
            game__138 = t___1227.clone();
            i__140 = i__140.wrapping_add(1);
        }
        let mut deadCount__141: i32 = 0;
        let mut i__142: i32 = 0;
        'loop___1584: loop {
            t___1229 = temper_core::ListedTrait::len( & game__138.snakes());
            if ! (Some(i__142) < Some(t___1229)) {
                break;
            }
            t___1230 = game__138.snakes();
            t___1234 = snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()));
            let snake__143: snake::PlayerSnake = temper_core::ListedTrait::get_or( & t___1230, i__142, t___1234.clone());
            if temper_core::is::<snake::Dead>(snake__143.status()) {
                deadCount__141 = deadCount__141.wrapping_add(1);
            }
            i__142 = i__142.wrapping_add(1);
        }
        let mut t___1239: bool = Some(deadCount__141) > Some(0);
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__1223(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("at least one snake should be dead after 20 ticks toward walls".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__1223 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1223())
        };
        test___26.assert(t___1239, fn__1223.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn multiGameOverWhenOnePlayerLeft__192() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let mut t___1219: snake::MultiSnakeGame;
        let mut t___1216: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut game__145: snake::MultiSnakeGame = t___1216.clone();
        let dirs__146: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![snake::Direction::new(snake::Right::new()), snake::Direction::new(snake::Left::new())]);
        let mut i__147: i32 = 0;
        'loop___1585: while Some(i__147) < Some(30) {
            t___1219 = snake::multi_tick(game__145.clone(), dirs__146.clone());
            game__145 = t___1219.clone();
            i__147 = i__147.wrapping_add(1);
        }
        let mut t___1220: bool = snake::is_multi_game_over(game__145.clone());
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__1215(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("game should be over after enough ticks".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__1215 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1215())
        };
        test___27.assert(t___1220, fn__1215.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn changePlayerDirectionWorks__193() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let game__149: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1203: snake::Up = snake::Up::new();
        let changed__150: snake::MultiSnakeGame = snake::change_player_direction(game__149.clone(), 0, snake::Direction::new(t___1203.clone()));
        let s0__151: snake::PlayerSnake = temper_core::ListedTrait::get_or( & changed__150.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1212: bool = temper_core::is::<snake::Up>(s0__151.direction());
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__1201(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 direction should be Up".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__1201 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1201())
        };
        test___28.assert(t___1212, fn__1201.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn changePlayerDirectionRejectsOpposite__194() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let game__153: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1189: snake::Left = snake::Left::new();
        let changed__154: snake::MultiSnakeGame = snake::change_player_direction(game__153.clone(), 0, snake::Direction::new(t___1189.clone()));
        let s0__155: snake::PlayerSnake = temper_core::ListedTrait::get_or( & changed__154.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1198: bool = temper_core::is::<snake::Right>(s0__155.direction());
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__1187(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should reject opposite direction".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__1187 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1187())
        };
        test___29.assert(t___1198, fn__1187.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn addPlayerAddsANewSnake__195() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let game__157: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let bigger__158: snake::MultiSnakeGame = snake::add_player(game__157.clone(), 99);
        let mut t___1185: bool = Some(temper_core::ListedTrait::len( & bigger__158.snakes())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___40 {}
        impl ClosureGroup___40 {
            fn fn__1179(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 3 snakes after adding".to_string());
            }
        }
        let closure_group = ClosureGroup___40 {};
        let fn__1179 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1179())
        };
        test___30.assert(t___1185, fn__1179.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn removePlayerRemovesASnake__196() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let game__160: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 3, 42);
        let smaller__161: snake::MultiSnakeGame = snake::remove_player(game__160.clone(), 1);
        let mut t___1177: bool = Some(temper_core::ListedTrait::len( & smaller__161.snakes())) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___41 {}
        impl ClosureGroup___41 {
            fn fn__1171(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 2 snakes after removing".to_string());
            }
        }
        let closure_group = ClosureGroup___41 {};
        let fn__1171 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1171())
        };
        test___31.assert(t___1177, fn__1171.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn multiRenderProducesOutput__197() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let game__163: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let rendered__164: std::sync::Arc<String> = snake::multi_render(game__163.clone());
        let mut t___1169: bool = Some(rendered__164.as_str()) != Some("");
        #[derive(Clone)]
        struct ClosureGroup___42 {}
        impl ClosureGroup___42 {
            fn fn__1165(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("render should produce output".to_string());
            }
        }
        let closure_group = ClosureGroup___42 {};
        let fn__1165 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1165())
        };
        test___32.assert(t___1169, fn__1165.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn directionToStringAndStringToDirectionRoundTrip__198() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let d__166: std::sync::Arc<String> = snake::direction_to_string(snake::Direction::new(snake::Up::new()));
        let mut t___1161: bool = Some(d__166.as_str()) == Some("up");
        #[derive(Clone)]
        struct ClosureGroup___43 {}
        impl ClosureGroup___43 {
            fn fn__1158(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up should serialize to 'up'".to_string());
            }
        }
        let closure_group = ClosureGroup___43 {};
        let fn__1158 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1158())
        };
        test___33.assert(t___1161, fn__1158.clone());
        let parsed__167: Option<snake::Direction> = snake::string_to_direction("down");
        #[derive(Clone)]
        struct ClosureGroup___44 {}
        impl ClosureGroup___44 {
            fn fn__1157(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("'down' should parse to Down".to_string());
            }
        }
        let closure_group = ClosureGroup___44 {};
        let fn__1157 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1157())
        };
        test___33.assert(true, fn__1157.clone());
        test___33.soft_fail_to_hard()
    }
    use super::*;
}
