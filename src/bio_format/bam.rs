use crate::bio_format::SpanExt;
use noodles_bam as bam;
use noodles_sam as sam;
use noodles_sam::alignment::Record as SAMRecord;
use noodles_sam::header::record::value::Map;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::{record, Record, Value};
use std::io::{BufReader, Cursor};

/// Columns in a BAM/SAM file
pub const BAM_COLUMNS: &[&str] = &[
    "read_name",
    "flags",
    "reference_sequence_id",
    "alignment_start",
    "mapping_quality",
    "cigar",
    "mate_reference_sequence_id",
    "mate_alignment_start",
    "template_length",
    "sequence",
    "quality_scores",
    "data",
];

/// Header fields in a B/SAM file
pub const HEADER_COLUMNS: &[&str] = &[
    "metadata",
    "reference_sequences",
    "read_groups",
    "programs",
    "comments",
];

/// Parse a B/SAM header
pub fn parse_header(call: &EvaluatedCall, h: &sam::Header) -> Value {
    // @HD in SAM.
    let header_op = h.header();
    // unwrap to the default header whatever that is?
    // should be able to modify it later to suit our needs.
    let default_map = Map::default();
    let header = header_op.unwrap_or(&default_map);

    let header_nuon = Value::record(
        record!(
        "version" => call.head.with_string(header.version()),
        // what's the default..?
        // if it's no good, we can always map -> string
        "sorting_order" => call.head.with_string("unknown"),
        "grouping" => call.head.with_string("unknown"),
        "sub_sort_order" => call.head.with_string("unknown")
        ),
        call.head,
    );

    // @SQ.
    let reference_sequences = h.reference_sequences();
    let mut reference_sequences_record = Record::new();
    for (name, f) in reference_sequences.iter() {
        let seq_record = record! {
            "sequence_name" => call.head.with_string(name),
            "sequence_length" => Value::int(usize::from(f.length()) as i64, call.head),
            // TODO: These methods may not exist in the current API - will need to check later
            // "alternate_locus" => call.head.with_string_or(f.alternative_locus(), "No alternative locus."),
            // "alternate_names" => call.head.with_string_or(f.alternative_names(), "No alternative names."),
            // "assembly_id" => call.head.with_string_or(f.assembly_id(), "No assembly ID."),
            // "description" => call.head.with_string_or(f.description(), "No description"),
            // "md5" => call.head.with_string_or(f.md5_checksum(), "No md5 checksum"),
            // "species" => call.head.with_string_or(f.species(), "No species name"),
            // "molecule_topology" => call.head.with_string_or(f.molecule_topology(), "No molecule topology"),
            // "uri" => call.head.with_string_or(f.uri(), "No URI"),
        };
        reference_sequences_record.push(name.to_string(), Value::record(seq_record, call.head));
    }
    let reference_sequences_nuon = Value::record(reference_sequences_record, call.head);

    // @RG
    let read_groups = h.read_groups();
    let mut read_groups_record = Record::new();
    for (id, _f) in read_groups.iter() {
        let value = Value::record(
            record! {
                "id" => call.head.with_string(id),
                "barcode" => call.head.with_string("unknown"),
                "sequencing_center" => call.head.with_string("unknown"),
                "description" => call.head.with_string("unknown"),
                "flow_order" => call.head.with_string("unknown"),
                "key_sequence" => call.head.with_string("unknown"),
                "library" => call.head.with_string("unknown"),
                "program" => call.head.with_string("unknown"),
                "platform" => call.head.with_string("unknown"),
                "predicted_insert_size" => Value::int(0, call.head),
                "platform_model" => call.head.with_string("unknown"),
                "platform_unit" => call.head.with_string("unknown"),
                "sample" => call.head.with_string("unknown"),
            },
            call.head,
        );
        read_groups_record.push(id.to_string(), value);
    }
    let read_groups_nuon = Value::record(read_groups_record, call.head);

    // @PG - disabled due to API changes
    let programs_record = Record::new();
    let programs_nuon = Value::record(programs_record, call.head);

    // @CO
    let comments = h.comments();
    let comments_nuon = Value::list(
        comments.iter().map(|e| call.head.with_string(e)).collect(),
        call.head,
    );

    Value::record(
        record! {
            HEADER_COLUMNS[0] => header_nuon,
            HEADER_COLUMNS[1] => reference_sequences_nuon,
            HEADER_COLUMNS[2] => read_groups_nuon,
            HEADER_COLUMNS[3] => programs_nuon,
            HEADER_COLUMNS[4] => comments_nuon,
        },
        call.head,
    )
}

