use std::io::{BufRead, BufReader, Cursor};

use noodles_fasta::{
    record::{Definition as FastaDefinition, Record as FastaRecord, Sequence},
    io::Writer as FastaWriter,
};
use noodles_fastq::{
    record::{Definition as FastqDefinition, Record as FastqRecord},
    io::Writer as FastqWriter,
};
use noodles_bgzf as bgzf;
use noodles_fasta as fasta;
use noodles_fastq as fastq;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::Value;

use crate::bio_format::{Compression, SpanExt};

/// Compression status of a fastq reader.
enum FastqReader<'a> {
    Uncompressed(Box<fastq::io::Reader<&'a [u8]>>),
    Compressed(Box<fastq::io::Reader<BufReader<bgzf::io::Reader<&'a [u8]>>>>),
}

/// Compression status of a fasta reader.
enum FastaReader<'a> {
    Uncompressed(Box<fasta::io::Reader<&'a [u8]>>),
    Compressed(fasta::io::Reader<Box<bgzf::io::Reader<&'a [u8]>>>),
}

/// Iterate over the records of a reader that implements [`BufRead`].
fn iterate_fastq_records<R: BufRead>(
    mut reader: fastq::io::Reader<R>,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
    description: bool,
    quality_scores: bool,
    cols: Vec<String>,
) -> Result<(), LabeledError> {
    // iterate over the records.
    for record in reader.records() {
        let r = record.map_err(|e| LabeledError::new(format!("Record reading failed. cause of failure: {}", e)))?;

        let mut vec_vals = Vec::new();
        vec_vals.push(call.head.with_string_from_utf8(r.name()));

        if description {
            vec_vals.push(call.head.with_string_from_utf8(r.description()));
        }

        if quality_scores {
            vec_vals.push(call.head.with_string_from_utf8(r.quality_scores()));
        }

        vec_vals.push(call.head.with_string_from_utf8(r.sequence()));

        let mut tmp_record = nu_protocol::Record::new();
        for (col, val) in cols.clone().iter().zip(vec_vals) {
            tmp_record.push(col, val);
        }
        value_records.push(Value::record(tmp_record, call.head))
    }

    Ok(())
}

pub fn from_fastq_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: Compression,
) -> Result<Vec<Value>, LabeledError> {
    // parse description flag.
    let description = call.has_flag("description")?;
    let quality_scores = call.has_flag("quality-scores")?;

    let bytes = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let reader = match gz {
        Compression::Uncompressed => FastqReader::Uncompressed(Box::new(fastq::io::Reader::new(bytes.as_slice()))),
        Compression::Gzipped => {
            let gz = bgzf::io::Reader::new(bytes.as_slice());
            FastqReader::Compressed(Box::new(fastq::io::Reader::new(BufReader::new(gz))))
        }
    };

    let cols = match (description, quality_scores) {
        (false, false) => vec!["id".to_string(), "sequence".to_string()],
        (true, false) => vec![
            "id".to_string(),
            "description".to_string(),
            "sequence".to_string(),
        ],
        (false, true) => vec![
            "id".to_string(),
            "quality_scores".to_string(),
            "sequence".to_string(),
        ],
        (true, true) => vec![
            "id".to_string(),
            "description".to_string(),
            "quality_scores".to_string(),
            "sequence".to_string(),
        ],
    };

    let mut value_records = Vec::new();

    match reader {
        FastqReader::Uncompressed(u) => iterate_fastq_records(
            *u,
            call,
            &mut value_records,
            description,
            quality_scores,
            cols,
        )?,
        FastqReader::Compressed(c) => iterate_fastq_records(
            *c,
            call,
            &mut value_records,
            description,
            quality_scores,
            cols,
        )?,
    };

    Ok(value_records)
}

fn iterate_fasta_records<R: BufRead>(
    mut reader: fasta::io::Reader<R>,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
    description: bool,
    cols: Vec<String>,
) -> Result<(), LabeledError> {
    // iterate over the records
    for record in reader.records() {
        let r = record.map_err(|e| LabeledError::new(format!("Record reading failed. cause of failure: {}", e)))?;

        let mut vec_vals = Vec::new();

        vec_vals.push(call.head.with_string(String::from_utf8_lossy(r.name())));

        if description {
            vec_vals.push(call.head.with_string_or(r.description(), ""));
        }

        vec_vals.push(call.head.with_string_from_utf8(r.sequence().as_ref()));

        let mut tmp_record = nu_protocol::Record::new();
        for (col, val) in cols.clone().iter().zip(vec_vals) {
            tmp_record.push(col, val);
        }
        value_records.push(Value::record(tmp_record, call.head))
    }
    Ok(())
}

/// Parse a fasta file into a nushell structure.
pub fn from_fasta_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: Compression,
) -> Result<Vec<Value>, LabeledError> {
    // parse description flag.
    let description = call.has_flag("description")?;

    let bytes = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let reader = match gz {
        Compression::Uncompressed => FastaReader::Uncompressed(Box::new(fasta::io::Reader::new(bytes.as_slice()))),
        Compression::Gzipped => {
            let gz = Box::new(bgzf::io::Reader::new(bytes.as_slice()));
            FastaReader::Compressed(fasta::io::Reader::new(gz))
        }
    };

    let cols = match description {
        false => vec!["id".to_string(), "sequence".to_string()],
        true => vec![
            "id".to_string(),
            "description".to_string(),
            "sequence".to_string(),
        ],
    };

    let mut value_records = Vec::new();

    match reader {
        FastaReader::Uncompressed(u) => {
            iterate_fasta_records(*u, call, &mut value_records, description, cols)?
        }
        FastaReader::Compressed(c) => {
            iterate_fasta_records(c, call, &mut value_records, description, cols)?
        }
    };

    Ok(value_records)
}

