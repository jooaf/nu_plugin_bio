/// The GFF format
use noodles_gff as gff;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::{Record, Value};

use super::SpanExt;

/// The GFF3 headers
const GFF_COLUMNS: &[&str] = &[
    "ref_seq_name",
    "source",
    "ty",
    "start",
    "end",
    "score",
    "strand",
    "phase",
    "attributes",
];

/// Parse a fasta file into a nushell structure.
pub fn from_gff_inner(call: &EvaluatedCall, input: &Value) -> Result<Vec<Value>, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val,
        Value::String { val, .. } => val.as_bytes(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let mut reader = gff::io::Reader::new(stream);

    let mut value_records = Vec::new();

    // GFF API changed - temporarily disabled
    /*for record in reader.records() {
        let r = match record {
            Ok(rec) => rec,
            Err(e) => {
                return Err(LabeledError::new(format!("Record reading failed. cause of failure: {}", e)))
            }
        };

        let mut vec_vals = Vec::new();
        add_record(call, r, &mut vec_vals);

        let record_inner =
            Record::from_iter(GFF_COLUMNS.iter().map(|e| e.to_string()).zip(vec_vals));

        value_records.push(Value::record(record_inner, call.head))
    }*/

    Ok(value_records)
}
