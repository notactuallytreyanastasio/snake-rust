#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            let mut detectedCols__31: i32 = temper_std::io::std_term_cols();
            let mut detectedRows__32: i32 = temper_std::io::std_term_rows();
            let boardWidth__33: i32;
            if Some(detectedCols__31) > Some(100) {
                boardWidth__33 = detectedCols__31.wrapping_sub(4);
            } else {
                boardWidth__33 = 80;
            }
            let boardHeight__34: i32;
            if Some(detectedRows__32) > Some(30) {
                boardHeight__34 = detectedRows__32.wrapping_sub(12);
            } else {
                boardHeight__34 = 30;
            }
            {
                * GAME.write().unwrap() = Some(snake::new_multi_game(boardWidth__33, boardHeight__34, 0, 42));
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
            crate::run_async(std::sync::Arc::new(fn__269.clone()).clone());
            crate::run_async(std::sync::Arc::new(fn__268.clone()).clone());
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
fn fn__269() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___283: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___250: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___251: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::PlayerSnake>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___255: std::sync::Arc<std::sync::RwLock<Option<snake::PlayerSnake>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___259: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::Direction>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___260: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut t___263: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut promise___280: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut dirs__49: std::sync::Arc<std::sync::RwLock<Option<temper_core::ListBuilder<snake::Direction>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut i__50: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut frame__52: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut conns__53: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<temper_std::ws::WsConnection>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut ci__54: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut promise___281: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___282: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___1 {
        caseIndex___283: std::sync::Arc<std::sync::RwLock<i32>>, promise___280: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, dirs__49: std::sync::Arc<std::sync::RwLock<Option<temper_core::ListBuilder<snake::Direction>>>>, t___250: std::sync::Arc<std::sync::RwLock<i32>>, i__50: std::sync::Arc<std::sync::RwLock<i32>>, t___259: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::Direction>>>>, t___260: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, frame__52: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, conns__53: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<temper_std::ws::WsConnection>>>>, t___251: std::sync::Arc<std::sync::RwLock<Option<temper_core::List<snake::PlayerSnake>>>>, t___255: std::sync::Arc<std::sync::RwLock<Option<snake::PlayerSnake>>>, t___263: std::sync::Arc<std::sync::RwLock<i32>>, ci__54: std::sync::Arc<std::sync::RwLock<i32>>, promise___281: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>, promise___282: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<()>>>>
    }
    impl ClosureGroup___1 {
        fn convertedCoroutine___294(& self, generator___279: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___308: loop {
                let caseIndexLocal___284: i32 = temper_core::read_locked( & self.caseIndex___283);
                {
                    * self.caseIndex___283.write().unwrap() = -1;
                }
                match caseIndexLocal___284.clone() {
                    0 => {
                        {
                            * self.caseIndex___283.write().unwrap() = 1;
                        }
                    },
                    1 => {
                        if ! (Some(temper_core::ListedTrait::len( & game().snakes())) == Some(0)) {
                            {
                                * self.caseIndex___283.write().unwrap() = 2;
                            }
                        } else {
                            {
                                * self.caseIndex___283.write().unwrap() = 3;
                            }
                        }
                    },
                    2 => {
                        println!("{}", "Game starting!");
                        {
                            * self.caseIndex___283.write().unwrap() = 6;
                        }
                    },
                    3 => {
                        {
                            * self.promise___280.write().unwrap() = Some(temper_std::io::std_sleep(500));
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 4;
                        }
                        temper_core::read_locked( & self.promise___280).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___279.clone().next();
                        }));
                        return Some(().clone());
                    },
                    4 => {
                        match temper_core::read_locked( & self.promise___280).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___283.write().unwrap() = 5;
                            },
                            _ => {
                                * self.caseIndex___283.write().unwrap() = 17;
                            }
                        };
                    },
                    5 => {
                        {
                            * self.caseIndex___283.write().unwrap() = 1;
                        }
                    },
                    6 => {
                        if running() {
                            {
                                * self.caseIndex___283.write().unwrap() = 7;
                            }
                        } else {
                            {
                                * self.caseIndex___283.write().unwrap() = 17;
                            }
                        }
                    },
                    7 => {
                        {
                            * self.dirs__49.write().unwrap() = Some(temper_core::listed::new_builder());
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 8;
                        }
                    },
                    8 => {
                        {
                            * self.t___250.write().unwrap() = temper_core::ListedTrait::len( & game().snakes());
                        }
                        if ! (Some(temper_core::read_locked( & self.i__50)) < Some(temper_core::read_locked( & self.t___250))) {
                            {
                                * self.caseIndex___283.write().unwrap() = 9;
                            }
                        } else {
                            {
                                * self.caseIndex___283.write().unwrap() = 10;
                            }
                        }
                    },
                    9 => {
                        {
                            * self.t___259.write().unwrap() = Some(temper_core::ListedTrait::to_list( & temper_core::read_locked( & self.dirs__49).clone().unwrap()));
                        }
                        {
                            * self.t___260.write().unwrap() = Some(snake::multi_tick(game().clone(), temper_core::read_locked( & self.t___259).clone().unwrap()));
                        }
                        {
                            * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___260).clone().unwrap());
                        }
                        {
                            * self.frame__52.write().unwrap() = snake::multi_render(game().clone());
                        }
                        {
                            * self.conns__53.write().unwrap() = Some(temper_core::ListedTrait::to_list( & ws_conns()));
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 11;
                        }
                    },
                    10 => {
                        {
                            * self.t___251.write().unwrap() = Some(game().snakes());
                        }
                        {
                            * self.t___255.write().unwrap() = Some(snake::PlayerSnake::new(0, [], snake::Direction::new(snake::Right::new()), 0, snake::PlayerStatus::new(snake::Dead::new())));
                        }
                        let snake__51: snake::PlayerSnake = temper_core::ListedTrait::get_or( & temper_core::read_locked( & self.t___251).clone().unwrap(), temper_core::read_locked( & self.i__50), temper_core::read_locked( & self.t___255).clone().unwrap());
                        temper_core::listed::add( & temper_core::read_locked( & self.dirs__49).clone().unwrap(), snake__51.direction(), None);
                        {
                            * self.i__50.write().unwrap() = temper_core::read_locked( & self.i__50).wrapping_add(1);
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 8;
                        }
                    },
                    11 => {
                        {
                            * self.t___263.write().unwrap() = temper_core::ListedTrait::len( & temper_core::read_locked( & self.conns__53).clone().unwrap());
                        }
                        if ! (Some(temper_core::read_locked( & self.ci__54)) < Some(temper_core::read_locked( & self.t___263))) {
                            {
                                * self.caseIndex___283.write().unwrap() = 12;
                            }
                        } else {
                            {
                                * self.caseIndex___283.write().unwrap() = 13;
                            }
                        }
                    },
                    12 => {
                        {
                            * self.promise___281.write().unwrap() = Some(temper_std::io::std_sleep(200));
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 16;
                        }
                        temper_core::read_locked( & self.promise___281).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___279.clone().next();
                        }));
                        return Some(().clone());
                    },
                    13 => {
                        let conn__55: temper_std::ws::WsConnection = temper_core::ListedTrait::get( & temper_core::read_locked( & self.conns__53).clone().unwrap(), temper_core::read_locked( & self.ci__54));
                        {
                            * self.promise___282.write().unwrap() = Some(temper_std::ws::ws_send(( * conn__55.clone()).clone(), temper_core::read_locked( & self.frame__52).clone()));
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 14;
                        }
                        temper_core::read_locked( & self.promise___282).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___279.clone().next();
                        }));
                        return Some(().clone());
                    },
                    14 => {
                        match temper_core::read_locked( & self.promise___282).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___283.write().unwrap() = 15;
                            },
                            _ => {
                                * self.caseIndex___283.write().unwrap() = 15;
                            }
                        };
                    },
                    15 => {
                        {
                            * self.ci__54.write().unwrap() = temper_core::read_locked( & self.ci__54).wrapping_add(1);
                        }
                        {
                            * self.caseIndex___283.write().unwrap() = 11;
                        }
                    },
                    16 => {
                        match temper_core::read_locked( & self.promise___281).clone().unwrap().get() {
                            Ok(x) => {
                                * self.caseIndex___283.write().unwrap() = 18;
                            },
                            _ => {
                                * self.caseIndex___283.write().unwrap() = 17;
                            }
                        };
                    },
                    17 => {
                        return None;
                    },
                    18 => {
                        {
                            * self.caseIndex___283.write().unwrap() = 6;
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
        caseIndex___283: caseIndex___283.clone(), promise___280: promise___280.clone(), dirs__49: dirs__49.clone(), t___250: t___250.clone(), i__50: i__50.clone(), t___259: t___259.clone(), t___260: t___260.clone(), frame__52: frame__52.clone(), conns__53: conns__53.clone(), t___251: t___251.clone(), t___255: t___255.clone(), t___263: t___263.clone(), ci__54: ci__54.clone(), promise___281: promise___281.clone(), promise___282: promise___282.clone()
    };
    let convertedCoroutine___294 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___279: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___294(generator___279))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___294.clone().clone());
}
fn fn__268() -> temper_core::SafeGenerator<()> {
    let mut caseIndex___299: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
    let mut t___238: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
    let mut t___132: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut server__40: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsServer>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___297: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsServer>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut ws__41: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    let mut promise___298: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
    #[derive(Clone)]
    struct ClosureGroup___2 {
        caseIndex___299: std::sync::Arc<std::sync::RwLock<i32>>, promise___297: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsServer>>>>, server__40: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsServer>>>, promise___298: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<temper_std::ws::WsConnection>>>>, t___132: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, ws__41: std::sync::Arc<std::sync::RwLock<Option<temper_std::ws::WsConnection>>>, t___238: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>
    }
    impl ClosureGroup___2 {
        fn convertedCoroutine___307(& self, generator___296: temper_core::SafeGenerator<()>) -> Option<()> {
            'loop___309: loop {
                let caseIndexLocal___300: i32 = temper_core::read_locked( & self.caseIndex___299);
                {
                    * self.caseIndex___299.write().unwrap() = -1;
                }
                match caseIndexLocal___300.clone() {
                    0 => {
                        println!("{}", "Snake Multiplayer Server");
                        println!("{}", "Starting on port 8080...");
                        {
                            * self.promise___297.write().unwrap() = Some(temper_std::ws::ws_listen(8080));
                        }
                        {
                            * self.caseIndex___299.write().unwrap() = 1;
                        }
                        temper_core::read_locked( & self.promise___297).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___296.clone().next();
                        }));
                        return Some(().clone());
                    },
                    1 => {
                        match temper_core::read_locked( & self.promise___297).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.server__40.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___299.write().unwrap() = 2;
                                }
                            },
                            _ => {
                                * self.caseIndex___299.write().unwrap() = 6;
                            }
                        };
                    },
                    2 => {
                        println!("{}", "Listening on ws://localhost:8080");
                        println!("{}", "Waiting for players to connect...");
                        {
                            * self.caseIndex___299.write().unwrap() = 3;
                        }
                    },
                    3 => {
                        if running() {
                            {
                                * self.caseIndex___299.write().unwrap() = 4;
                            }
                        } else {
                            {
                                * self.caseIndex___299.write().unwrap() = 6;
                            }
                        }
                    },
                    4 => {
                        {
                            * self.promise___298.write().unwrap() = Some(temper_std::ws::ws_accept(( * temper_core::read_locked( & self.server__40).clone().unwrap()).clone()));
                        }
                        {
                            * self.caseIndex___299.write().unwrap() = 5;
                        }
                        temper_core::read_locked( & self.promise___298).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                    generator___296.clone().next();
                        }));
                        return Some(().clone());
                    },
                    5 => {
                        match temper_core::read_locked( & self.promise___298).clone().unwrap().get() {
                            Ok(x) => {
                                {
                                    * self.t___132.write().unwrap() = Some(x);
                                }
                                {
                                    * self.caseIndex___299.write().unwrap() = 7;
                                }
                            },
                            _ => {
                                * self.caseIndex___299.write().unwrap() = 6;
                            }
                        };
                    },
                    6 => {
                        return None;
                    },
                    7 => {
                        {
                            * self.ws__41.write().unwrap() = Some(temper_core::read_locked( & self.t___132).clone().unwrap());
                        }
                        let playerId__42: i32 = next_id();
                        {
                            * NEXT_ID.write().unwrap() = Some(next_id().wrapping_add(1));
                        }
                        {
                            * GAME.write().unwrap() = Some(snake::add_player(game().clone(), playerId__42.wrapping_mul(7).wrapping_add(13)));
                        }
                        temper_core::listed::add( & ws_conns(), temper_core::read_locked( & self.ws__41).clone().unwrap(), None);
                        let symbol__43: std::sync::Arc<String> = snake::player_head_char(playerId__42);
                        {
                            * self.t___238.write().unwrap() = temper_core::int_to_string(playerId__42, None);
                        }
                        println!("Player {} ({}) connected!", temper_core::read_locked( & self.t___238).clone(), symbol__43.clone());
                        let connId__44: i32 = playerId__42;
                        let connWs__45: temper_std::ws::WsConnection = temper_core::read_locked( & self.ws__41).clone().unwrap();
                        #[derive(Clone)]
                        struct ClosureGroup___3 {
                            connWs__45: temper_std::ws::WsConnection, connId__44: i32
                        }
                        impl ClosureGroup___3 {
                            fn fn__231(& self) -> temper_core::SafeGenerator<()> {
                                let mut caseIndex___304: std::sync::Arc<std::sync::RwLock<i32>> = std::sync::Arc::new(std::sync::RwLock::new(0));
                                let mut t___217: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
                                let mut t___218: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___219: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___220: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___221: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___222: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___223: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___224: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___225: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___226: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut t___116: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut t___127: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>> = std::sync::Arc::new(std::sync::RwLock::new(std::sync::Arc::new("".to_string())));
                                let mut msg__47: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                let mut promise___303: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
                                #[derive(Clone)]
                                struct ClosureGroup___4 {
                                    caseIndex___304: std::sync::Arc<std::sync::RwLock<i32>>, promise___303: std::sync::Arc<std::sync::RwLock<Option<temper_core::Promise<Option<std::sync::Arc<String>>>>>>, connWs__45: temper_std::ws::WsConnection, t___116: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, msg__47: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<String>>>>, t___217: std::sync::Arc<std::sync::RwLock<bool>>, t___226: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, connId__44: i32, t___127: std::sync::Arc<std::sync::RwLock<std::sync::Arc<String>>>, t___218: std::sync::Arc<std::sync::RwLock<Option<snake::Up>>>, t___219: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___220: std::sync::Arc<std::sync::RwLock<Option<snake::Down>>>, t___221: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___222: std::sync::Arc<std::sync::RwLock<Option<snake::Left>>>, t___223: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>, t___224: std::sync::Arc<std::sync::RwLock<Option<snake::Right>>>, t___225: std::sync::Arc<std::sync::RwLock<Option<snake::MultiSnakeGame>>>
                                }
                                impl ClosureGroup___4 {
                                    fn convertedCoroutine___306(& self, generator___302: temper_core::SafeGenerator<()>) -> Option<()> {
                                        'loop___310: loop {
                                            let caseIndexLocal___305: i32 = temper_core::read_locked( & self.caseIndex___304);
                                            {
                                                * self.caseIndex___304.write().unwrap() = -1;
                                            }
                                            match caseIndexLocal___305.clone() {
                                                0 => {
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 1;
                                                    }
                                                },
                                                1 => {
                                                    if running() {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 2;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 4;
                                                        }
                                                    }
                                                },
                                                2 => {
                                                    {
                                                        * self.promise___303.write().unwrap() = Some(temper_std::ws::ws_recv(( * self.connWs__45.clone()).clone()));
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 3;
                                                    }
                                                    temper_core::read_locked( & self.promise___303).clone().unwrap().on_ready(std::sync::Arc::new(move | |{
                                                                generator___302.clone().next();
                                                    }));
                                                    return Some(().clone());
                                                },
                                                3 => {
                                                    match temper_core::read_locked( & self.promise___303).clone().unwrap().get() {
                                                        Ok(x) => {
                                                            {
                                                                * self.t___116.write().unwrap() = x;
                                                            }
                                                            {
                                                                * self.caseIndex___304.write().unwrap() = 5;
                                                            }
                                                        },
                                                        _ => {
                                                            * self.caseIndex___304.write().unwrap() = 4;
                                                        }
                                                    };
                                                },
                                                4 => {
                                                    return None;
                                                },
                                                5 => {
                                                    {
                                                        * self.msg__47.write().unwrap() = temper_core::read_locked( & self.t___116).clone();
                                                    }
                                                    if ! temper_core::read_locked( & self.msg__47).is_none() {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 6;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 7;
                                                        }
                                                    }
                                                },
                                                6 => {
                                                    {
                                                        * self.t___217.write().unwrap() = temper_core::read_locked( & self.msg__47).clone().is_some();
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 8;
                                                    }
                                                },
                                                7 => {
                                                    {
                                                        * self.t___217.write().unwrap() = false;
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 8;
                                                    }
                                                },
                                                8 => {
                                                    if temper_core::read_locked( & self.t___217) {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 9;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 10;
                                                        }
                                                    }
                                                },
                                                9 => {
                                                    if temper_core::read_locked( & self.msg__47).is_none() {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 11;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 12;
                                                        }
                                                    }
                                                },
                                                10 => {
                                                    {
                                                        * self.t___226.write().unwrap() = temper_core::int_to_string(self.connId__44, None);
                                                    }
                                                    println!("Player {} disconnected", temper_core::read_locked( & self.t___226).clone());
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 4;
                                                    }
                                                },
                                                11 => {
                                                    {
                                                        * self.t___127.write().unwrap() = panic!();
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 13;
                                                    }
                                                },
                                                12 => {
                                                    {
                                                        * self.t___127.write().unwrap() = temper_core::read_locked( & self.msg__47).clone().unwrap();
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 13;
                                                    }
                                                },
                                                13 => {
                                                    if Some(temper_core::read_locked( & self.t___127).as_str()) == Some("u") {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 14;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 15;
                                                        }
                                                    }
                                                },
                                                14 => {
                                                    {
                                                        * self.t___218.write().unwrap() = Some(snake::Up::new());
                                                    }
                                                    {
                                                        * self.t___219.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__44, snake::Direction::new(temper_core::read_locked( & self.t___218).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___219).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 21;
                                                    }
                                                },
                                                15 => {
                                                    if Some(temper_core::read_locked( & self.t___127).as_str()) == Some("d") {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 16;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 17;
                                                        }
                                                    }
                                                },
                                                16 => {
                                                    {
                                                        * self.t___220.write().unwrap() = Some(snake::Down::new());
                                                    }
                                                    {
                                                        * self.t___221.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__44, snake::Direction::new(temper_core::read_locked( & self.t___220).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___221).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 21;
                                                    }
                                                },
                                                17 => {
                                                    if Some(temper_core::read_locked( & self.t___127).as_str()) == Some("l") {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 18;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 19;
                                                        }
                                                    }
                                                },
                                                18 => {
                                                    {
                                                        * self.t___222.write().unwrap() = Some(snake::Left::new());
                                                    }
                                                    {
                                                        * self.t___223.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__44, snake::Direction::new(temper_core::read_locked( & self.t___222).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___223).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 21;
                                                    }
                                                },
                                                19 => {
                                                    if Some(temper_core::read_locked( & self.t___127).as_str()) == Some("r") {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 20;
                                                        }
                                                    } else {
                                                        {
                                                            * self.caseIndex___304.write().unwrap() = 21;
                                                        }
                                                    }
                                                },
                                                20 => {
                                                    {
                                                        * self.t___224.write().unwrap() = Some(snake::Right::new());
                                                    }
                                                    {
                                                        * self.t___225.write().unwrap() = Some(snake::change_player_direction(game().clone(), self.connId__44, snake::Direction::new(temper_core::read_locked( & self.t___224).clone().unwrap())));
                                                    }
                                                    {
                                                        * GAME.write().unwrap() = Some(temper_core::read_locked( & self.t___225).clone().unwrap());
                                                    }
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 21;
                                                    }
                                                },
                                                21 => {
                                                    {
                                                        * self.caseIndex___304.write().unwrap() = 1;
                                                    }
                                                },
                                                _ => {
                                                    return None;
                                                }
                                            }
                                        }
                                    }
                                }
                                let closure_group = ClosureGroup___4 {
                                    caseIndex___304: caseIndex___304.clone(), promise___303: promise___303.clone(), connWs__45: self.connWs__45.clone(), t___116: t___116.clone(), msg__47: msg__47.clone(), t___217: t___217.clone(), t___226: t___226.clone(), connId__44: self.connId__44, t___127: t___127.clone(), t___218: t___218.clone(), t___219: t___219.clone(), t___220: t___220.clone(), t___221: t___221.clone(), t___222: t___222.clone(), t___223: t___223.clone(), t___224: t___224.clone(), t___225: t___225.clone()
                                };
                                let convertedCoroutine___306 = {
                                    let closure_group = closure_group.clone();
                                    std::sync::Arc::new(move | generator___302: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___306(generator___302))
                                };
                                return temper_core::SafeGenerator::from_fn(convertedCoroutine___306.clone().clone());
                            }
                        }
                        let closure_group = ClosureGroup___3 {
                            connWs__45: connWs__45.clone(), connId__44
                        };
                        let fn__231 = {
                            let closure_group = closure_group.clone();
                            std::sync::Arc::new(move | | closure_group.fn__231())
                        };
                        crate::run_async(fn__231.clone().clone());
                        {
                            * self.caseIndex___299.write().unwrap() = 3;
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
        caseIndex___299: caseIndex___299.clone(), promise___297: promise___297.clone(), server__40: server__40.clone(), promise___298: promise___298.clone(), t___132: t___132.clone(), ws__41: ws__41.clone(), t___238: t___238.clone()
    };
    let convertedCoroutine___307 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | generator___296: temper_core::SafeGenerator<()> | closure_group.convertedCoroutine___307(generator___296))
    };
    return temper_core::SafeGenerator::from_fn(convertedCoroutine___307.clone().clone());
}
