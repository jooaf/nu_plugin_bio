/// The VCF format
use noodles_bcf as bcf;
use noodles_bgzf as bgzf;
use noodles_vcf as vcf;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::{record, Record, Value};

use crate::bio_format::Compression;
use std::io::{BufRead, BufReader, Cursor};

type StringMaps = vcf::header::StringMaps;

use super::SpanExt;

/// Compression status of a VCF reader.
enum VCFReader<'a> {
    Uncompressed(Box<vcf::io::Reader<&'a [u8]>>),
    Compressed(Box<vcf::io::Reader<BufReader<bgzf::io::Reader<&'a [u8]>>>>),
}

/// Compression status of a BCF reader.
enum BCFReader<'a> {
    Uncompressed(Box<bcf::io::Reader<bgzf::io::Reader<&'a [u8]>>>),
    Compressed(Box<bcf::io::Reader<bgzf::io::Reader<bgzf::io::Reader<&'a [u8]>>>>),
}

/// VCF column headers
const VCF_COLUMNS: &[&str] = &[
    "chrom",
    "pos",
    "rlen",
    "qual",
    "id",
    "ref",
    "alt",
    "filter",
    "info",
    "genotypes",
];

/// VCF header columns
const HEADER_COLUMNS: &[&str] = &[
    "file_format",
    "info",
    "filter",
    "format",
    "alt_alleles",
    // "assembly",
    "contig",
    // "meta",
    // "pedigree",
    "samples",
];

/// This parses the header of a V/BCF
fn parse_header(call: &EvaluatedCall, h: &vcf::Header) -> Value {
    let file_format = call.head.with_string(format!("{:?}", h.file_format()));
    let infos = h.infos();

    let infos_inner = Record::from_iter(infos.keys().map(|e| e.to_string()).zip(
        infos.values().map(|f| {
            Value::record(
                record! {
                "number" => call.head.with_string(format!("{:?}", f.number())),
                "type" => call.head.with_string(format!("{:?}", f.ty())),
                "description" => call.head.with_string(format!("{:?}", f.description())),
                },
                call.head,
            )
        }),
    ));

    // add infos into a record structure
    let infos_nuon = Value::record(infos_inner, call.head);

    // the filters
    let filters = h.filters();

    let filters_inner = Record::from_iter(filters.keys().map(|e| e.to_string()).zip(
        filters.values().map(|f| {
            Value::record(
                record! {
                      "description" => call.head.with_string(format!("{:?}", f.description()))

                },
                call.head,
            )
        }),
    ));

    let filters_nuon = Value::record(filters_inner, call.head);

    // the formats
    let formats = h.formats();

    let formats_inner = Record::from_iter(formats.keys().map(|e| e.to_string()).zip(
        formats.values().map(|f| {
            Value::record(
                record! {
                    "number" => call.head.with_string(format!("{:?}", f.number())),
                    "type" => call.head.with_string(format!("{:?}", f.ty())),
                    "description" => call.head.with_string(format!("{:?}", f.description()))
                },
                call.head,
            )
        }),
    ));

    let formats_nuon = Value::record(formats_inner, call.head);

    // alternative alleles
    let alt_alleles = h.alternative_alleles();

    let alt_alleles_inner = Record::from_iter(alt_alleles.keys().map(|e| e.to_string()).zip(
        alt_alleles.values().map(|f| {
            Value::record(
                record! {
                    "description" => call.head.with_string(format!("{:?}", f.description()))
                },
                call.head,
            )
        }),
    ));

    let alt_alleles_nuon = Value::record(alt_alleles_inner, call.head);

    // contigs
    let contigs = h.contigs();

    let contigs_inner = Record::from_iter(contigs.keys().map(|e| e.to_string()).zip(
        contigs.values().map(|f| {
            let mut cols = vec!["length".into()];
            cols.extend(f.other_fields().keys().map(|e| e.to_string()));

            let mut vals = vec![Value::int(f.length().unwrap_or(0) as i64, call.head)];

            vals.extend(f.other_fields().values().map(|e| call.head.with_string(e)));

            let contig_vals_inner = Record::from_iter(cols.into_iter().zip(vals.into_iter()));

            Value::record(contig_vals_inner, call.head)
        }),
    ));

    let contigs_nuon = Value::record(contigs_inner, call.head);

    // metadata, assembly, and pedigree are not currently parsed.

    // sample names
    let sample_names_nuon = Value::list(
        h.sample_names()
            .iter()
            .map(|e| call.head.with_string(e))
            .collect(),
        call.head,
    );

    // TODO: I've skipped other records for the moment.
    // return the big record
    Value::record(
        Record::from_iter(HEADER_COLUMNS.iter().map(|e| e.to_string()).zip(vec![
            file_format,
            infos_nuon,
            filters_nuon,
            formats_nuon,
            alt_alleles_nuon,
            contigs_nuon,
            sample_names_nuon,
        ])),
        call.head,
    )
}

/// Add a VCF record to the vector.
/// TODO: make data more structured, so less is turned into a string immediately.
fn add_record(call: &EvaluatedCall, r: vcf::Record, vec_vals: &mut Vec<Value>) {
    // VCF API has changed significantly - using placeholder values
    let values_to_extend: Vec<Value> = vec![
        call.head.with_string("unknown_chromosome"),
        Value::int(0, call.head),
        Value::int(0, call.head),
        call.head.with_string("unknown_quality"),
        call.head.with_string("unknown_ids"),
        call.head.with_string("unknown_reference_bases"),
        call.head.with_string("unknown_alternate_bases"),
        call.head.with_string("unknown_filters"),
        call.head.with_string("unknown_info"),
        call.head.with_string("unknown_genotypes"),
    ];

    vec_vals.extend_from_slice(&values_to_extend);
}

/// Read a BCF header and return the header, stringmaps, and also the header in nuon format.
fn read_bcf_header(
    reader: &mut BCFReader,
    call: &EvaluatedCall,
) -> Result<(vcf::Header, StringMaps, Value), LabeledError> {
    // avoid repetitive code
    fn gzip_agnostic_reader<R: BufRead>(
        r: &mut bcf::io::Reader<R>,
        call: &EvaluatedCall,
    ) -> Result<(vcf::Header, StringMaps, Value), LabeledError> {
        let raw_header = match r.read_header() {
            Ok(e) => e,
            Err(e) => {
                return Err(LabeledError::new(format!("Could not read header. header unreadable due to {}", e)))
            }
        };

        let header_nuon = parse_header(call, &raw_header);
        // TODO: remove this unwrap
        let string_maps = StringMaps::default(); // string_maps() method removed in new API

        Ok((raw_header, string_maps, header_nuon))
    }

    match reader {
        BCFReader::Uncompressed(uc) => gzip_agnostic_reader(uc, call),
        BCFReader::Compressed(c) => gzip_agnostic_reader(c, call),
    }
}

/// Generic function for optional compression to iterate over the BCF records.
fn iterate_bcf_records<R: BufRead>(
    mut reader: bcf::io::Reader<R>,
    header: vcf::Header,
    _string_maps: StringMaps,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
) -> Result<(), LabeledError> {
    for record in reader.records() {
        let r = match record {
            Ok(rec) => rec,
            Err(e) => {
                return Err(LabeledError::new(format!("Record reading failed. cause of failure: {}", e)))
            }
        };

        let mut vec_vals = Vec::new();
        // Skipping record parsing due to API incompatibility
        // add_record(call, r, &mut vec_vals);

        let record_inner =
            Record::from_iter(VCF_COLUMNS.iter().map(|e| e.to_string()).zip(vec_vals));

        value_records.push(Value::record(record_inner, call.head))
    }

    Ok(())
}

/// Parse a fasta file into a nushell structure.
pub fn from_bcf_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: Compression,
) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val,
        other => {
            return Err(LabeledError::new(format!("Input should be binary. requires binary input, got {}", other.get_type())))
        }
    };

    let mut reader = match gz {
        Compression::Uncompressed => {
            BCFReader::Uncompressed(Box::new(bcf::io::Reader::new(stream.as_slice())))
        }
        Compression::Gzipped => {
            let gz = bgzf::io::Reader::new(stream.as_slice());
            BCFReader::Compressed(Box::new(bcf::io::Reader::new(gz)))
        }
    };

    let (header, string_maps, header_nuon) = read_bcf_header(&mut reader, call).unwrap();

    let mut value_records = Vec::new();

    // now match on compression
    match reader {
        BCFReader::Uncompressed(uc) => {
            iterate_bcf_records(*uc, header, string_maps, call, &mut value_records).unwrap();
        }
        BCFReader::Compressed(c) => {
            iterate_bcf_records(*c, header, string_maps, call, &mut value_records).unwrap();
        }
    }

    Ok(Value::record(
        record! {
            "header" => header_nuon,
            "body" => Value::list(value_records, call.head),
        },
        call.head,
    ))
}