/// Go from a parsed nuon fasta structure to a string to stdout
///
/// Note that this assumes that we are parsing fasta format specifically.
pub fn nuon_to_fasta(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let mut out = FastaWriter::new(Vec::new());

    if let Ok(list) = input.as_list() {
        for el in list {
            let inner = el.as_record()?;
            let mut vals: Vec<Value> = inner.iter().map(|(_, v)| v.clone()).collect();
            let last = vals.pop().unwrap();
            let sequence = last.as_str()?;

            let id = vals.get(0).map(|e| e.as_str().unwrap());
            let description = vals.get(1).map(|e| e.as_str().unwrap());

            let fa_def = FastaDefinition::new(id.unwrap_or("".into()), description.map(|s| s.as_bytes().into()));
            let fa_seq = Sequence::from(sequence.as_bytes().to_vec());

            out.write_record(&FastaRecord::new(fa_def.clone(), fa_seq))
                .map_err(|err| LabeledError::new(format!("Error in writing record ({}) to fasta: {}", fa_def, err)))?;
        }
    }

    let bytes = out.get_ref();
    let out_final = String::from_utf8(bytes.clone()).map_err(|err| LabeledError::new(format!("Can't format bytes as UTF-8: {}", err)))?;

    Ok(Value::string(out_final, call.head))
}

pub fn nuon_to_fastq(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let mut out = FastqWriter::new(Vec::new());

    if let Ok(list) = input.as_list() {
        // we need to check the columns
        let first = list.first();
        let (description, quality) = match first {
            Some(e) => {
                let first_inner = e.as_record()?;
                let cols: Vec<String> = first_inner.iter().map(|(k, _)| k.clone()).collect();
                (
                    cols.contains(&String::from("description")),
                    cols.contains(&String::from("quality_scores")),
                )
            }
            None => {
                // what's the error?
                return Err(LabeledError::new("No value: There was no first value to call `to fastq` on"));
            }
        };

        // if we don't have quality scores no point going further.
        if !quality {
            return Err(LabeledError::new("No quality scores: Consider using `to fasta` if you don't have any quality scores, or pass the -q option on a fastq"));
        }

        for el in list {
            let inner = el.as_record()?;
            // we need to check the columns.
            let mut vals: Vec<Value> = inner.iter().map(|(_, v)| v.clone()).collect();
            let last = vals.pop().unwrap();
            let sequence = last.as_str()?;

            let id = vals.get(0).map(|e| e.as_str().unwrap());

            let (d, q) = match (description, quality) {
                (true, true) => {
                    // we got both
                    let d = vals.get(1).map(|e| e.as_str().unwrap());
                    let q = vals.get(2).map(|e| e.as_str().unwrap());
                    (d, q)
                }
                (false, true) => {
                    let q = vals.get(1).map(|e| e.as_str().unwrap());
                    (None, q)
                }
                _ => unreachable!(),
            };

            let fq_def = FastqDefinition::new(id.unwrap_or("".into()), d.unwrap_or("".into()));

            out.write_record(&FastqRecord::new(
                fq_def.clone(),
                sequence.as_bytes(),
                q.unwrap_or("".into()).as_bytes(),
            ))
            .map_err(|err| LabeledError::new(format!("Error in writing record ({:?}) to fastq: {}", fq_def, err)))?;
        }
    }

    let bytes = out.get_ref();
    let out_final = String::from_utf8(bytes.clone()).map_err(|err| LabeledError::new(format!("Can't format bytes as UTF-8: {}", err)))?;

    Ok(Value::string(out_final, call.head))
}
