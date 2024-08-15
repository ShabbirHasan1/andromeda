use andromeda_core::{Extension, ExtensionOp};
use nova_vm::ecmascript::{
    builtins::ArgumentsList,
    execution::{Agent, JsResult},
    types::Value,
};

#[derive(Default)]
pub struct BatteryExt;

impl BatteryExt {
    pub fn new_extension() -> Extension {
        Extension {
            name: "battery",
            ops: vec![ExtensionOp::new(
                "internal_get_battery",
                Self::internal_get_battery,
                0,
            )],
            storage: None,
            files: vec![],
        }
    }

    fn internal_get_battery(agent: &mut Agent, _this: Value, _: ArgumentsList) -> JsResult<Value> {
        let manager = battery::Manager::new().unwrap();
        let _battery = match manager.batteries().unwrap().next() {
            Some(Ok(battery)) => battery,
            Some(Err(_)) => {
                return Ok(Value::from_string(
                    agent,
                    format!("Error: Unable to access battery information"),
                ));
            }
            None => {
                return Ok(Value::from_string(
                    agent,
                    format!("Error: Unable to find any batteries"),
                ));
            }
        };
        // {
        //     chargingTime: battery.time_to_full().unwrap();
        // }
        Ok(Value::Undefined)
    }
}
