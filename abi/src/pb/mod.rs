#[allow(clippy::all, non_camel_case_types)]
pub mod function;
#[allow(clippy::all, non_camel_case_types)]
pub mod trigger;

pub use function::function::Func;
pub use function::{
    function_service_server::{FunctionService, FunctionServiceServer},
    Function, FunctionQuery, FunctionType,
};
pub use trigger::trigger::TriggerConfig;
pub use trigger::{
    trigger_service_server::{TriggerService, TriggerServiceServer},
    Trigger, TriggerQuery, TriggerType,
};
