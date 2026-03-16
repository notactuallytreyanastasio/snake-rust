#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            let mut detectedCols__35: i32 = temper_std::io::std_term_cols();
            let mut detectedRows__36: i32 = temper_std::io::std_term_rows();
            let boardWidth__37: i32;
            if Some(detectedCols__35) > Some(100) {
                boardWidth__37 = detectedCols__35.wrapping_sub(4);
            } else {
                boardWidth__37 = 80;
            }
            let boardHeight__38: i32;
            if Some(detectedRows__36) > Some(30) {
                boardHeight__38 = detectedRows__36.wrapping_sub(12);
            } else {
                boardHeight__38 = 30;
            }
            {
                * GAME.write().unwrap() = Some(snake::new_multi_game(boardWidth__37, boardHeight__38, 0, 42));
            }
            {
                * WS_CONNS.write().unwrap() = Some(temper_core::listed::new_builder());
            }
            {
                * NEXT_ID.write().unwrap() = Some(0);
            }
            {
                * RUNNING.write().unwrap() = Some(true);
            }
            crate::run_async(std::sync::Arc::new(fn__330.clone()).clone());
            crate::run_async(std::sync::Arc::new(fn__329.clone()).clone());
            Ok(())
    }).clone()
}
static GAME: std::sync::RwLock<Option<snake::MultiSnakeGame>> = std::sync::RwLock::new(None);
fn game() -> snake::MultiSnakeGame {
    GAME.read().unwrap().clone().unwrap()
}
static WS_CONNS: std::sync::RwLock<Option<temper_core::ListBuilder<temper_std::ws::WsConnection>>> = std::sync::RwLock::new(None);
fn ws_conns() -> temper_core::ListBuilder<temper_std::ws::WsConnection> {
    WS_CONNS.read().unwrap().clone().unwrap()
}
static NEXT_ID: std::sync::RwLock<Option<i32>> = std::sync::RwLock::new(None);
fn next_id() -> i32 {
    NEXT_ID.read().unwrap().unwrap()
}
static RUNNING: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);
fn running() -> bool {
    RUNNING.read().unwrap().unwrap()
}
fn fn__330() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___343: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___311: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___312: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::PlayerSnake>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___316: std::sync::Arc<std::sync::RwLock<Option<snake::PlayerSnake>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___320: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::Direction>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___321: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___341: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut dirs__55: std::sync::Arc<std::sync::RwLock<Option<temper_core::ListBuilder<snake::Direction>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut i__56: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut promise___342: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___1 {
        caseIndex___343: std::sync::Arc<std::sync::RwLock<i32>>, promise___341: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, dirs__55: std::sync::Arc<std::sync::RwLock<Option<temper_core::ListBuilder<snake::Direction>>>>, t___311: std::sync::Arc<std::sync::RwLock<i32>>, i__56: std::sync::Arc<std::sync::RwLock<i32>>, t___320: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::Direction>>>>, t___321: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, promise___342: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, t___312: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::PlayerSnake>>>>, t___316: std::sync::Arc<std::sync::RwLock<Option<snake::PlayerSnake>>>
    }
    impl ClosureGroup___1 {
        fn convertedCoroutine___354(& self, generator___340: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___369: loop {
                let caseIndexLocal___344: i32 = temper_core::read_locked( & self.caseIndex___343);
                {
                    * self.caseIndex___343.write().unwrap() = -1;
                }
                match caseIndexLocal___344.clone() {
                    0 => {
                        {
                            * self.caseIndex___343.write().unwrap() = 1;
                        }
                    },
                    1 => {
                        if ! (Some(temper_core::ListedTrait::len( & game().snakes())) == Some(0)) {
                            {
                                * self.caseIndex___343.write().unwrap() = 2;
                            }
                        } else {
                            {
                                * self.caseIndex___343.write().unwrap() = 3;
                            }
                        }
                    },
                    2 => {
                        println!("{}", "Game starting!");
                        {
                            * self.caseIndex___343.write().unwrap() = 6;
                        }
                    },
                    3 => {
                        {
                            * self.promise___341.write().unwrap() = Some(temper_std::io::std_sleep(500));
                        }
                        {
                            * self.caseIndex___343.write().unwrap() = 4;
                        }
                        temper_core::read_locked( & self.promise___341).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___340.clone().next();
                        }));
                        return Some(().clone());
                    },
                    4 => {
                        match temper_core::read_locked( & self.promise___341).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___343.write().unwrap() = 5;
                            },
                            _ => {
                                * self.caseIndex___343.write().unwrap() = 12;
                            }
                        };
                    },
                    5 => {
                        {
                            * self.caseIndex___343.write().unwrap() = 1;
                        }
                    },
                    6 => {
                        if running() {
                            {
                                * self.caseIndex___343.write().unwrap() = 7;
                            }
                        } else {
                            {
                                * self.caseIndex___343.write().unwrap() = 12;
                            }
                        }
                    },
                    7 => {
                        {
                            * self.dirs__55.write().unwrap() = Some(temper_core::listed::new_builder());
                        }
                        {
                            * self.caseIndex___343.write().unwrap() = 8;
                        }
                    },
                    8 => {
                        {
                            * self.t___311.write().unwrap() = temper_core::ListedTrait::len( & game().snakes());
                        }
                        if ! (Some(temper_core::read_locked( & self.i__56)) < Some(temper_core::read_locked( & self.t___311))) {
                            {
                                * self.caseIndex___343.write().unwrap() = 9;
                            }
                        } else {
                            {
                                * self.caseIndex___343.write().unwrap() = 10;
                            }
                        }
                    },
                    9 => {
                        {
                            * self.t___320.write().unwrap() = Some(temper_core::ListedTrait::to_list( & temper_core::read_locked( & self.dirs__55).clone().unwrap()));
                        }
                        {
                            * self.t___321.write().unwrap() = Some(snake::multi_tick(game().clone(), temper_core::read_locked( & self.t___320).clone().unwrap()));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___321).clone().unwrap());
                        }
                        let frame__58: std::sync::Arc<String> = snake::multi_render(game().clone());
                        let conns__59: temper_core::List<temper_std::ws::WsConnection> = temper_core::ListedTrait::to_list( & ws_conns());
                        #[derive(Clone)]
                        struct ClosureGroup___2 {
                            frame__58: std::sync::Arc<String>
                        }
                        impl ClosureGroup___2 {
                            fn fn__305(& self, conn__60: temper_std::ws::WsConnection) {
                                temper_std::ws::ws_send(( * conn__60.clone()).clone(), self.frame__58.clone());
                            }
                        }
                        let closure_group = ClosureGroup___2 {
                            frame__58: frame__58.clone()
                        };
                        let fn__305 = {
                            let closure_group = closure_group.clone();
                            std::sync::Arc::new(move | conn__60: temper_std::ws::WsConnection | closure_group.fn__305(conn__60))
                        };
                        temper_core::listed::list_for_each( & conns__59, & ( * fn__305.clone()));
                        {
                            * self.promise___342.write().unwrap() = Some(temper_std::io::std_sleep(200));
                        }
                        {
                            * self.caseIndex___343.write().unwrap() = 11;
                        }
                        temper_core::read_locked( & self.promise___342).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___340.clone().next();
                        }));
                        return Some(().clone());
                    },
                    10 => {
                        {
                            * self.t___312.write().unwrap() = Some(game().snakes());
                        }
                        {
                            * self.t___316.write().unwrap() = Some(snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
                        }
                        let snake__57: snake::PlayerSnake = temper_core::ListedTrait::get_or( & temper_core::read_locked( & self.t___312).clone().unwrap(), temper_core::read_locked( & self.i__56), temper_core::read_locked( & self.t___316).clone().unwrap());
                        temper_core::listed::add( & temper_core::read_locked( & self.dirs__55).clone().unwrap(), snake__57.direction(), None);
                        {
                            * self.i__56.write().unwrap() = temper_core::read_locked( & self.i__56).wrapping_add(1);
                        }
                        {
                            * self.caseIndex___343.write().unwrap() = 8;
                        }
                    },
                    11 => {
                        match temper_core::read_locked( & self.promise___342).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___343.write().unwrap() = 13;
                            },
                            _ => {
                                * self.caseIndex___343.write().unwrap() = 12;
                            }
                        };
                    },
                    12 => {
                        return None;
                    },
                    13 => {
                        {
                            * self.caseIndex___343.write().unwrap() = 6;
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
        caseIndex___343: caseIndex___343.clone(), promise___341: promise___341.clone(), dirs__55: dirs__55.clone(), t___311: t___311.clone(), i__56: i__56.clone(), t___320: t___320.clone(), t___321: t___321.clone(), promise___342: promise___342.clone(), t___312: t___312.clone(), t___316: t___316.clone()
    };
    let convertedCoroutine___354 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___340: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___354(generator___340))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___354.clone().clone());
}
fn fn__329() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___360: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___284: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___287: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___289: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut t___291: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut t___292: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___293: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___294: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___295: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___296: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___297: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___298: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___299: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___151: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___152: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___154: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___156: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut t___171: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut server__44: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsServer>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___357: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsServer>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut ws__45: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___358: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut firstMsgRaw__46: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___359: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut isSpectator__47: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
    let mut playerId__48: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    #[derive(Clone)]
    struct ClosureGroup___3 {
        caseIndex___360: std::sync::Arc<std::sync::RwLock<i32>>, promise___357: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsServer>>>>, server__44: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsServer>>>, promise___358: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>>, t___151: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, ws__45: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, promise___359: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, t___152: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___154: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, firstMsgRaw__46: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___284: std::sync::Arc<std::sync::RwLock<bool>>, t___156: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, isSpectator__47: std::sync::Arc<std::sync::RwLock<bool>>, playerId__48: std::sync::Arc<std::sync::RwLock<i32>>, t___287: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___289: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___291: std::sync::Arc<std::sync::RwLock<bool>>, t___171: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___292: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>>, t___293: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___294: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>>, t___295: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___296: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>>, t___297: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___298: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>>, t___299: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>
    }
    impl ClosureGroup___3 {
        fn convertedCoroutine___368(& self, generator___356: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___370: loop {
                let caseIndexLocal___361: i32 = temper_core::read_locked( & self.caseIndex___360);
                {
                    * self.caseIndex___360.write().unwrap() = -1;
                }
                match caseIndexLocal___361.clone() {
                    0 => {
                        println!("{}", "Snake Multiplayer Server");
                        println!("{}", "Starting on port 8080...");
                        {
                            * self.promise___357.write().unwrap() = Some(temper_std::ws::ws_listen(8080));
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 1;
                        }
                        temper_core::read_locked( & self.promise___357).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___356.clone().next();
                        }));
                        return Some(().clone());
                    },
                    1 => {
                        match temper_core::read_locked( & self.promise___357).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.server__44.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___360.write().unwrap() = 2;
                                }
                            },
                            _ => {
                                * self.caseIndex___360.write().unwrap() = 6;
                            }
                        };
                    },
                    2 => {
                        println!("{}", "Listening on ws://localhost:8080");
                        println!("{}", "Waiting for players to connect...");
                        {
                            * self.caseIndex___360.write().unwrap() = 3;
                        }
                    },
                    3 => {
                        if running() {
                            {
                                * self.caseIndex___360.write().unwrap() = 4;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 6;
                            }
                        }
                    },
                    4 => {
                        {
                            * self.promise___358.write().unwrap() = Some(temper_std::ws::ws_accept(( * temper_core::read_locked( & self.server__44).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 5;
                        }
                        temper_core::read_locked( & self.promise___358).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___356.clone().next();
                        }));
                        return Some(().clone());
                    },
                    5 => {
                        match temper_core::read_locked( & self.promise___358).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___151.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___360.write().unwrap() = 7;
                                }
                            },
                            _ => {
                                * self.caseIndex___360.write().unwrap() = 6;
                            }
                        };
                    },
                    6 => {
                        return None;
                    },
                    7 => {
                        {
                            * self.ws__45.write().unwrap() = Some(temper_core::read_locked( & self.t___151).clone().unwrap());
                        }
                        {
                            * self.promise___359.write().unwrap() = Some(temper_std::ws::ws_recv(( * temper_core::read_locked( & self.ws__45).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 8;
                        }
                        temper_core::read_locked( & self.promise___359).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___356.clone().next();
                        }));
                        return Some(().clone());
                    },
                    8 => {
                        match temper_core::read_locked( & self.promise___359).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___152.write().unwrap() = x;
                                }
                                {
                                    * self.caseIndex___360.write().unwrap() = 10;
                                }
                            },
                            _ => {
                                * self.caseIndex___360.write().unwrap() = 9;
                            }
                        };
                    },
                    9 => {
                        {
                            * self.t___154.write().unwrap() = None;
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 11;
                        }
                    },
                    10 => {
                        {
                            * self.t___154.write().unwrap() = temper_core::read_locked( & self.t___152).clone();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 11;
                        }
                    },
                    11 => {
                        {
                            * self.firstMsgRaw__46.write().unwrap() = temper_core::read_locked( & self.t___154).clone();
                        }
                        if ! temper_core::read_locked( & self.firstMsgRaw__46).is_none() {
                            {
                                * self.caseIndex___360.write().unwrap() = 12;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 13;
                            }
                        }
                    },
                    12 => {
                        {
                            * self.t___284.write().unwrap() = temper_core::read_locked( & self.firstMsgRaw__46).clone().is_some();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 14;
                        }
                    },
                    13 => {
                        {
                            * self.t___284.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 14;
                        }
                    },
                    14 => {
                        if temper_core::read_locked( & self.t___284) {
                            {
                                * self.caseIndex___360.write().unwrap() = 15;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 20;
                            }
                        }
                    },
                    15 => {
                        if temper_core::read_locked( & self.firstMsgRaw__46).is_none() {
                            {
                                * self.caseIndex___360.write().unwrap() = 16;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 17;
                            }
                        }
                    },
                    16 => {
                        {
                            * self.t___156.write().unwrap() = panic!();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 18;
                        }
                    },
                    17 => {
                        {
                            * self.t___156.write().unwrap() = temper_core::read_locked( & self.firstMsgRaw__46).clone().unwrap();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 18;
                        }
                    },
                    18 => {
                        if Some(temper_core::read_locked( & self.t___156).as_str()) == Some("spectate") {
                            {
                                * self.caseIndex___360.write().unwrap() = 19;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 20;
                            }
                        }
                    },
                    19 => {
                        {
                            * self.isSpectator__47.write().unwrap() = true;
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 20;
                        }
                    },
                    20 => {
                        if temper_core::read_locked( & self.isSpectator__47) {
                            {
                                * self.caseIndex___360.write().unwrap() = 21;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 22;
                            }
                        }
                    },
                    21 => {
                        temper_core::listed::add( & ws_conns(), temper_core::read_locked( & self.ws__45).clone().unwrap(), None);
                        println!("{}", "Spectator connected!");
                        {
                            * self.caseIndex___360.write().unwrap() = 38;
                        }
                    },
                    22 => {
                        {
                            * self.playerId__48.write().unwrap() = next_id();
                        }
                        {
                            * NEXT_ID.write().unwrap() = Some(next_id().wrapping_add(1));
                        }
                        {
                            * self.t___287.write().unwrap() = Some(snake::add_player(game().clone(), temper_core::read_locked( & self.playerId__48).wrapping_mul(7).wrapping_add(13)));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___287).clone().unwrap());
                        }
                        temper_core::listed::add( & ws_conns(), temper_core::read_locked( & self.ws__45).clone().unwrap(), None);
                        let symbol__49: std::sync::Arc<String> = snake::player_head_char(temper_core::read_locked( & self.playerId__48));
                        {
                            * self.t___289.write().unwrap() = temper_core::int_to_string(temper_core::read_locked( & self.playerId__48), None);
                        }
                        println!("Player {} ({}) connected!", temper_core::read_locked( & self.t___289).clone(), symbol__49.clone());
                        if ! temper_core::read_locked( & self.firstMsgRaw__46).is_none() {
                            {
                                * self.caseIndex___360.write().unwrap() = 23;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 24;
                            }
                        }
                    },
                    23 => {
                        {
                            * self.t___291.write().unwrap() = temper_core::read_locked( & self.firstMsgRaw__46).clone().is_some();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 25;
                        }
                    },
                    24 => {
                        {
                            * self.t___291.write().unwrap() = false;
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 25;
                        }
                    },
                    25 => {
                        if temper_core::read_locked( & self.t___291) {
                            {
                                * self.caseIndex___360.write().unwrap() = 26;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 37;
                            }
                        }
                    },
                    26 => {
                        if temper_core::read_locked( & self.firstMsgRaw__46).is_none() {
                            {
                                * self.caseIndex___360.write().unwrap() = 27;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 28;
                            }
                        }
                    },
                    27 => {
                        {
                            * self.t___171.write().unwrap() = panic!();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 29;
                        }
                    },
                    28 => {
                        {
                            * self.t___171.write().unwrap() = temper_core::read_locked( & self.firstMsgRaw__46).clone().unwrap();
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 29;
                        }
                    },
                    29 => {
                        if Some(temper_core::read_locked( & self.t___171).as_str()) == Some("u") {
                            {
                                * self.caseIndex___360.write().unwrap() = 30;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 31;
                            }
                        }
                    },
                    30 => {
                        {
                            * self.t___292.write().unwrap() = Some(snake::Up::new());
                        }
                        {
                            * self.t___293.write().unwrap() = Some(snake::change_player_direction(game().clone(), temper_core::read_locked( & self.playerId__48), snake::Direction::new(temper_core::read_locked( & self.t___292).clone().unwrap())));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___293).clone().unwrap());
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 37;
                        }
                    },
                    31 => {
                        if Some(temper_core::read_locked( & self.t___171).as_str()) == Some("d") {
                            {
                                * self.caseIndex___360.write().unwrap() = 32;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 33;
                            }
                        }
                    },
                    32 => {
                        {
                            * self.t___294.write().unwrap() = Some(snake::Down::new());
                        }
                        {
                            * self.t___295.write().unwrap() = Some(snake::change_player_direction(game().clone(), temper_core::read_locked( & self.playerId__48), snake::Direction::new(temper_core::read_locked( & self.t___294).clone().unwrap())));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___295).clone().unwrap());
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 37;
                        }
                    },
                    33 => {
                        if Some(temper_core::read_locked( & self.t___171).as_str()) == Some("l") {
                            {
                                * self.caseIndex___360.write().unwrap() = 34;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 35;
                            }
                        }
                    },
                    34 => {
                        {
                            * self.t___296.write().unwrap() = Some(snake::Left::new());
                        }
                        {
                            * self.t___297.write().unwrap() = Some(snake::change_player_direction(game().clone(), temper_core::read_locked( & self.playerId__48), snake::Direction::new(temper_core::read_locked( & self.t___296).clone().unwrap())));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___297).clone().unwrap());
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 37;
                        }
                    },
                    35 => {
                        if Some(temper_core::read_locked( & self.t___171).as_str()) == Some("r") {
                            {
                                * self.caseIndex___360.write().unwrap() = 36;
                            }
                        } else {
                            {
                                * self.caseIndex___360.write().unwrap() = 37;
                            }
                        }
                    },
                    36 => {
                        {
                            * self.t___298.write().unwrap() = Some(snake::Right::new());
                        }
                        {
                            * self.t___299.write().unwrap() = Some(snake::change_player_direction(game().clone(), temper_core::read_locked( & self.playerId__48), snake::Direction::new(temper_core::read_locked( & self.t___298).clone().unwrap())));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___299).clone().unwrap());
                        }
                        {
                            * self.caseIndex___360.write().unwrap() = 37;
                        }
                    },
                    37 => {
                        let connId__50: i32 = temper_core::read_locked( & self.playerId__48);
                        let connWs__51: temper_std::ws::WsConnection = temper_core::read_locked( & self.ws__45).clone().unwrap();
                        #[derive(Clone)]
                        struct ClosureGroup___4 {
                            connWs__51: temper_std::ws::WsConnection, connId__50: i32
                        }
                        impl ClosureGroup___4 {
                            fn fn__279(& self) -> temper_core::SafeGenerator<()> {
                                let mut caseIndex___365: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
                                let mut t___265: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
                                let mut t___266: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___267: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___268: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___269: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___270: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___271: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___272: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___273: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___274: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut t___135: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___146: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut msg__53: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___364: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                #[derive(Clone)]
                                struct ClosureGroup___5 {
                                    caseIndex___365: std::sync::Arc<std::sync::RwLock<i32>>, promise___364: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, connWs__51: temper_std::ws::WsConnection, t___135: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, msg__53: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___265: std::sync::Arc<std::sync::RwLock<bool>>, t___274: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, connId__50: i32, t___146: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___266: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>>, t___267: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___268: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>>, t___269: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___270: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>>, t___271: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___272: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>>, t___273: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>
                                }
                                impl ClosureGroup___5 {
                                    fn convertedCoroutine___367(& self, generator___363: temper_core::SafeGenerator<()>) -> Option<()> {
                                        'loop___374: loop {
                                            let caseIndexLocal___366: i32 = temper_core::read_locked( & self.caseIndex___365);
                                            {
                                                * self.caseIndex___365.write().unwrap() = -1;
                                            }
                                            match caseIndexLocal___366.clone() {
                                                0 => {
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 1;
                                                    }
                                                },
                                                1 => {
                                                    if running() {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 2;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 4;
                                                        }
                                                    }
                                                },
                                                2 => {
                                                    {
                                                        * self.promise___364.write().unwrap() = Some(temper_std::ws::ws_recv(( * self.connWs__51.clone()).clone()));
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 3;
                                                    }
                                                    temper_core::read_locked( & self.promise___364).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___363.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                3 => {
                                                    match temper_core::read_locked( & self.promise___364).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            {
                                                                * self.t___135.write().unwrap() = x;
                                                            }
                                                            {
                                                                * self.caseIndex___365.write().unwrap() = 5;
                                                            }
                                                        },
                                                        _ => {
                                                            * self.caseIndex___365.write().unwrap() = 4;
                                                        }
                                                    };
                                                },
                                                4 => {
                                                    return None;
                                                },
                                                5 => {
                                                    {
                                                        * self.msg__53.write().unwrap() = temper_core::read_locked( & self.t___135).clone();
                                                    }
                                                    if ! temper_core::read_locked( & self.msg__53).is_none() {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 6;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 7;
                                                        }
                                                    }
                                                },
                                                6 => {
                                                    {
                                                        * self.t___265.write().unwrap() = temper_core::read_locked( & self.msg__53).clone().is_some();
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 8;
                                                    }
                                                },
                                                7 => {
                                                    {
                                                        * self.t___265.write().unwrap() = false;
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 8;
                                                    }
                                                },
                                                8 => {
                                                    if temper_core::read_locked( & self.t___265) {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 9;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 10;
                                                        }
                                                    }
                                                },
                                                9 => {
                                                    if temper_core::read_locked( & self.msg__53).is_none() {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 11;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 12;
                                                        }
                                                    }
                                                },
                                                10 => {
                                                    {
                                                        * self.t___274.write().unwrap() = temper_core::int_to_string(self.connId__50, None);
                                                    }
                                                    println!("Player {} disconnected", temper_core::read_locked( & self.t___274).clone());
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 4;
                                                    }
                                                },
                                                11 => {
                                                    {
                                                        * self.t___146.write().unwrap() = panic!();
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 13;
                                                    }
                                                },
                                                12 => {
                                                    {
                                                        * self.t___146.write().unwrap() = temper_core::read_locked( & self.msg__53).clone().unwrap();
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 13;
                                                    }
                                                },
                                                13 => {
                                                    if Some(temper_core::read_locked( & self.t___146).as_str()) == Some("u") {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 14;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 15;
                                                        }
                                                    }
                                                },
                                                14 => {
                                                    {
                                                        * self.t___266.write().unwrap() = Some(snake::Up::new());
                                                    }
                                                    {
                                                        * self.t___267.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__50, snake::Direction::new(temper_core::read_locked( & self.t___266).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___267).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 21;
                                                    }
                                                },
                                                15 => {
                                                    if Some(temper_core::read_locked( & self.t___146).as_str()) == Some("d") {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 16;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 17;
                                                        }
                                                    }
                                                },
                                                16 => {
                                                    {
                                                        * self.t___268.write().unwrap() = Some(snake::Down::new());
                                                    }
                                                    {
                                                        * self.t___269.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__50, snake::Direction::new(temper_core::read_locked( & self.t___268).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___269).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 21;
                                                    }
                                                },
                                                17 => {
                                                    if Some(temper_core::read_locked( & self.t___146).as_str()) == Some("l") {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 18;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 19;
                                                        }
                                                    }
                                                },
                                                18 => {
                                                    {
                                                        * self.t___270.write().unwrap() = Some(snake::Left::new());
                                                    }
                                                    {
                                                        * self.t___271.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__50, snake::Direction::new(temper_core::read_locked( & self.t___270).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___271).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 21;
                                                    }
                                                },
                                                19 => {
                                                    if Some(temper_core::read_locked( & self.t___146).as_str()) == Some("r") {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 20;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___365.write().unwrap() = 21;
                                                        }
                                                    }
                                                },
                                                20 => {
                                                    {
                                                        * self.t___272.write().unwrap() = Some(snake::Right::new());
                                                    }
                                                    {
                                                        * self.t___273.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__50, snake::Direction::new(temper_core::read_locked( & self.t___272).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___273).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 21;
                                                    }
                                                },
                                                21 => {
                                                    {
                                                        * self.caseIndex___365.write().unwrap() = 1;
                                                    }
                                                },
                                                _ => {
                                                    return None;
                                                }
                                            }
                                        }
                                    }
                                }
                                let closure_group = ClosureGroup___5 {
                                    caseIndex___365: caseIndex___365.clone(), promise___364: promise___364.clone(), connWs__51: self.connWs__51.clone(), t___135: t___135.clone(), msg__53: msg__53.clone(), t___265: t___265.clone(), t___274: t___274.clone(), connId__50: self.connId__50, t___146: t___146.clone(), t___266: t___266.clone(), t___267: t___267.clone(), t___268: t___268.clone(), t___269: t___269.clone(), t___270: t___270.clone(), t___271: t___271.clone(), t___272: t___272.clone(), t___273: t___273.clone()
                                };
                                let convertedCoroutine___367 = {
                                    let closure_group = closure_group.clone();
                                    std::sync::Arc::new(move | generator___363: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___367(generator___363))
                                };
                                return temper_core::SafeGenerator::from_fn(convertedCoroutine___367.clone().clone());
                            }
                        }
                        let closure_group = ClosureGroup___4 {
                            connWs__51: connWs__51.clone(), connId__50
                        };
                        let fn__279 = {
                            let closure_group = closure_group.clone();
                            std::sync::Arc::new(move | | closure_group.fn__279())
                        };
                        crate::run_async(fn__279.clone().clone());
                        {
                            * self.caseIndex___360.write().unwrap() = 38;
                        }
                    },
                    38 => {
                        {
                            * self.caseIndex___360.write().unwrap() = 3;
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
        caseIndex___360: caseIndex___360.clone(), promise___357: promise___357.clone(), server__44: server__44.clone(), promise___358: promise___358.clone(), t___151: t___151.clone(), ws__45: ws__45.clone(), promise___359: promise___359.clone(), t___152: t___152.clone(), t___154: t___154.clone(), firstMsgRaw__46: firstMsgRaw__46.clone(), t___284: t___284.clone(), t___156: t___156.clone(), isSpectator__47: isSpectator__47.clone(), playerId__48: playerId__48.clone(), t___287: t___287.clone(), t___289: t___289.clone(), t___291: t___291.clone(), t___171: t___171.clone(), t___292: t___292.clone(), t___293: t___293.clone(), t___294: t___294.clone(), t___295: t___295.clone(), t___296: t___296.clone(), t___297: t___297.clone(), t___298: t___298.clone(), t___299: t___299.clone()
    };
    let convertedCoroutine___368 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___356: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___368(generator___356))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___368.clone().clone());
}
