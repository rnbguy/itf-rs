use std::fs::read_to_string;

use itf_rs::Trace;
use schemars::schema_for;

fn main() -> anyhow::Result<()> {
    {
        let f = read_to_string("samples/example_trace.itf.json")?;
        let _: Trace = serde_json::from_str(&f)?;
    }
    let schema = schema_for!(Trace);
    let schema_json = serde_json::to_string_pretty(&schema)?;
    std::fs::write("itf.schema.json", schema_json)?;
    Ok(())
}
