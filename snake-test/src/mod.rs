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
    fn initialStateHasSnakeNearCenter__86() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___0 = temper_std::testing::Test::new();
        let game__38: snake::SnakeGame = snake::new_game(10, 10, 42);
        let head__39: snake::Point = temper_core::ListedTrait::get_or( & game__38.snake(), 0, snake::Point::new(-1, -1));
        let mut t___766: bool = Some(head__39.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___1 {
            head__39: snake::Point
        }
        impl ClosureGroup___1 {
            fn fn__759(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should be 5, got {}", self.head__39.x()));
            }
        }
        let closure_group = ClosureGroup___1 {
            head__39: head__39.clone()
        };
        let fn__759 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__759())
        };
        test___0.assert(t___766, fn__759.clone());
        let mut t___770: bool = Some(head__39.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___2 {
            head__39: snake::Point
        }
        impl ClosureGroup___2 {
            fn fn__758(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should be 5, got {}", self.head__39.y()));
            }
        }
        let closure_group = ClosureGroup___2 {
            head__39: head__39.clone()
        };
        let fn__758 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__758())
        };
        test___0.assert(t___770, fn__758.clone());
        let mut t___775: bool = Some(temper_core::ListedTrait::len( & game__38.snake())) == Some(3);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__757(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should start with 3 segments".to_string());
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__757 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__757())
        };
        test___0.assert(t___775, fn__757.clone());
        test___0.soft_fail_to_hard()
    }
    #[test]
    fn initialStatusIsPlaying__87() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___1 = temper_std::testing::Test::new();
        let game__41: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___750: bool = temper_core::is::<snake::Playing>(game__41.status());
        #[derive(Clone)]
        struct ClosureGroup___4 {}
        impl ClosureGroup___4 {
            fn fn__747(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial status should be Playing".to_string());
            }
        }
        let closure_group = ClosureGroup___4 {};
        let fn__747 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__747())
        };
        test___1.assert(t___750, fn__747.clone());
        test___1.soft_fail_to_hard()
    }
    #[test]
    fn initialDirectionIsRight__88() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___2 = temper_std::testing::Test::new();
        let game__43: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___744: bool = temper_core::is::<snake::Right>(game__43.direction());
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__741(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial direction should be Right".to_string());
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__741 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__741())
        };
        test___2.assert(t___744, fn__741.clone());
        test___2.soft_fail_to_hard()
    }
    #[test]
    fn initialScoreIs0__89() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___3 = temper_std::testing::Test::new();
        let game__45: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut t___739: bool = Some(game__45.score()) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___6 {}
        impl ClosureGroup___6 {
            fn fn__735(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("initial score should be 0".to_string());
            }
        }
        let closure_group = ClosureGroup___6 {};
        let fn__735 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__735())
        };
        test___3.assert(t___739, fn__735.clone());
        test___3.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesRight__90() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___4 = temper_std::testing::Test::new();
        let game__47: snake::SnakeGame = snake::new_game(10, 10, 42);
        let moved__48: snake::SnakeGame = snake::tick(game__47.clone());
        let head__49: snake::Point = temper_core::ListedTrait::get_or( & moved__48.snake(), 0, snake::Point::new(-1, -1));
        let mut t___729: bool = Some(head__49.x()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___7 {
            head__49: snake::Point
        }
        impl ClosureGroup___7 {
            fn fn__721(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move right to x=6, got {}", self.head__49.x()));
            }
        }
        let closure_group = ClosureGroup___7 {
            head__49: head__49.clone()
        };
        let fn__721 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__721())
        };
        test___4.assert(t___729, fn__721.clone());
        let mut t___733: bool = Some(head__49.y()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___8 {
            head__49: snake::Point
        }
        impl ClosureGroup___8 {
            fn fn__720(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head y should stay 5, got {}", self.head__49.y()));
            }
        }
        let closure_group = ClosureGroup___8 {
            head__49: head__49.clone()
        };
        let fn__720 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__720())
        };
        test___4.assert(t___733, fn__720.clone());
        test___4.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesDown__91() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___5 = temper_std::testing::Test::new();
        let game__51: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Down::new()));
        let moved__52: snake::SnakeGame = snake::tick(game__51.clone());
        let head__53: snake::Point = temper_core::ListedTrait::get_or( & moved__52.snake(), 0, snake::Point::new(-1, -1));
        let mut t___710: bool = Some(head__53.x()) == Some(5);
        #[derive(Clone)]
        struct ClosureGroup___9 {
            head__53: snake::Point
        }
        impl ClosureGroup___9 {
            fn fn__701(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head x should stay 5, got {}", self.head__53.x()));
            }
        }
        let closure_group = ClosureGroup___9 {
            head__53: head__53.clone()
        };
        let fn__701 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__701())
        };
        test___5.assert(t___710, fn__701.clone());
        let mut t___714: bool = Some(head__53.y()) == Some(6);
        #[derive(Clone)]
        struct ClosureGroup___10 {
            head__53: snake::Point
        }
        impl ClosureGroup___10 {
            fn fn__700(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move down to y=6, got {}", self.head__53.y()));
            }
        }
        let closure_group = ClosureGroup___10 {
            head__53: head__53.clone()
        };
        let fn__700 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__700())
        };
        test___5.assert(t___714, fn__700.clone());
        test___5.soft_fail_to_hard()
    }
    #[test]
    fn snakeMovesUp__92() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___6 = temper_std::testing::Test::new();
        let game__55: snake::SnakeGame = snake::change_direction(snake::new_game(10, 10, 42), snake::Direction::new(snake::Up::new()));
        let moved__56: snake::SnakeGame = snake::tick(game__55.clone());
        let head__57: snake::Point = temper_core::ListedTrait::get_or( & moved__56.snake(), 0, snake::Point::new(-1, -1));
        let mut t___694: bool = Some(head__57.y()) == Some(4);
        #[derive(Clone)]
        struct ClosureGroup___11 {
            head__57: snake::Point
        }
        impl ClosureGroup___11 {
            fn fn__685(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("head should move up to y=4, got {}", self.head__57.y()));
            }
        }
        let closure_group = ClosureGroup___11 {
            head__57: head__57.clone()
        };
        let fn__685 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__685())
        };
        test___6.assert(t___694, fn__685.clone());
        test___6.soft_fail_to_hard()
    }
    #[test]
    fn oppositeDirectionIsRejected__93() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___7 = temper_std::testing::Test::new();
        let game__59: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__60: snake::SnakeGame = snake::change_direction(game__59.clone(), snake::Direction::new(snake::Left::new()));
        let mut t___680: bool = temper_core::is::<snake::Right>(changed__60.direction());
        #[derive(Clone)]
        struct ClosureGroup___12 {}
        impl ClosureGroup___12 {
            fn fn__676(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be Right after trying Left".to_string());
            }
        }
        let closure_group = ClosureGroup___12 {};
        let fn__676 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__676())
        };
        test___7.assert(t___680, fn__676.clone());
        test___7.soft_fail_to_hard()
    }
    #[test]
    fn nonOppositeDirectionIsAccepted__94() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___8 = temper_std::testing::Test::new();
        let game__62: snake::SnakeGame = snake::new_game(10, 10, 42);
        let changed__63: snake::SnakeGame = snake::change_direction(game__62.clone(), snake::Direction::new(snake::Up::new()));
        let mut t___673: bool = temper_core::is::<snake::Up>(changed__63.direction());
        #[derive(Clone)]
        struct ClosureGroup___13 {}
        impl ClosureGroup___13 {
            fn fn__669(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should change to Up".to_string());
            }
        }
        let closure_group = ClosureGroup___13 {};
        let fn__669 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__669())
        };
        test___8.assert(t___673, fn__669.clone());
        test___8.soft_fail_to_hard()
    }
    #[test]
    fn wallCollisionCausesGameOver__95() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___9 = temper_std::testing::Test::new();
        let mut t___664: snake::SnakeGame;
        let mut t___663: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__65: snake::SnakeGame = t___663.clone();
        let mut i__66: i32 = 0;
        'loop___780: while Some(i__66) < Some(10) {
            t___664 = snake::tick(game__65.clone());
            game__65 = t___664.clone();
            i__66 = i__66.wrapping_add(1);
        }
        let mut t___666: bool = temper_core::is::<snake::GameOver>(game__65.status());
        #[derive(Clone)]
        struct ClosureGroup___14 {}
        impl ClosureGroup___14 {
            fn fn__662(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after hitting wall".to_string());
            }
        }
        let closure_group = ClosureGroup___14 {};
        let fn__662 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__662())
        };
        test___9.assert(t___666, fn__662.clone());
        test___9.soft_fail_to_hard()
    }
    #[test]
    fn selfCollisionCausesGameOver__96() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___10 = temper_std::testing::Test::new();
        let snake__68: temper_core::List<snake::Point> = std::sync::Arc::new(vec![snake::Point::new(5, 5), snake::Point::new(6, 5), snake::Point::new(6, 4), snake::Point::new(5, 4), snake::Point::new(4, 4), snake::Point::new(4, 5), snake::Point::new(4, 6)]);
        let mut t___656: snake::SnakeGame = snake::SnakeGame::new(10, 10, snake__68.clone(), snake::Direction::new(snake::Left::new()), snake::Point::new(0, 0), 0, snake::GameStatus::new(snake::Playing::new()), 42);
        let mut game__69: snake::SnakeGame = t___656.clone();
        let mut t___657: snake::SnakeGame = snake::tick(game__69.clone());
        game__69 = t___657.clone();
        let mut t___659: bool = temper_core::is::<snake::GameOver>(game__69.status());
        #[derive(Clone)]
        struct ClosureGroup___15 {}
        impl ClosureGroup___15 {
            fn fn__645(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver after self collision".to_string());
            }
        }
        let closure_group = ClosureGroup___15 {};
        let fn__645 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__645())
        };
        test___10.assert(t___659, fn__645.clone());
        test___10.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForSamePoints__97() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___11 = temper_std::testing::Test::new();
        let mut t___643: bool = snake::point_equals(snake::Point::new(3, 4), snake::Point::new(3, 4));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__639(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same points should be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__639 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__639())
        };
        test___11.assert(t___643, fn__639.clone());
        test___11.soft_fail_to_hard()
    }
    #[test]
    fn pointEqualsWorksForDifferentPoints__98() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___12 = temper_std::testing::Test::new();
        let mut t___637: bool = ! snake::point_equals(snake::Point::new(3, 4), snake::Point::new(5, 6));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__633(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("different points should not be equal".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__633())
        };
        test___12.assert(t___637, fn__633.clone());
        test___12.soft_fail_to_hard()
    }
    #[test]
    fn isOppositeDetectsOppositeDirections__99() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___13 = temper_std::testing::Test::new();
        let mut t___621: bool = snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Down::new()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__617(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Down are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__617 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__617())
        };
        test___13.assert(t___621, fn__617.clone());
        let mut t___626: bool = snake::is_opposite(snake::Direction::new(snake::Left::new()), snake::Direction::new(snake::Right::new()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__616(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Left/Right are opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__616 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__616())
        };
        test___13.assert(t___626, fn__616.clone());
        let mut t___631: bool = ! snake::is_opposite(snake::Direction::new(snake::Up::new()), snake::Direction::new(snake::Left::new()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__615(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up/Left are not opposite".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__615 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__615())
        };
        test___13.assert(t___631, fn__615.clone());
        test___13.soft_fail_to_hard()
    }
    #[test]
    fn directionDeltaReturnsCorrectDeltas__100() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___14 = temper_std::testing::Test::new();
        let mut t___607: i32;
        let mut t___612: i32;
        let mut t___279: bool;
        let mut t___284: bool;
        let up__74: snake::Point = snake::direction_delta(snake::Direction::new(snake::Up::new()));
        if Some(up__74.x()) == Some(0) {
            t___607 = up__74.y();
            t___279 = Some(t___607) == Some(-1);
        } else {
            t___279 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__604(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Up should be (0, -1)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__604 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__604())
        };
        test___14.assert(t___279, fn__604.clone());
        let right__75: snake::Point = snake::direction_delta(snake::Direction::new(snake::Right::new()));
        if Some(right__75.x()) == Some(1) {
            t___612 = right__75.y();
            t___284 = Some(t___612) == Some(0);
        } else {
            t___284 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__603(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("Right should be (1, 0)".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__603 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__603())
        };
        test___14.assert(t___284, fn__603.clone());
        test___14.soft_fail_to_hard()
    }
    #[test]
    fn prngIsDeterministic__101() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___17 = temper_std::testing::Test::new();
        let r1__77: snake::RandomResult = snake::next_random(42, 100);
        let r2__78: snake::RandomResult = snake::next_random(42, 100);
        let mut t___596: bool = Some(r1__77.value()) == Some(r2__78.value());
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__592(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same value".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__592 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__592())
        };
        test___17.assert(t___596, fn__592.clone());
        let mut t___601: bool = Some(r1__77.next_seed()) == Some(r2__78.next_seed());
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__591(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("same seed should produce same next seed".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__591 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__591())
        };
        test___17.assert(t___601, fn__591.clone());
        test___17.soft_fail_to_hard()
    }
    #[test]
    fn prngProducesValuesInRange__102() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___18 = temper_std::testing::Test::new();
        let mut t___588: i32;
        let mut t___269: bool;
        let r__80: snake::RandomResult = snake::next_random(42, 10);
        if Some(r__80.value()) >= Some(0) {
            t___588 = r__80.value();
            t___269 = Some(t___588) < Some(10);
        } else {
            t___269 = false;
        }
        #[derive(Clone)]
        struct ClosureGroup___25 {
            r__80: snake::RandomResult
        }
        impl ClosureGroup___25 {
            fn fn__586(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("value should be in [0, 10), got {}", self.r__80.value()));
            }
        }
        let closure_group = ClosureGroup___25 {
            r__80: r__80.clone()
        };
        let fn__586 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__586())
        };
        test___18.assert(t___269, fn__586.clone());
        test___18.soft_fail_to_hard()
    }
    #[test]
    fn tickDoesNothingWhenGameIsOver__103() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___20 = temper_std::testing::Test::new();
        let mut t___569: snake::SnakeGame;
        let mut t___568: snake::SnakeGame = snake::new_game(10, 10, 42);
        let mut game__82: snake::SnakeGame = t___568.clone();
        let mut i__83: i32 = 0;
        'loop___781: while Some(i__83) < Some(10) {
            t___569 = snake::tick(game__82.clone());
            game__82 = t___569.clone();
            i__83 = i__83.wrapping_add(1);
        }
        let mut t___571: bool = temper_core::is::<snake::GameOver>(game__82.status());
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__567(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be GameOver".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__567 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__567())
        };
        test___20.assert(t___571, fn__567.clone());
        let head1__84: snake::Point = temper_core::ListedTrait::get_or( & game__82.snake(), 0, snake::Point::new(-1, -1));
        let mut t___577: snake::SnakeGame = snake::tick(game__82.clone());
        game__82 = t___577.clone();
        let head2__85: snake::Point = temper_core::ListedTrait::get_or( & game__82.snake(), 0, snake::Point::new(-1, -1));
        let mut t___582: bool = snake::point_equals(head1__84.clone(), head2__85.clone());
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__566(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("snake should not move after game over".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__566 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__566())
        };
        test___20.assert(t___582, fn__566.clone());
        test___20.soft_fail_to_hard()
    }
    use super::*;
}
