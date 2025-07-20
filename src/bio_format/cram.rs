/// The CRAM format
use noodles_cram as cram;
use noodles_sam as sam;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::{record, Record, Value};
use crate::bio_format::SpanExt;

use crate::bio_format::bam::{create_record_values, parse_header, BAM_COLUMNS};
// TODO: also allow the reference to be passed, so we can view the alignment sequences?

/// Parse a CRAM file into a nushell structure.
pub fn from_cram_inner(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        other => {
            return Err(LabeledError::new(format!("Input must be binary or string data, got {}", other.get_type())))
        }
    };

    let mut reader = cram::io::Reader::new(stream.as_slice());

    match reader.read_file_definition() {
        Ok(_) => (),
        Err(e) => {
            return Err(LabeledError::new(format!("Could not read CRAM file definition. cause of failure: {}", e)))
        }
    };

    let header: sam::Header = match reader.read_file_header() {
        Ok(s) => s,
        Err(e) => {
            return Err(LabeledError::new(format!("CRAM file header reading failed. cause of failure: {}", e)))
        }
    };

    let header_nuon = parse_header(call, &header);

    let mut value_records = Vec::new();

    // CRAM files often require a reference sequence, but we'll try to read what we can
    // If this fails, we'll return just the header with empty records
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut temp_records = Vec::new();
        for result in reader.records(&header) {
            match result {
                Ok(record) => {
                    let vec_vals = create_record_values(call, record, &header);
                    let records_inner = Record::from_iter(
                        BAM_COLUMNS.iter().map(|e| e.to_string()).zip(vec_vals)
                    );
                    temp_records.push(Value::record(records_inner, call.head));
                },
                Err(_) => break, // Stop on first error rather than failing completely
            }
        }
        temp_records
    })) {
        Ok(records) => value_records = records,
        Err(_) => {
            // If reading records fails (e.g., missing reference), return empty records with a note
            return Ok(Value::record(
                record! {
                    "header" => header_nuon,
                    "body" => Value::list(vec![], call.head),
                    "note" => call.head.with_string("CRAM file may require a reference sequence for full parsing")
                },
                call.head,
            ));
        }
    }

    Ok(Value::record(
        record! {
            "header" => header_nuon,
            "body" => Value::list(value_records, call.head)
        },
        call.head,
    ))
}
