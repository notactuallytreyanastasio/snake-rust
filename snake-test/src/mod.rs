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
    fn initialStateHasSnakeNearCenter__170() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___0 = temper_std::testing::Test::new();
        let game__65: snake::SnakeGame = snake::new_game(10, 10, 42);
        let head__66: snake::Point = temper_core::ListedTrait::get_or( & game__65.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1569: bool = Some(head__66.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___1 {
            head__66: snake::Point
        }
        impl ClosureGroup___1 {
            fn fn__1562(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should be 5, got {}", self.head__66.x()));
            }
        }
        let closure_group = ClosureGroup___1 {
            head__66: head__66.clone()
        };
        let fn__1562 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1562())
        };
        test___0.assert(t___1569, fn__1562.clone());
        let mut t___1573: bool = Some(head__66.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___2 {
            head__66: snake::Point
        }
        impl ClosureGroup___2 {
            fn fn__1561(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should be 5, got {}", self.head__66.y()));
            }
        }
        let closure_group = ClosureGroup___2 {
            head__66: head__66.clone()
        };
        let fn__1561 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1561())
        };
        test___0.assert(t___1573, fn__1561.clone());
        let mut t___1578: bool = Some(temper_core::ListedTrait::len( & game__65.snake())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__1560(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should start with 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__1560 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1560())
        };
        test___0.assert(t___1578, fn__1560.clone());
        test___0.soft_fail_to_hard()
    }
    #[test]
    fn initialStatusIsPlaying__171() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___1 = temper_std::testing::Test::new();
        let game__68: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1553: bool = temper_core::is::<snake::Playing>(game__68.status());
        #[derive(Clone)]
        struct ClosureGroup___4 {}
        impl ClosureGroup___4 {
            fn fn__1550(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial status should be Playing".to_string());
            }
        }
        let closure_group = ClosureGroup___4 {};
        let fn__1550 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1550())
        };
        test___1.assert(t___1553, fn__1550.clone());
        test___1.soft_fail_to_hard()
    }
    #[test]
    fn initialDirectionIsRight__172() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___2 = temper_std::testing::Test::new();
        let game__70: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1547: bool = temper_core::is::<snake::Right>(game__70.direction());
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__1544(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial direction should be Right".to_string());
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__1544 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1544())
        };
        test___2.assert(t___1547, fn__1544.clone());
        test___2.soft_fail_to_hard()
    }
    #[test]
    fn initialScoreIs0__173() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___3 = temper_std::testing::Test::new();
        let game__72: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___1542: bool = Some(game__72.score()) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___6 {}
        impl ClosureGroup___6 {
            fn fn__1538(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial score should be 0".to_string());
            }
        }
        let closure_group = ClosureGroup___6 {};
        let fn__1538 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1538())
        };
        test___3.assert(t___1542, fn__1538.clone());
        test___3.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesRight__174() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___4 = temper_std::testing::Test::new();
        let game__74: snake::SnakeGame = snake::new_game(10, 10, 42);
        let moved__75: snake::SnakeGame = snake::tick(game__74.clone());
        let head__76: snake::Point = temper_core::ListedTrait::get_or( & moved__75.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1532: bool = Some(head__76.x()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___7 {
            head__76: snake::Point
        }
        impl ClosureGroup___7 {
            fn fn__1524(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move right to x=6, got {}", self.head__76.x()));
            }
        }
        let closure_group = ClosureGroup___7 {
            head__76: head__76.clone()
        };
        let fn__1524 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1524())
        };
        test___4.assert(t___1532, fn__1524.clone());
        let mut t___1536: bool = Some(head__76.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___8 {
            head__76: snake::Point
        }
        impl ClosureGroup___8 {
            fn fn__1523(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should stay 5, got {}", self.head__76.y()));
            }
        }
        let closure_group = ClosureGroup___8 {
            head__76: head__76.clone()
        };
        let fn__1523 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1523())
        };
        test___4.assert(t___1536, fn__1523.clone());
        test___4.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesDown__175() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___5 = temper_std::testing::Test::new();
        let game__78: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Down::new()));
        let moved__79: snake::SnakeGame = snake::tick(game__78.clone());
        let head__80: snake::Point = temper_core::ListedTrait::get_or( & moved__79.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1513: bool = Some(head__80.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___9 {
            head__80: snake::Point
        }
        impl ClosureGroup___9 {
            fn fn__1504(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should stay 5, got {}", self.head__80.x()));
            }
        }
        let closure_group = ClosureGroup___9 {
            head__80: head__80.clone()
        };
        let fn__1504 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1504())
        };
        test___5.assert(t___1513, fn__1504.clone());
        let mut t___1517: bool = Some(head__80.y()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___10 {
            head__80: snake::Point
        }
        impl ClosureGroup___10 {
            fn fn__1503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move down to y=6, got {}", self.head__80.y()));
            }
        }
        let closure_group = ClosureGroup___10 {
            head__80: head__80.clone()
        };
        let fn__1503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1503())
        };
        test___5.assert(t___1517, fn__1503.clone());
        test___5.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesUp__176() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___6 = temper_std::testing::Test::new();
        let game__82: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Up::new()));
        let moved__83: snake::SnakeGame = snake::tick(game__82.clone());
        let head__84: snake::Point = temper_core::ListedTrait::get_or( & moved__83.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1497: bool = Some(head__84.y()) == Some(4);
        #[derive(Clone)]
        struct ClosureGroup___11 {
            head__84: snake::Point
        }
        impl ClosureGroup___11 {
            fn fn__1488(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move up to y=4, got {}", self.head__84.y()));
            }
        }
        let closure_group = ClosureGroup___11 {
            head__84: head__84.clone()
        };
        let fn__1488 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1488())
        };
        test___6.assert(t___1497, fn__1488.clone());
        test___6.soft_fail_to_hard()
    }
    #[test]
    fn oppositeDirectionIsRejected__177() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___7 = temper_std::testing::Test::new();
        let game__86: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__87: snake::SnakeGame = snake::change_direction(game__86.clone(), snake::Direction::new(snake::Left::new()));
        let mut t___1483: bool = temper_core::is::<snake::Right>(changed__87.direction());
        #[derive(Clone)]
        struct ClosureGroup___12 {}
        impl ClosureGroup___12 {
            fn fn__1479(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be Right after trying Left".to_string());
            }
        }
        let closure_group = ClosureGroup___12 {};
        let fn__1479 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1479())
        };
        test___7.assert(t___1483, fn__1479.clone());
        test___7.soft_fail_to_hard()
    }
    #[test]
    fn nonOppositeDirectionIsAccepted__178() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___8 = temper_std::testing::Test::new();
        let game__89: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__90: snake::SnakeGame = snake::change_direction(game__89.clone(), snake::Direction::new(snake::Up::new()));
        let mut t___1476: bool = temper_core::is::<snake::Up>(changed__90.direction());
        #[derive(Clone)]
        struct ClosureGroup___13 {}
        impl ClosureGroup___13 {
            fn fn__1472(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should change to Up".to_string());
            }
        }
        let closure_group = ClosureGroup___13 {};
        let fn__1472 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1472())
        };
        test___8.assert(t___1476, fn__1472.clone());
        test___8.soft_fail_to_hard()
    }
    #[test]
    fn wallCollisionCausesGameOver__179() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___9 = temper_std::testing::Test::new();
        let mut t___1467: snake::SnakeGame;
        let mut t___1466: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__92: snake::SnakeGame = t___1466.clone();
        let mut i__93: i32 = 0;
        'loop___1583: while Some(i__93) < Some(10) {
            t___1467 = snake::tick(game__92.clone());
            game__92 = t___1467.clone();
            i__93 = i__93.wrapping_add(1);
        }
        let mut t___1469: bool = temper_core::is::<snake::GameOver>(game__92.status());
        #[derive(Clone)]
        struct ClosureGroup___14 {}
        impl ClosureGroup___14 {
            fn fn__1465(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after hitting wall".to_string());
            }
        }
        let closure_group = ClosureGroup___14 {};
        let fn__1465 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1465())
        };
        test___9.assert(t___1469, fn__1465.clone());
        test___9.soft_fail_to_hard()
    }
    #[test]
    fn selfCollisionCausesGameOver__180() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___10 = temper_std::testing::Test::new();
        let snake__95: temper_core::List<snake::Point> = std::sync::Arc::new(vec![snake::Point::new(5, 5), snake::Point::new(6, 5), snake::Point::new(6, 4), snake::Point::new(5, 4), snake::Point::new(4, 4), snake::Point::new(4, 5), snake::Point::new(4, 6)]);
        let mut t___1459: snake::SnakeGame = snake::SnakeGame::new(10, 10, snake__95.clone(), snake::Direction::new(snake::Left::new()), snake::Point::new(0, 0), 0, snake::GameStatus::new(snake::Playing::new()), 42);
        let mut game__96: snake::SnakeGame = t___1459.clone();
        let mut t___1460: snake::SnakeGame = snake::tick(game__96.clone());
        game__96 = t___1460.clone();
        let mut t___1462: bool = temper_core::is::<snake::GameOver>(game__96.status());
        #[derive(Clone)]
        struct ClosureGroup___15 {}
        impl ClosureGroup___15 {
            fn fn__1448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after self collision".to_string());
            }
        }
        let closure_group = ClosureGroup___15 {};
        let fn__1448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1448())
        };
        test___10.assert(t___1462, fn__1448.clone());
        test___10.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForSamePoints__181() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___11 = temper_std::testing::Test::new();
        let mut t___1446: bool = snake::point_equals(snake::Point::new(3, 4), snake::Point::new(3, 4));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__1442(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same points should be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__1442 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1442())
        };
        test___11.assert(t___1446, fn__1442.clone());
        test___11.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForDifferentPoints__182() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___12 = temper_std::testing::Test::new();
        let mut t___1440: bool = ! snake::point_equals(snake::Point::new(3, 4), snake::Point::new(5, 6));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__1436(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("different points should not be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__1436 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1436())
        };
        test___12.assert(t___1440, fn__1436.clone());
        test___12.soft_fail_to_hard()
    }
    #[test]
    fn isOppositeDetectsOppositeDirections__183() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___13 = temper_std::testing::Test::new();
        let mut t___1424: bool = snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Down::new()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__1420(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Down are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__1420 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1420())
        };
        test___13.assert(t___1424, fn__1420.clone());
        let mut t___1429: bool = snake::is_opposite(snake::Direction::new(snake::Left::new()), snake::Direction::new(snake::Right::new()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__1419(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Left/Right are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__1419 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1419())
        };
        test___13.assert(t___1429, fn__1419.clone());
        let mut t___1434: bool = ! snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Left::new()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__1418(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Left are not opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__1418 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1418())
        };
        test___13.assert(t___1434, fn__1418.clone());
        test___13.soft_fail_to_hard()
    }
    #[test]
    fn directionDeltaReturnsCorrectDeltas__184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___14 = temper_std::testing::Test::new();
        let mut t___1410: i32;
        let mut t___1415: i32;
        let mut t___703: bool;
        let mut t___708: bool;
        let up__101: snake::Point = snake::direction_delta(snake::Direction::new(snake::Up::new()));
        if Some(up__101.x()) == Some(0) {
            t___1410 = up__101.y();
            t___703 = Some(t___1410) == Some(-1);
        } else {
            t___703 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__1407(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up should be (0, -1)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__1407 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1407())
        };
        test___14.assert(t___703, fn__1407.clone());
        let right__102: snake::Point = snake::direction_delta(snake::Direction::new(snake::Right::new()));
        if Some(right__102.x()) == Some(1) {
            t___1415 = right__102.y();
            t___708 = Some(t___1415) == Some(0);
        } else {
            t___708 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__1406(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Right should be (1, 0)".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__1406 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1406())
        };
        test___14.assert(t___708, fn__1406.clone());
        test___14.soft_fail_to_hard()
    }
    #[test]
    fn prngIsDeterministic__185() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___17 = temper_std::testing::Test::new();
        let r1__104: snake::RandomResult = snake::next_random(42, 100);
        let r2__105: snake::RandomResult = snake::next_random(42, 100);
        let mut t___1399: bool = Some(r1__104.value()) == Some(r2__105.value());
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__1395(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same value".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__1395 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1395())
        };
        test___17.assert(t___1399, fn__1395.clone());
        let mut t___1404: bool = Some(r1__104.next_seed()) == Some(r2__105.next_seed());
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__1394(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same next seed".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__1394 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1394())
        };
        test___17.assert(t___1404, fn__1394.clone());
        test___17.soft_fail_to_hard()
    }
    #[test]
    fn prngProducesValuesInRange__186() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___18 = temper_std::testing::Test::new();
        let mut t___1391: i32;
        let mut t___693: bool;
        let r__107: snake::RandomResult = snake::next_random(42, 10);
        if Some(r__107.value()) >= Some(0) {
            t___1391 = r__107.value();
            t___693 = Some(t___1391) < Some(10);
        } else {
            t___693 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___25 {
            r__107: snake::RandomResult
        }
        impl ClosureGroup___25 {
            fn fn__1389(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("value should be in [0, 10), got {}", self.r__107.value()));
            }
        }
        let closure_group = ClosureGroup___25 {
            r__107: r__107.clone()
        };
        let fn__1389 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1389())
        };
        test___18.assert(t___693, fn__1389.clone());
        test___18.soft_fail_to_hard()
    }
    #[test]
    fn tickDoesNothingWhenGameIsOver__187() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___20 = temper_std::testing::Test::new();
        let mut t___1372: snake::SnakeGame;
        let mut t___1371: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__109: snake::SnakeGame = t___1371.clone();
        let mut i__110: i32 = 0;
        'loop___1584: while Some(i__110) < Some(10) {
            t___1372 = snake::tick(game__109.clone());
            game__109 = t___1372.clone();
            i__110 = i__110.wrapping_add(1);
        }
        let mut t___1374: bool = temper_core::is::<snake::GameOver>(game__109.status());
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__1370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__1370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1370())
        };
        test___20.assert(t___1374, fn__1370.clone());
        let head1__111: snake::Point = temper_core::ListedTrait::get_or( & game__109.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1380: snake::SnakeGame = snake::tick(game__109.clone());
        game__109 = t___1380.clone();
        let head2__112: snake::Point = temper_core::ListedTrait::get_or( & game__109.snake(), 0, snake::Point::new(-1, -1));
        let mut t___1385: bool = snake::point_equals(head1__111.clone(), head2__112.clone());
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__1369(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should not move after game over".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__1369 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1369())
        };
        test___20.assert(t___1385, fn__1369.clone());
        test___20.soft_fail_to_hard()
    }
    #[test]
    fn multiGameCreatesCorrectNumberOfSnakes__188() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___21 = temper_std::testing::Test::new();
        let game__114: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1367: bool = Some(temper_core::ListedTrait::len( & game__114.snakes())) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__1362(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 2 snakes".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__1362 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1362())
        };
        test___21.assert(t___1367, fn__1362.clone());
        test___21.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesStartAlive__189() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let game__116: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let s0__117: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__116.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__118: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__116.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1355: bool = temper_core::is::<snake::Alive>(s0__117.status());
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__1340(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 should be alive".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__1340 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1340())
        };
        test___22.assert(t___1355, fn__1340.clone());
        let mut t___1359: bool = temper_core::is::<snake::Alive>(s1__118.status());
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__1339(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 1 should be alive".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__1339 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1339())
        };
        test___22.assert(t___1359, fn__1339.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesStartAtDifferentPositions__190() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let game__120: snake::MultiSnakeGame = snake::new_multi_game(60, 30, 2, 42);
        let s0__121: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__120.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__122: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__120.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let h0__123: snake::Point = temper_core::ListedTrait::get_or( & s0__121.segments(), 0, snake::Point::new(-1, -1));
        let h1__124: snake::Point = temper_core::ListedTrait::get_or( & s1__122.segments(), 0, snake::Point::new(-1, -1));
        let mut t___1337: bool = ! snake::point_equals(h0__123.clone(), h1__124.clone());
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__1316(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snakes should start at different positions".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__1316 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1316())
        };
        test___23.assert(t___1337, fn__1316.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn multiGameSnakesHave3_segmentsEach__191() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let game__126: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let s0__127: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__126.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__128: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__126.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1309: bool = Some(temper_core::ListedTrait::len( & s0__127.segments())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__1292(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 should have 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__1292 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1292())
        };
        test___24.assert(t___1309, fn__1292.clone());
        let mut t___1314: bool = Some(temper_core::ListedTrait::len( & s1__128.segments())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__1291(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 1 should have 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__1291 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1291())
        };
        test___24.assert(t___1314, fn__1291.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn multiTickMovesBothSnakes__192() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let game__130: snake::MultiSnakeGame = snake::new_multi_game(60, 30, 2, 42);
        let s0__131: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__130.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let s1__132: snake::PlayerSnake = temper_core::ListedTrait::get_or( & game__130.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let h0Before__133: snake::Point = temper_core::ListedTrait::get_or( & s0__131.segments(), 0, snake::Point::new(0, 0));
        let h1Before__134: snake::Point = temper_core::ListedTrait::get_or( & s1__132.segments(), 0, snake::Point::new(0, 0));
        let dirs__135: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![s0__131.direction(), s1__132.direction()]);
        let after__136: snake::MultiSnakeGame = snake::multi_tick(game__130.clone(), dirs__135.clone());
        let h0After__137: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & after__136.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let h1After__138: snake::Point = temper_core::ListedTrait::get_or( & temper_core::ListedTrait::get_or( & after__136.snakes(), 1, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()))).segments(), 0, snake::Point::new(0, 0));
        let mut t___1286: bool = ! snake::point_equals(h0Before__133.clone(), h0After__137.clone());
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__1244(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake 0 should have moved".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__1244 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1244())
        };
        test___25.assert(t___1286, fn__1244.clone());
        let mut t___1289: bool = ! snake::point_equals(h1Before__134.clone(), h1After__138.clone());
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__1243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake 1 should have moved".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__1243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1243())
        };
        test___25.assert(t___1289, fn__1243.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn multiWallCollisionKillsOneSnake__193() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let mut t___1229: snake::MultiSnakeGame;
        let mut t___1231: i32;
        let mut t___1232: temper_core::List<snake::PlayerSnake>;
        let mut t___1236: snake::PlayerSnake;
        let mut t___1226: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut game__140: snake::MultiSnakeGame = t___1226.clone();
        let dirs__141: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![snake::Direction::new(snake::Right::new()), snake::Direction::new(snake::Left::new())]);
        let mut i__142: i32 = 0;
        'loop___1585: while Some(i__142) < Some(20) {
            t___1229 = snake::multi_tick(game__140.clone(), dirs__141.clone());
            game__140 = t___1229.clone();
            i__142 = i__142.wrapping_add(1);
        }
        let mut deadCount__143: i32 = 0;
        let mut i__144: i32 = 0;
        'loop___1586: loop {
            t___1231 = temper_core::ListedTrait::len( & game__140.snakes());
            if ! (Some(i__144) < Some(t___1231)) {
                break;
            }
            t___1232 = game__140.snakes();
            t___1236 = snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new()));
            let snake__145: snake::PlayerSnake = temper_core::ListedTrait::get_or( & t___1232, i__144, t___1236.clone());
            if temper_core::is::<snake::Dead>(snake__145.status()) {
                deadCount__143 = deadCount__143.wrapping_add(1);
            }
            i__144 = i__144.wrapping_add(1);
        }
        let mut t___1241: bool = Some(deadCount__143) > Some(0);
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__1225(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("at least one snake should be dead after 20 ticks toward walls".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__1225 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1225())
        };
        test___26.assert(t___1241, fn__1225.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn multiGameOverWhenOnePlayerLeft__194() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let mut t___1221: snake::MultiSnakeGame;
        let mut t___1218: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut game__147: snake::MultiSnakeGame = t___1218.clone();
        let dirs__148: temper_core::List<snake::Direction> = std::sync::Arc::new(vec![snake::Direction::new(snake::Right::new()), snake::Direction::new(snake::Left::new())]);
        let mut i__149: i32 = 0;
        'loop___1587: while Some(i__149) < Some(30) {
            t___1221 = snake::multi_tick(game__147.clone(), dirs__148.clone());
            game__147 = t___1221.clone();
            i__149 = i__149.wrapping_add(1);
        }
        let mut t___1222: bool = snake::is_multi_game_over(game__147.clone());
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__1217(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("game should be over after enough ticks".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__1217 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1217())
        };
        test___27.assert(t___1222, fn__1217.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn changePlayerDirectionWorks__195() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let game__151: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1205: snake::Up = snake::Up::new();
        let changed__152: snake::MultiSnakeGame = snake::change_player_direction(game__151.clone(), 0, snake::Direction::new(t___1205.clone()));
        let s0__153: snake::PlayerSnake = temper_core::ListedTrait::get_or( & changed__152.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1214: bool = temper_core::is::<snake::Up>(s0__153.direction());
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__1203(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("player 0 direction should be Up".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__1203 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1203())
        };
        test___28.assert(t___1214, fn__1203.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn changePlayerDirectionRejectsOpposite__196() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let game__155: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let mut t___1191: snake::Left = snake::Left::new();
        let changed__156: snake::MultiSnakeGame = snake::change_player_direction(game__155.clone(), 0, snake::Direction::new(t___1191.clone()));
        let s0__157: snake::PlayerSnake = temper_core::ListedTrait::get_or( & changed__156.snakes(), 0, snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
        let mut t___1200: bool = temper_core::is::<snake::Right>(s0__157.direction());
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__1189(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should reject opposite direction".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__1189 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1189())
        };
        test___29.assert(t___1200, fn__1189.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn addPlayerAddsANewSnake__197() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let game__159: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let bigger__160: snake::MultiSnakeGame = snake::add_player(game__159.clone(), 99);
        let mut t___1187: bool = Some(temper_core::ListedTrait::len( & bigger__160.snakes())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___40 {}
        impl ClosureGroup___40 {
            fn fn__1181(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 3 snakes after adding".to_string());
            }
        }
        let closure_group = ClosureGroup___40 {};
        let fn__1181 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1181())
        };
        test___30.assert(t___1187, fn__1181.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn removePlayerRemovesASnake__198() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let game__162: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 3, 42);
        let smaller__163: snake::MultiSnakeGame = snake::remove_player(game__162.clone(), 1);
        let mut t___1179: bool = Some(temper_core::ListedTrait::len( & smaller__163.snakes())) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___41 {}
        impl ClosureGroup___41 {
            fn fn__1173(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have 2 snakes after removing".to_string());
            }
        }
        let closure_group = ClosureGroup___41 {};
        let fn__1173 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1173())
        };
        test___31.assert(t___1179, fn__1173.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn multiRenderProducesOutput__199() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let game__165: snake::MultiSnakeGame = snake::new_multi_game(20, 10, 2, 42);
        let rendered__166: std::sync::Arc<String> = snake::multi_render(game__165.clone());
        let mut t___1171: bool = Some(rendered__166.as_str()) != Some("");
        #[derive(Clone)]
        struct ClosureGroup___42 {}
        impl ClosureGroup___42 {
            fn fn__1167(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("render should produce output".to_string());
            }
        }
        let closure_group = ClosureGroup___42 {};
        let fn__1167 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1167())
        };
        test___32.assert(t___1171, fn__1167.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn directionToStringAndStringToDirectionRoundTrip__200() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let d__168: std::sync::Arc<String> = snake::direction_to_string(snake::Direction::new(snake::Up::new()));
        let mut t___1163: bool = Some(d__168.as_str()) == Some("up");
        #[derive(Clone)]
        struct ClosureGroup___43 {}
        impl ClosureGroup___43 {
            fn fn__1160(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up should serialize to 'up'".to_string());
            }
        }
        let closure_group = ClosureGroup___43 {};
        let fn__1160 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1160())
        };
        test___33.assert(t___1163, fn__1160.clone());
        let parsed__169: Option<snake::Direction> = snake::string_to_direction("down");
        #[derive(Clone)]
        struct ClosureGroup___44 {}
        impl ClosureGroup___44 {
            fn fn__1159(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("'down' should parse to Down".to_string());
            }
        }
        let closure_group = ClosureGroup___44 {};
        let fn__1159 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1159())
        };
        test___33.assert(true, fn__1159.clone());
        test___33.soft_fail_to_hard()
    }
    use super::*;
}
