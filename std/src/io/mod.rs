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
pub fn sleep(ms__4: i32) -> temper_core::Promise<()> {
    return panic!();
}
pub fn read_line() -> temper_core::Promise<Option<std::sync::Arc<String>>> {
    return panic!();
}
pub fn terminal_columns() -> i32 {
    return panic!();
}
pub fn terminal_rows() -> i32 {
    return panic!();
}