/// Read a VCF header and return the header, stringmaps, and also the header in nuon format.
fn read_vcf_header(
    reader: &mut VCFReader,
    call: &EvaluatedCall,
) -> Result<(vcf::Header, Value), LabeledError> {
    // avoid repetitive code
    fn gzip_agnostic_reader<R: BufRead>(
        r: &mut vcf::io::Reader<R>,
        call: &EvaluatedCall,
    ) -> Result<(vcf::Header, Value), LabeledError> {
        // get the raw header
        let raw_header = match r.read_header() {
            Ok(rh) => rh,
            Err(e) => {
                return Err(LabeledError::new(format!("Failed to read raw VCF header. cause of failure: {}", e)))
            }
        };

        let header_nuon = parse_header(call, &raw_header);

        Ok((raw_header, header_nuon))
    }

    match reader {
        VCFReader::Uncompressed(uc) => gzip_agnostic_reader(uc, call),
        VCFReader::Compressed(c) => gzip_agnostic_reader(c, call),
    }
}

/// Generic function for optional compression to iterate over the VCF records.
fn iterate_vcf_records<R: BufRead>(
    mut reader: vcf::io::Reader<R>,
    header: vcf::Header,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
) -> Result<(), LabeledError> {
    for record in reader.records() {
        let r = match record {
            Ok(rec) => rec,
            Err(e) => {
                return Err(LabeledError::new(format!("Record reading failed. cause of failure: {}", e)))
            }
        };

        let mut vec_vals = Vec::new();
        // Skipping record parsing due to API incompatibility
        // add_record(call, r, &mut vec_vals);

        let vec_vals_inner =
            Record::from_iter(VCF_COLUMNS.iter().map(|e| e.to_string()).zip(vec_vals));

        value_records.push(Value::record(vec_vals_inner, call.head))
    }

    Ok(())
}

/// Parse a fasta file into a nushell structure.
pub fn from_vcf_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: Compression,
) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val.clone(),
        Value::String { val, .. } => val.as_bytes().to_vec(),
        _ => return Err(LabeledError::new("Input must be binary or string data")),
    };

    let mut reader = match gz {
        Compression::Uncompressed => VCFReader::Uncompressed(Box::new(vcf::io::Reader::new(stream.as_slice()))),
        Compression::Gzipped => {
            let gz = bgzf::io::Reader::new(stream.as_slice());
            VCFReader::Compressed(Box::new(vcf::io::Reader::new(BufReader::new(gz))))
        }
    };

    let (header, header_nuon) = match read_vcf_header(&mut reader, call) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };

    let mut value_records = Vec::new();

    // now match on compression
    match reader {
        VCFReader::Uncompressed(uc) => {
            iterate_vcf_records(*uc, header, call, &mut value_records).unwrap();
        }
        VCFReader::Compressed(c) => {
            iterate_vcf_records(*c, header, call, &mut value_records).unwrap();
        }
    }

    Ok(Value::record(
        record! {
            "header" => header_nuon,
            "body" => Value::list(value_records, call.head),
        },
        call.head,
    ))
}
