pub mod event;
pub mod mem;
pub mod result;

pub const EVENT_HANDLER_ENTRY_FUNC_NAME: &str = "event_handler_entry";

pub const ALLOCATE_FUNC_RUST: &str = "allocate";
pub const ALLOCATE_FUNC_GO: &str = "malloc";
pub const ALLOCATE_FUNC_C: &str = "malloc";

pub const DEALLOCATE_FUNC_RUST: &str = "deallocate";
pub const DEALLOCATE_FUNC_GO: &str = "free";
pub const DEALLOCATE_FUNC_C: &str = "free";
