use std::io::{self, Read};

use anyhow::anyhow;
use bson::Document;
use common::event::{ChangeStreamEvent, OperationType};

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("{:?}", buf);

    let event: ChangeStreamEvent<Document> = serde_json::from_str(&buf)?;

    println!("handle event: {:?}", event);
    if event.operation_type == OperationType::Delete {
        return Err(anyhow!("unsuppored op_type: Delete"));
    }

    Ok(())
}
