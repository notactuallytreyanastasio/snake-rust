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
            crate::run_async(std::sync::Arc::new(fn__102.clone()).clone());
            Ok(())
    }).clone()
}
static CONNECTED: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);
fn connected() -> bool {
    CONNECTED.read().unwrap().unwrap()
}
fn fn__102() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___111: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___96: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___61: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___64: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___66: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___108: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut msg__17: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___109: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___110: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___1 {
        caseIndex___111: std::sync::Arc<std::sync::RwLock<i32>>, promise___108: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>>, t___61: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, promise___109: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___64: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, msg__17: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___96: std::sync::Arc<std::sync::RwLock<bool>>, t___66: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, promise___110: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
    }
    impl ClosureGroup___1 {
        fn convertedCoroutine___127(& self, generator___107: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___128: loop {
                let caseIndexLocal___112: i32 = temper_core::read_locked( & self.caseIndex___111);
                {
                    * self.caseIndex___111.write().unwrap() = -1;
                }
                match caseIndexLocal___112.clone() {
                    0 => {
                        println!("{}", "Snake Multiplayer Client");
                        println!("{}", "Connecting to ws://localhost:8080...");
                        {
                            * self.promise___108.write().unwrap() = Some(temper_std::ws::ws_connect("ws://localhost:8080".clone()));
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 1;
                        }
                        temper_core::read_locked( & self.promise___108).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___107.clone().next();
                        }));
                        return Some(().clone());
                    },
                    1 => {
                        match temper_core::read_locked( & self.promise___108).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___61.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___111.write().unwrap() = 2;
                                }
                            },
                            _ => {
                                * self.caseIndex___111.write().unwrap() = 18;
                            }
                        };
                    },
                    2 => {
                        {
                            * self.ws__14.write().unwrap() = Some(temper_core::read_locked( & self.t___61).clone().unwrap());
                        }
                        println!("{}", "Connected! Use w/a/s/d to move.");
                        #[derive(Clone)]
                        struct ClosureGroup___2 {
                            ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>
                        }
                        impl ClosureGroup___2 {
                            fn fn__90(& self) -> temper_core::SafeGenerator<()> {
                                let mut caseIndex___122: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
                                let mut t___86: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
                                let mut t___49: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___59: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut line__16: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___117: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___118: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___119: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___120: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___121: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                #[derive(Clone)]
                                struct ClosureGroup___3 {
                                    caseIndex___122: std::sync::Arc<std::sync::RwLock<i32>>, promise___117: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___49: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, line__16: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___86: std::sync::Arc<std::sync::RwLock<bool>>, t___59: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, promise___118: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, ws__14: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, promise___119: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___120: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___121: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
                                }
                                impl ClosureGroup___3 {
                                    fn convertedCoroutine___125(& self, generator___116: temper_core::SafeGenerator<()>) -> Option<()> {
                                        'loop___129: loop {
                                            let caseIndexLocal___123: i32 = temper_core::read_locked( & self.caseIndex___122);
                                            {
                                                * self.caseIndex___122.write().unwrap() = -1;
                                            }
                                            match caseIndexLocal___123.clone() {
                                                0 => {
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 1;
                                                    }
                                                },
                                                1 => {
                                                    if connected() {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 2;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 9;
                                                        }
                                                    }
                                                },
                                                2 => {
                                                    {
                                                        * self.promise___117.write().unwrap() = Some(temper_std::io::std_read_line());
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 3;
                                                    }
                                                    temper_core::read_locked( & self.promise___117).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___116.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                3 => {
                                                    match temper_core::read_locked( & self.promise___117).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            {
                                                                * self.t___49.write().unwrap() = x;
                                                            }
                                                            {
                                                                * self.caseIndex___122.write().unwrap() = 4;
                                                            }
                                                        },
                                                        _ => {
                                                            * self.caseIndex___122.write().unwrap() = 9;
                                                        }
                                                    };
                                                },
                                                4 => {
                                                    {
                                                        * self.line__16.write().unwrap() = temper_core::read_locked( & self.t___49).clone();
                                                    }
                                                    if ! temper_core::read_locked( & self.line__16).is_none() {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 5;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 6;
                                                        }
                                                    }
                                                },
                                                5 => {
                                                    {
                                                        * self.t___86.write().unwrap() = temper_core::read_locked( & self.line__16).clone().is_some();
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 7;
                                                    }
                                                },
                                                6 => {
                                                    {
                                                        * self.t___86.write().unwrap() = false;
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 7;
                                                    }
                                                },
                                                7 => {
                                                    if temper_core::read_locked( & self.t___86) {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 8;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 9;
                                                        }
                                                    }
                                                },
                                                8 => {
                                                    if temper_core::read_locked( & self.line__16).is_none() {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 10;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 11;
                                                        }
                                                    }
                                                },
                                                9 => {
                                                    return None;
                                                },
                                                10 => {
                                                    {
                                                        * self.t___59.write().unwrap() = panic!();
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 12;
                                                    }
                                                },
                                                11 => {
                                                    {
                                                        * self.t___59.write().unwrap() = temper_core::read_locked( & self.line__16).clone().unwrap();
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 12;
                                                    }
                                                },
                                                12 => {
                                                    if Some(temper_core::read_locked( & self.t___59).as_str()) == Some("w") {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 13;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 14;
                                                        }
                                                    }
                                                },
                                                13 => {
                                                    {
                                                        * self.promise___118.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "u"));
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 15;
                                                    }
                                                    temper_core::read_locked( & self.promise___118).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___116.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                14 => {
                                                    if Some(temper_core::read_locked( & self.t___59).as_str()) == Some("s") {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 16;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 17;
                                                        }
                                                    }
                                                },
                                                15 => {
                                                    match temper_core::read_locked( & self.promise___118).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                16 => {
                                                    {
                                                        * self.promise___119.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "d"));
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 18;
                                                    }
                                                    temper_core::read_locked( & self.promise___119).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___116.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                17 => {
                                                    if Some(temper_core::read_locked( & self.t___59).as_str()) == Some("a") {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 19;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 20;
                                                        }
                                                    }
                                                },
                                                18 => {
                                                    match temper_core::read_locked( & self.promise___119).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                19 => {
                                                    {
                                                        * self.promise___120.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "l"));
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 21;
                                                    }
                                                    temper_core::read_locked( & self.promise___120).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___116.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                20 => {
                                                    if Some(temper_core::read_locked( & self.t___59).as_str()) == Some("d") {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 22;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        }
                                                    }
                                                },
                                                21 => {
                                                    match temper_core::read_locked( & self.promise___120).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                22 => {
                                                    {
                                                        * self.promise___121.write().unwrap() = Some(temper_std::ws::ws_send(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone(), "r"));
                                                    }
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 23;
                                                    }
                                                    temper_core::read_locked( & self.promise___121).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___116.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                23 => {
                                                    match temper_core::read_locked( & self.promise___121).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        },
                                                        _ => {
                                                            * self.caseIndex___122.write().unwrap() = 24;
                                                        }
                                                    };
                                                },
                                                24 => {
                                                    {
                                                        * self.caseIndex___122.write().unwrap() = 1;
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
                                    caseIndex___122: caseIndex___122.clone(), promise___117: promise___117.clone(), t___49: t___49.clone(), line__16: line__16.clone(), t___86: t___86.clone(), t___59: t___59.clone(), promise___118: promise___118.clone(), ws__14: self.ws__14.clone(), promise___119: promise___119.clone(), promise___120: promise___120.clone(), promise___121: promise___121.clone()
                                };
                                let convertedCoroutine___125 = {
                                    let closure_group = closure_group.clone();
                                    std::sync::Arc::new(move | generator___116: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___125(generator___116))
                                };
                                return temper_core::SafeGenerator::from_fn(convertedCoroutine___125.clone().clone());
                            }
                        }
                        let closure_group = ClosureGroup___2 {
                            ws__14: self.ws__14.clone()
                        };
                        let fn__90 = {
                            let closure_group = closure_group.clone();
                            std::sync::Arc::new(move | | closure_group.fn__90())
                        };
                        crate::run_async(fn__90.clone().clone());
                        {
                            * self.caseIndex___111.write().unwrap() = 3;
                        }
                    },
                    3 => {
                        if connected() {
                            {
                                * self.caseIndex___111.write().unwrap() = 4;
                            }
                        } else {
                            {
                                * self.caseIndex___111.write().unwrap() = 16;
                            }
                        }
                    },
                    4 => {
                        {
                            * self.promise___109.write().unwrap() = Some(temper_std::ws::ws_recv(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 5;
                        }
                        temper_core::read_locked( & self.promise___109).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___107.clone().next();
                        }));
                        return Some(().clone());
                    },
                    5 => {
                        match temper_core::read_locked( & self.promise___109).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___64.write().unwrap() = x;
                                }
                                {
                                    * self.caseIndex___111.write().unwrap() = 6;
                                }
                            },
                            _ => {
                                * self.caseIndex___111.write().unwrap() = 18;
                            }
                        };
                    },
                    6 => {
                        {
                            * self.msg__17.write().unwrap() = temper_core::read_locked( & self.t___64).clone();
                        }
                        if ! temper_core::read_locked( & self.msg__17).is_none() {
                            {
                                * self.caseIndex___111.write().unwrap() = 7;
                            }
                        } else {
                            {
                                * self.caseIndex___111.write().unwrap() = 8;
                            }
                        }
                    },
                    7 => {
                        {
                            * self.t___96.write().unwrap() = temper_core::read_locked( & self.msg__17).clone().is_some();
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 9;
                        }
                    },
                    8 => {
                        {
                            * self.t___96.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 9;
                        }
                    },
                    9 => {
                        if temper_core::read_locked( & self.t___96) {
                            {
                                * self.caseIndex___111.write().unwrap() = 10;
                            }
                        } else {
                            {
                                * self.caseIndex___111.write().unwrap() = 11;
                            }
                        }
                    },
                    10 => {
                        if temper_core::read_locked( & self.msg__17).is_none() {
                            {
                                * self.caseIndex___111.write().unwrap() = 12;
                            }
                        } else {
                            {
                                * self.caseIndex___111.write().unwrap() = 13;
                            }
                        }
                    },
                    11 => {
                        println!("{}", "Disconnected from server.");
                        {
                            * CONNECTED.write().unwrap() = Some(false);
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 15;
                        }
                    },
                    12 => {
                        {
                            * self.t___66.write().unwrap() = panic!();
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 14;
                        }
                    },
                    13 => {
                        {
                            * self.t___66.write().unwrap() = temper_core::read_locked( & self.msg__17).clone().unwrap();
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 14;
                        }
                    },
                    14 => {
                        println!("{}", temper_core::read_locked( & self.t___66).clone());
                        {
                            * self.caseIndex___111.write().unwrap() = 15;
                        }
                    },
                    15 => {
                        {
                            * self.caseIndex___111.write().unwrap() = 3;
                        }
                    },
                    16 => {
                        {
                            * self.promise___110.write().unwrap() = Some(temper_std::ws::ws_close(( * temper_core::read_locked( & self.ws__14).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___111.write().unwrap() = 17;
                        }
                        temper_core::read_locked( & self.promise___110).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___107.clone().next();
                        }));
                        return Some(().clone());
                    },
                    17 => {
                        match temper_core::read_locked( & self.promise___110).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___111.write().unwrap() = 18;
                            },
                            _ => {
                                * self.caseIndex___111.write().unwrap() = 18;
                            }
                        };
                    },
                    18 => {
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
        caseIndex___111: caseIndex___111.clone(), promise___108: promise___108.clone(), t___61: t___61.clone(), ws__14: ws__14.clone(), promise___109: promise___109.clone(), t___64: t___64.clone(), msg__17: msg__17.clone(), t___96: t___96.clone(), t___66: t___66.clone(), promise___110: promise___110.clone()
    };
    let convertedCoroutine___127 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___107: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___127(generator___107))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___127.clone().clone());
}
