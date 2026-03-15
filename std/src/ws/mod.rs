#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
mod support;
pub use support::*;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
pub trait WsServerTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> WsServer;
}
#[derive(Clone)]
pub struct WsServer(std::sync::Arc<dyn WsServerTrait>);
impl WsServer {
    pub fn new(selfish: impl WsServerTrait + 'static) -> WsServer {
        WsServer(std::sync::Arc::new(selfish))
    }
}
impl WsServerTrait for WsServer {
    fn clone_boxed(& self) -> WsServer {
        WsServerTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(WsServer);
impl std::ops::Deref for WsServer {
    type Target = dyn WsServerTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait WsConnectionTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> WsConnection;
}
#[derive(Clone)]
pub struct WsConnection(std::sync::Arc<dyn WsConnectionTrait>);
impl WsConnection {
    pub fn new(selfish: impl WsConnectionTrait + 'static) -> WsConnection {
        WsConnection(std::sync::Arc::new(selfish))
    }
}
impl WsConnectionTrait for WsConnection {
    fn clone_boxed(& self) -> WsConnection {
        WsConnectionTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(WsConnection);
impl std::ops::Deref for WsConnection {
    type Target = dyn WsConnectionTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub fn ws_listen(port__6: i32) -> temper_core::Promise<WsServer> {
    return panic!();
}
pub fn ws_accept(server__8: WsServer) -> temper_core::Promise<WsConnection> {
    return panic!();
}
pub fn ws_connect(url__10: impl temper_core::ToArcString) -> temper_core::Promise<WsConnection> {
    let url__10 = url__10.to_arc_string();
    return panic!();
}
pub fn ws_send(conn__12: WsConnection, msg__13: impl temper_core::ToArcString) -> temper_core::Promise<()> {
    let msg__13 = msg__13.to_arc_string();
    return panic!();
}
pub fn ws_recv(conn__15: WsConnection) -> temper_core::Promise<Option<std::sync::Arc<String>>> {
    return panic!();
}
pub fn ws_close(conn__17: WsConnection) -> temper_core::Promise<()> {
    return panic!();
}
