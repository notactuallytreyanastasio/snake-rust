#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            {
                * INPUT_DIRECTION.write().unwrap() = Some(snake::Direction::new(snake::Right::new()));
            }
            crate::run_async(std::sync::Arc::new(fn__145.clone()).clone());
            crate::run_async(std::sync::Arc::new(fn__144.clone()).clone());
            Ok(())
    }).clone()
}
static INPUT_DIRECTION: std::sync::RwLock<Option<snake::Direction>> = std::sync::RwLock::new(None);
fn input_direction() -> snake::Direction {
    INPUT_DIRECTION.read().unwrap().clone().unwrap()
}
fn parseInput__6(line__21: impl temper_core::ToArcString) -> Option<snake::Direction> {
    let line__21 = line__21.to_arc_string();
    let return__5: Option<snake::Direction>;
    if Some(line__21.as_str()) == Some("w") {
        return__5 = Some(snake::Direction::new(snake::Up::new()));
    } else {
        if Some(line__21.as_str()) == Some("s") {
            return__5 = Some(snake::Direction::new(snake::Down::new()));
        } else {
            if Some(line__21.as_str()) == Some("a") {
                return__5 = Some(snake::Direction::new(snake::Left::new()));
            } else {
                if Some(line__21.as_str()) == Some("d") {
                    return__5 = Some(snake::Direction::new(snake::Right::new()));
                } else {
                    return__5 = None;
                }
            }
        }
    }
    return return__5.clone();
}
fn fn__145() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___155: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___135: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___136: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___78: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___82: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut line__24: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___154: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut dir__25: std::sync::Arc<std::sync::RwLock<Option<snake::Direction>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___1 {
        caseIndex___155: std::sync::Arc<std::sync::RwLock<i32>>, promise___154: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___78: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, line__24: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___135: std::sync::Arc<std::sync::RwLock<bool>>, t___82: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, dir__25: std::sync::Arc<std::sync::RwLock<Option<snake::Direction>>>, t___136: std::sync::Arc<std::sync::RwLock<bool>>
    }
    impl ClosureGroup___1 {
        fn convertedCoroutine___160(& self, generator___153: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___171: loop {
                let caseIndexLocal___156: i32 = temper_core::read_locked( & self.caseIndex___155);
                {
                    * self.caseIndex___155.write().unwrap() = -1;
                }
                match caseIndexLocal___156.clone() {
                    0 => {
                        {
                            * self.caseIndex___155.write().unwrap() = 1;
                        }
                    },
                    1 => {
                        {
                            * self.promise___154.write().unwrap() = Some(temper_std::io::std_read_line());
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 2;
                        }
                        temper_core::read_locked( & self.promise___154).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___153.clone().next();
                        }));
                        return Some(().clone());
                    },
                    2 => {
                        match temper_core::read_locked( & self.promise___154).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___78.write().unwrap() = x;
                                }
                                {
                                    * self.caseIndex___155.write().unwrap() = 3;
                                }
                            },
                            _ => {
                                * self.caseIndex___155.write().unwrap() = 8;
                            }
                        };
                    },
                    3 => {
                        {
                            * self.line__24.write().unwrap() = temper_core::read_locked( & self.t___78).clone();
                        }
                        if ! temper_core::read_locked( & self.line__24).is_none() {
                            {
                                * self.caseIndex___155.write().unwrap() = 4;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 5;
                            }
                        }
                    },
                    4 => {
                        {
                            * self.t___135.write().unwrap() = temper_core::read_locked( & self.line__24).clone().is_some();
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 6;
                        }
                    },
                    5 => {
                        {
                            * self.t___135.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 6;
                        }
                    },
                    6 => {
                        if temper_core::read_locked( & self.t___135) {
                            {
                                * self.caseIndex___155.write().unwrap() = 7;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 8;
                            }
                        }
                    },
                    7 => {
                        if temper_core::read_locked( & self.line__24).is_none() {
                            {
                                * self.caseIndex___155.write().unwrap() = 9;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 10;
                            }
                        }
                    },
                    8 => {
                        return None;
                    },
                    9 => {
                        {
                            * self.t___82.write().unwrap() = panic!();
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 11;
                        }
                    },
                    10 => {
                        {
                            * self.t___82.write().unwrap() = temper_core::read_locked( & self.line__24).clone().unwrap();
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 11;
                        }
                    },
                    11 => {
                        {
                            * self.dir__25.write().unwrap() = parseInput__6(temper_core::read_locked( & self.t___82).clone());
                        }
                        if ! temper_core::read_locked( & self.dir__25).is_none() {
                            {
                                * self.caseIndex___155.write().unwrap() = 12;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 13;
                            }
                        }
                    },
                    12 => {
                        {
                            * self.t___136.write().unwrap() = temper_core::read_locked( & self.dir__25).clone().is_some();
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 14;
                        }
                    },
                    13 => {
                        {
                            * self.t___136.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 14;
                        }
                    },
                    14 => {
                        if temper_core::read_locked( & self.t___136) {
                            {
                                * self.caseIndex___155.write().unwrap() = 15;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 16;
                            }
                        }
                    },
                    15 => {
                        if temper_core::read_locked( & self.dir__25).is_none() {
                            {
                                * self.caseIndex___155.write().unwrap() = 17;
                            }
                        } else {
                            {
                                * self.caseIndex___155.write().unwrap() = 18;
                            }
                        }
                    },
                    16 => {
                        {
                            * self.caseIndex___155.write().unwrap() = 1;
                        }
                    },
                    17 => {
                        {
                            * INPUT_DIRECTION.write().unwrap() = Some(temper_core::cast::<snake::Direction>(panic!()).unwrap());
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 16;
                        }
                    },
                    18 => {
                        {
                            * INPUT_DIRECTION.write().unwrap() = Some(temper_core::read_locked( & self.dir__25).clone().unwrap());
                        }
                        {
                            * self.caseIndex___155.write().unwrap() = 16;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
        }
    }
    let closure_group = ClosureGroup___1 {
        caseIndex___155: caseIndex___155.clone(), promise___154: promise___154.clone(), t___78: t___78.clone(), line__24: line__24.clone(), t___135: t___135.clone(), t___82: t___82.clone(), dir__25: dir__25.clone(), t___136: t___136.clone()
    };
    let convertedCoroutine___160 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___153: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___160(generator___153))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___160.clone().clone());
}
fn fn__144() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___165: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___121: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___124: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___125: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut t___127: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut t___130: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut promise___163: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut game__27: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___164: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___2 {
        caseIndex___165: std::sync::Arc<std::sync::RwLock<i32>>, promise___163: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, t___121: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>>, game__27: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>>, t___127: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___130: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___124: std::sync::Arc<std::sync::RwLock<Option<snake::SnakeGame>>>, t___125: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, promise___164: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
    }
    impl ClosureGroup___2 {
        fn convertedCoroutine___169(& self, generator___162: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___175: loop {
                let caseIndexLocal___166: i32 = temper_core::read_locked( & self.caseIndex___165);
                {
                    * self.caseIndex___165.write().unwrap() = -1;
                }
                match caseIndexLocal___166.clone() {
                    0 => {
                        println!("{}", "Snake! Use w/a/s/d + Enter to move.");
                        println!("{}", "Starting in 1 second...");
                        {
                            * self.promise___163.write().unwrap() = Some(temper_std::io::std_sleep(1000));
                        }
                        {
                            * self.caseIndex___165.write().unwrap() = 1;
                        }
                        temper_core::read_locked( & self.promise___163).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___162.clone().next();
                        }));
                        return Some(().clone());
                    },
                    1 => {
                        match temper_core::read_locked( & self.promise___163).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___165.write().unwrap() = 2;
                            },
                            _ => {
                                * self.caseIndex___165.write().unwrap() = 7;
                            }
                        };
                    },
                    2 => {
                        {
                            * self.t___121.write().unwrap() = Some(snake::new_game(20, 10, 42));
                        }
                        {
                            * self.game__27.write().unwrap() = Some(temper_core::read_locked( & self.t___121).clone().unwrap());
                        }
                        {
                            * self.caseIndex___165.write().unwrap() = 3;
                        }
                    },
                    3 => {
                        if ! temper_core::is::<snake::Playing>(temper_core::read_locked( & self.game__27).clone().unwrap().status()) {
                            {
                                * self.caseIndex___165.write().unwrap() = 4;
                            }
                        } else {
                            {
                                * self.caseIndex___165.write().unwrap() = 5;
                            }
                        }
                    },
                    4 => {
                        {
                            * self.t___127.write().unwrap() = snake::render(temper_core::read_locked( & self.game__27).clone().unwrap());
                        }
                        println!("{}", temper_core::read_locked( & self.t___127).clone());
                        {
                            * self.t___130.write().unwrap() = temper_core::int_to_string(temper_core::read_locked( & self.game__27).clone().unwrap().score(), None);
                        }
                        println!("Final score: {}", temper_core::read_locked( & self.t___130).clone());
                        {
                            * self.caseIndex___165.write().unwrap() = 7;
                        }
                    },
                    5 => {
                        {
                            * self.game__27.write().unwrap() = Some(snake::change_direction(temper_core::read_locked( & self.game__27).clone().unwrap(), input_direction().clone()));
                        }
                        {
                            * self.t___124.write().unwrap() = Some(snake::tick(temper_core::read_locked( & self.game__27).clone().unwrap()));
                        }
                        {
                            * self.game__27.write().unwrap() = Some(temper_core::read_locked( & self.t___124).clone().unwrap());
                        }
                        {
                            * self.t___125.write().unwrap() = snake::render(temper_core::read_locked( & self.game__27).clone().unwrap());
                        }
                        println!("{}", temper_core::read_locked( & self.t___125).clone());
                        {
                            * self.promise___164.write().unwrap() = Some(temper_std::io::std_sleep(200));
                        }
                        {
                            * self.caseIndex___165.write().unwrap() = 6;
                        }
                        temper_core::read_locked( & self.promise___164).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___162.clone().next();
                        }));
                        return Some(().clone());
                    },
                    6 => {
                        match temper_core::read_locked( & self.promise___164).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___165.write().unwrap() = 8;
                            },
                            _ => {
                                * self.caseIndex___165.write().unwrap() = 7;
                            }
                        };
                    },
                    7 => {
                        return None;
                    },
                    8 => {
                        {
                            * self.caseIndex___165.write().unwrap() = 3;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
        }
    }
    let closure_group = ClosureGroup___2 {
        caseIndex___165: caseIndex___165.clone(), promise___163: promise___163.clone(), t___121: t___121.clone(), game__27: game__27.clone(), t___127: t___127.clone(), t___130: t___130.clone(), t___124: t___124.clone(), t___125: t___125.clone(), promise___164: promise___164.clone()
    };
    let convertedCoroutine___169 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___162: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___169(generator___162))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___169.clone().clone());
}
