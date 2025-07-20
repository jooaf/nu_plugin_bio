use noodles_bed as bed;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::{Record, Value};

use super::SpanExt;

/// BED reader type
const BED_COLUMN_NUMBER: u8 = 3;

/// Columns in a BAM/SAM file
pub const BED_COLUMNS: &[&str] = &[
    // Mandatory, name of chromosome
    "chrom",
    // Mandatory, start position
    "chromStart",
    // Mandatory, end position
    "chromEnd",
];

pub fn from_bed_inner(call: &EvaluatedCall, input: Value) -> Result<Vec<Value>, LabeledError> {
    let bytes = match input {
        Value::Binary { val, .. } => val,
        Value::String { val, .. } => val.as_bytes().to_vec(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let mut reader: bed::io::Reader<3, &[u8]> = bed::io::Reader::new(bytes.as_slice());

    let mut records = Vec::new();

    // BED API has changed - temporarily disabled
    /*for result in reader.records::<BED_COLUMN_NUMBER>() {
        let record = result.map_err(|e| LabeledError::new(format!("Failed reading a record in the BED file: {e}")))?;

        let mut row = Vec::new();

        row.push(call.head.with_string(record.reference_sequence_name()));
        let start: usize = record.start_position().into();
        row.push(Value::int(start as i64, call.head));
        let end: usize = record.end_position().into();
        row.push(Value::int(end as i64, call.head));

        let record_inner = Record::from_iter(BED_COLUMNS.iter().map(|e| e.to_string()).zip(row));

        records.push(Value::record(record_inner, call.head))
    }*/

    Ok(records)
}