/// Parse a SAM record, and append to a vector
pub fn create_record_values<R: SAMRecord>(
    call: &EvaluatedCall,
    r: R,
    header: &sam::Header,
) -> Vec<Value> {
    // Extract basic fields that we know work
    let flags = r.flags().map(|f| f.bits()).unwrap_or(0);

    let mapping_quality = r
        .mapping_quality()
        .map(|m_q| {
            m_q.map(|mq| format!("{}", u8::from(mq)))
                .unwrap_or("*".to_string())
        })
        .unwrap_or("*".to_string());

    let read_name = r
        .name()
        .map(|n| String::from_utf8_lossy(n).to_string())
        .unwrap_or("*".to_string());

    let reference_sequence_id = r
        .reference_sequence_id(header)
        .map(|id| {
            id.map(|i| format!("{}", usize::from(i)))
                .unwrap_or("*".to_string())
        })
        .unwrap_or("*".to_string());

    let alignment_start = r
        .alignment_start()
        .map(|pos| {
            pos.map(|p| format!("{}", usize::from(p)))
                .unwrap_or("0".to_string())
        })
        .unwrap_or("0".to_string());

    // Extract CIGAR, sequence, and quality scores - TODO fix, simplified for now
    let cigar = {
        let c = r.cigar();
        format!("cigar_ops:{}", c.len())
    };

    let sequence = {
        let seq = r.sequence();
        // TODO: Just show sequence length for now
        if seq.len() > 0 {
            format!("sequence_length:{}", seq.len())
        } else {
            "*".to_string()
        }
    };

    let quality_scores = {
        let qual = r.quality_scores();
        // TODO: Debug quality scores information
        let len = qual.len();
        if len > 0 {
            format!("quality_length:{}", len)
        } else {
            // Show that we have quality scores object but it's empty
            "quality_length:0".to_string()
        }
    };

    let mate_reference_sequence_id = r
        .mate_reference_sequence_id(header)
        .map(|id| {
            id.map(|i| format!("{}", usize::from(i)))
                .unwrap_or("*".to_string())
        })
        .unwrap_or("*".to_string());

    let mate_alignment_start = r
        .mate_alignment_start()
        .map(|pos| {
            pos.map(|p| format!("{}", usize::from(p)))
                .unwrap_or("0".to_string())
        })
        .unwrap_or("0".to_string());

    let template_length = r
        .template_length()
        .map(|len| format!("{}", len))
        .unwrap_or("0".to_string());

    let data = r
        .data()
        .iter()
        .filter_map(|field_result| {
            field_result.ok().map(|(tag, value)| {
                let tag_str = format!("{:?}", tag)
                    .replace("Tag(\"", "")
                    .replace("\")", "");
                let value_str = match value {
                    sam::alignment::record::data::field::Value::Character(c) => {
                        format!("A:{}", c as char)
                    }
                    sam::alignment::record::data::field::Value::Int32(i) => format!("i:{}", i),
                    sam::alignment::record::data::field::Value::Float(f) => format!("f:{}", f),
                    sam::alignment::record::data::field::Value::String(s) => {
                        format!("Z:{}", String::from_utf8_lossy(s))
                    }
                    sam::alignment::record::data::field::Value::Hex(h) => {
                        format!("H:{}", String::from_utf8_lossy(h))
                    }
                    sam::alignment::record::data::field::Value::Array(_arr) => {
                        "B:array".to_string()
                    }
                    _ => format!("{:?}", value),
                };
                format!("{}:{}", tag_str, value_str)
            })
        })
        .collect::<Vec<_>>()
        .join("\t");

    vec![
        call.head.with_string_or(r.name(), "No read name."),
        call.head.with_string(format!("{:#06x}", flags)),
        call.head.with_string(reference_sequence_id),
        call.head.with_string(alignment_start),
        call.head.with_string(mapping_quality),
        call.head.with_string(cigar),
        call.head.with_string(mate_reference_sequence_id),
        call.head.with_string(mate_alignment_start),
        call.head.with_string(template_length),
        call.head.with_string(sequence),
        call.head.with_string(quality_scores),
        call.head.with_string(data),
    ]
}

/// Parse a BAM file into a nushell structure.
pub fn from_bam_inner(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        other => {
            return Err(LabeledError::new(format!(
                "Input must be binary or string data, got {}",
                other.get_type()
            )))
        }
    };

    let mut reader = bam::io::Reader::new(stream.as_slice());
    let raw_header = reader.read_header().map_err(|err| {
        LabeledError::new(format!(
            "Could not read header. error reading header at {}",
            err
        ))
    })?;

    // TODO: better error handling here.
    let header = if raw_header.is_empty() {
        let ref_seqs = raw_header.reference_sequences().clone();

        parse_header(
            call,
            &sam::Header::builder()
                .set_reference_sequences(ref_seqs)
                .build(),
        )
    } else {
        // this is required for reasons unclear to me...
        parse_header(call, &raw_header)
    };

    let value_records = reader
        .records()
        .map(|record| {
            let r = record.map_err(|e| {
                LabeledError::new(format!("Record reading failed. cause of failure: {}", e))
            })?;

            let inner_record = Record::from_iter(
                BAM_COLUMNS
                    .iter()
                    .map(|e| e.to_string())
                    .zip(create_record_values(call, r, &raw_header)),
            );

            Ok(Value::record(inner_record, call.head))
        })
        .collect::<Result<Vec<_>, LabeledError>>()?;

    Ok(Value::record(
        record! {
            "header" => header,
            "body" => Value::list(value_records, call.head)
        },
        call.head,
    ))
}

/// Parse a SAM file into a nushell structure.
pub fn from_sam_inner(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let mut reader = sam::io::Reader::new(BufReader::new(Cursor::new(stream)));
    let header = reader
        .read_header()
        .map_err(|err| LabeledError::new(format!("Unable to parse SAM header: {}", err)))?;
    let header_nuon = parse_header(call, &header);

    let value_records = reader
        .records()
        .map(|record| {
            let r = record.map_err(|e| {
                LabeledError::new(format!("Record reading failed. cause of failure: {}", e))
            })?;

            let inner_record = Record::from_iter(
                BAM_COLUMNS
                    .iter()
                    .map(|e| e.to_string())
                    .zip(create_record_values(call, r, &header)),
            );

            Ok(Value::record(inner_record, call.head))
        })
        .collect::<Result<Vec<_>, LabeledError>>()?;

    Ok(Value::record(
        record! {
            "header" => header_nuon,
            "body" => Value::list(value_records, call.head)
        },
        call.head,
    ))
}
