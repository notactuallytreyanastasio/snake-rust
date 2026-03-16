#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            {
                * CONNECTED.write().unwrap() = Some(true);
            }
            let serverUrl__12: std::sync::Arc<String> = std::sync::Arc::new("ws://localhost:8080".to_string());
            crate::run_async(std::sync::Arc::new(fn__107.clone()).clone());
            Ok(())
    }).clone()
}
static CONNECTED: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);
fn connected() -> bool {
    CONNECTED.read().unwrap().unwrap()
}
fn fn__107() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___117: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___101: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___63: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___69: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___71: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___113: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___114: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut msg__17: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___115: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___116: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___1 {
        caseIndex___117: std::sync::Arc<std::sync::RwLock<i32>>, promise___113: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>>, t___63: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, promise___114: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___115: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___69: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, msg__17: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___101: std::sync::Arc<std::sync::RwLock<bool>>, t___71: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, promise___116: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
    }
    impl ClosureGroup___1 {
        fn convertedCoroutine___133(& self, generator___112: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___134: loop {
                let caseIndexLocal___118: i32 = temper_core::read_locked( & self.caseIndex___117);
                {
                    * self.caseIndex___117.write().unwrap() = -1;
                }
                match caseIndexLocal___118.clone() {
                    0 => {
                        println!("{}", "Snake Multiplayer Client");
                        println!("{}", "Connecting to ws://localhost:8080...");
                        {
                            * self.promise___113.write().unwrap() = Some(temper_std::ws::ws_connect("ws://localhost:8080".clone()));
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 1;
                        }
                        temper_core::read_locked( & self.promise___113).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___112.clone().next();
                        }));
                        return Some(().clone());
                    },
                    1 => {
                        match temper_core::read_locked( & self.promise___113).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___63.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___117.write().unwrap() = 2;
                                }
                            },
                            _ => {
                                * self.caseIndex___117.write().unwrap() = 20;
                            }
                        };
                    },
                    2 => {
                        {
                            * self.ws__14.write().unwrap() = Some(temper_core::read_locked( & self.t___63).clone().unwrap());
                        }
                        {
                            * self.promise___114.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "join"));
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 3;
                        }
                        temper_core::read_locked( & self.promise___114).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___112.clone().next();
                        }));
                        return Some(().clone());
                    },
                    3 => {
                        match temper_core::read_locked( & self.promise___114).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___117.write().unwrap() = 4;
                            },
                            _ => {
                                * self.caseIndex___117.write().unwrap() = 4;
                            }
                        };
                    },
                    4 => {
                        println!("{}", "Connected! Use w/a/s/d to move.");
                        #[derive(Clone)]
                        struct ClosureGroup___2 {
                            ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>
                        }
                        impl ClosureGroup___2 {
                            fn fn__95(& self) -> temper_core::SafeGenerator<()> {
                                let mut caseIndex___128: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
                                let mut t___91: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
                                let mut t___51: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___61: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut line__16: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___123: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___124: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___125: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___126: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___127: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                #[derive(Clone)]
                                struct ClosureGroup___3 {
                                    caseIndex___128: std::sync::Arc<std::sync::RwLock<i32>>, promise___123: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___51: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, line__16: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___91: std::sync::Arc<std::sync::RwLock<bool>>, t___61: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, promise___124: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, promise___125: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___126: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___127: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
                                }
                                impl ClosureGroup___3 {
                                    fn convertedCoroutine___131(& self, generator___122: temper_core::SafeGenerator<()>) -> Option<()> {
                                        'loop___135: loop {
                                            let caseIndexLocal___129: i32 = temper_core::read_locked( & self.caseIndex___128);
                                            {
                                                * self.caseIndex___128.write().unwrap() = -1;
                                            }
                                            match caseIndexLocal___129.clone() {
                                                0 => {
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 1;
                                                    }
                                                },
                                                1 => {
                                                    if connected() {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 2;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 9;
                                                        }
                                                    }
                                                },
                                                2 => {
                                                    {
                                                        * self.promise___123.write().unwrap() = Some(temper_std::io::std_read_line());
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 3;
                                                    }
                                                    temper_core::read_locked( & self.promise___123).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___122.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                3 => {
                                                    match temper_core::read_locked( & self.promise___123).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            {
                                                                * self.t___51.write().unwrap() = x;
                                                            }
                                                            {
                                                                * self.caseIndex___128.write().unwrap() = 4;
                                                            }
                                                        },
                                                        _ => {
                                                            * self.caseIndex___128.write().unwrap() = 9;
                                                        }
                                                    };
                                                },
                                                4 => {
                                                    {
                                                        * self.line__16.write().unwrap() = temper_core::read_locked( & self.t___51).clone();
                                                    }
                                                    if ! temper_core::read_locked( & self.line__16).is_none() {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 5;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 6;
                                                        }
                                                    }
                                                },
                                                5 => {
                                                    {
                                                        * self.t___91.write().unwrap() = temper_core::read_locked( & self.line__16).clone().is_some();
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 7;
                                                    }
                                                },
                                                6 => {
                                                    {
                                                        * self.t___91.write().unwrap() = false;
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 7;
                                                    }
                                                },
                                                7 => {
                                                    if temper_core::read_locked( & self.t___91) {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 8;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 9;
                                                        }
                                                    }
                                                },
                                                8 => {
                                                    if temper_core::read_locked( & self.line__16).is_none() {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 10;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 11;
                                                        }
                                                    }
                                                },
                                                9 => {
                                                    return None;
                                                },
                                                10 => {
                                                    {
                                                        * self.t___61.write().unwrap() = panic!();
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 12;
                                                    }
                                                },
                                                11 => {
                                                    {
                                                        * self.t___61.write().unwrap() = temper_core::read_locked( & self.line__16).clone().unwrap();
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 12;
                                                    }
                                                },
                                                12 => {
                                                    if Some(temper_core::read_locked( & self.t___61).as_str()) == Some("w") {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 13;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 14;
                                                        }
                                                    }
                                                },
                                                13 => {
                                                    {
                                                        * self.promise___124.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "u"));
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 15;
                                                    }
                                                    temper_core::read_locked( & self.promise___124).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___122.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                14 => {
                                                    if Some(temper_core::read_locked( & self.t___61).as_str()) == Some("s") {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 16;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 17;
                                                        }
                                                    }
                                                },
                                                15 => {
                                                    match temper_core::read_locked( & self.promise___124).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                16 => {
                                                    {
                                                        * self.promise___125.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "d"));
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 18;
                                                    }
                                                    temper_core::read_locked( & self.promise___125).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___122.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                17 => {
                                                    if Some(temper_core::read_locked( & self.t___61).as_str()) == Some("a") {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 19;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 20;
                                                        }
                                                    }
                                                },
                                                18 => {
                                                    match temper_core::read_locked( & self.promise___125).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                19 => {
                                                    {
                                                        * self.promise___126.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "l"));
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 21;
                                                    }
                                                    temper_core::read_locked( & self.promise___126).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___122.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                20 => {
                                                    if Some(temper_core::read_locked( & self.t___61).as_str()) == Some("d") {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 22;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        }
                                                    }
                                                },
                                                21 => {
                                                    match temper_core::read_locked( & self.promise___126).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                22 => {
                                                    {
                                                        * self.promise___127.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "r"));
                                                    }
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 23;
                                                    }
                                                    temper_core::read_locked( & self.promise___127).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___122.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                23 => {
                                                    match temper_core::read_locked( & self.promise___127).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___128.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                24 => {
                                                    {
                                                        * self.caseIndex___128.write().unwrap() = 1;
                                                    }
                                                },
                                                _ => {
                                                    return None;
                                                }
                                            }
                                        }
                                    }
                                }
                                let closure_group = ClosureGroup___3 {
                                    caseIndex___128: caseIndex___128.clone(), promise___123: promise___123.clone(), t___51: t___51.clone(), line__16: line__16.clone(), t___91: t___91.clone(), t___61: t___61.clone(), promise___124: promise___124.clone(), ws__14: self.ws__14.clone(), promise___125: promise___125.clone(), promise___126: promise___126.clone(), promise___127: promise___127.clone()
                                };
                                let convertedCoroutine___131 = {
                                    let closure_group = closure_group.clone();
                                    std::sync::Arc::new(move | generator___122: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___131(generator___122))
                                };
                                return temper_core::SafeGenerator::from_fn(convertedCoroutine___131.clone().clone());
                            }
                        }
                        let closure_group = ClosureGroup___2 {
                            ws__14: self.ws__14.clone()
                        };
                        let fn__95 = {
                            let closure_group = closure_group.clone();
                            std::sync::Arc::new(move | | closure_group.fn__95())
                        };
                        crate::run_async(fn__95.clone().clone());
                        {
                            * self.caseIndex___117.write().unwrap() = 5;
                        }
                    },
                    5 => {
                        if connected() {
                            {
                                * self.caseIndex___117.write().unwrap() = 6;
                            }
                        } else {
                            {
                                * self.caseIndex___117.write().unwrap() = 18;
                            }
                        }
                    },
                    6 => {
                        {
                            * self.promise___115.write().unwrap() = Some(temper_std::ws::ws_recv(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 7;
                        }
                        temper_core::read_locked( & self.promise___115).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___112.clone().next();
                        }));
                        return Some(().clone());
                    },
                    7 => {
                        match temper_core::read_locked( & self.promise___115).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___69.write().unwrap() = x;
                                }
                                {
                                    * self.caseIndex___117.write().unwrap() = 8;
                                }
                            },
                            _ => {
                                * self.caseIndex___117.write().unwrap() = 20;
                            }
                        };
                    },
                    8 => {
                        {
                            * self.msg__17.write().unwrap() = temper_core::read_locked( & self.t___69).clone();
                        }
                        if ! temper_core::read_locked( & self.msg__17).is_none() {
                            {
                                * self.caseIndex___117.write().unwrap() = 9;
                            }
                        } else {
                            {
                                * self.caseIndex___117.write().unwrap() = 10;
                            }
                        }
                    },
                    9 => {
                        {
                            * self.t___101.write().unwrap() = temper_core::read_locked( & self.msg__17).clone().is_some();
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 11;
                        }
                    },
                    10 => {
                        {
                            * self.t___101.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 11;
                        }
                    },
                    11 => {
                        if temper_core::read_locked( & self.t___101) {
                            {
                                * self.caseIndex___117.write().unwrap() = 12;
                            }
                        } else {
                            {
                                * self.caseIndex___117.write().unwrap() = 13;
                            }
                        }
                    },
                    12 => {
                        if temper_core::read_locked( & self.msg__17).is_none() {
                            {
                                * self.caseIndex___117.write().unwrap() = 14;
                            }
                        } else {
                            {
                                * self.caseIndex___117.write().unwrap() = 15;
                            }
                        }
                    },
                    13 => {
                        println!("{}", "Disconnected from server.");
                        {
                            * CONNECTED.write().unwrap() = Some(false);
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 17;
                        }
                    },
                    14 => {
                        {
                            * self.t___71.write().unwrap() = panic!();
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 16;
                        }
                    },
                    15 => {
                        {
                            * self.t___71.write().unwrap() = temper_core::read_locked( & self.msg__17).clone().unwrap();
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 16;
                        }
                    },
                    16 => {
                        println!("{}", temper_core::read_locked( & self.t___71).clone());
                        {
                            * self.caseIndex___117.write().unwrap() = 17;
                        }
                    },
                    17 => {
                        {
                            * self.caseIndex___117.write().unwrap() = 5;
                        }
                    },
                    18 => {
                        {
                            * self.promise___116.write().unwrap() = Some(temper_std::ws::ws_close(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___117.write().unwrap() = 19;
                        }
                        temper_core::read_locked( & self.promise___116).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___112.clone().next();
                        }));
                        return Some(().clone());
                    },
                    19 => {
                        match temper_core::read_locked( & self.promise___116).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___117.write().unwrap() = 20;
                            },
                            _ => {
                                * self.caseIndex___117.write().unwrap() = 20;
                            }
                        };
                    },
                    20 => {
                        return None;
                    },
                    _ => {
                        return None;
                    }
                }
            }
        }
    }
    let closure_group = ClosureGroup___1 {
        caseIndex___117: caseIndex___117.clone(), promise___113: promise___113.clone(), t___63: t___63.clone(), ws__14: ws__14.clone(), promise___114: promise___114.clone(), promise___115: promise___115.clone(), t___69: t___69.clone(), msg__17: msg__17.clone(), t___101: t___101.clone(), t___71: t___71.clone(), promise___116: promise___116.clone()
    };
    let convertedCoroutine___133 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___112: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___133(generator___112))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___133.clone().clone());
}
