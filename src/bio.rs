use crate::bio_format::bam::{from_bam_inner, from_sam_inner};
use crate::bio_format::bcf::{from_bcf_inner, from_vcf_inner};
use crate::bio_format::bed::from_bed_inner;
use crate::bio_format::cram::from_cram_inner;
use crate::bio_format::fasta::{from_fasta_inner, from_fastq_inner, nuon_to_fasta, nuon_to_fastq};
use crate::bio_format::gfa::from_gfa_inner;
use crate::bio_format::gff::from_gff_inner;
use crate::bio_format::Compression;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use nu_protocol::Value;

/// We implement a bunch of parsers on the `Bio` struct.
pub struct Bio;

impl Bio {
    /// Parsing a fasta into Nushell.
    pub fn from_fasta(
        &self,
        call: &EvaluatedCall,
        input: &Value,
        gz: Compression,
    ) -> Result<Value, LabeledError> {
        let value_records = from_fasta_inner(call, input, gz)?;

        Ok(Value::list(value_records, call.head))
    }

    pub fn to_fasta(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        nuon_to_fasta(call, input)
    }

    /// Parsing a fastq into Nushell.
    pub fn from_fastq(
        &self,
        call: &EvaluatedCall,
        input: &Value,
        gz: Compression,
    ) -> Result<Value, LabeledError> {
        let value_records = from_fastq_inner(call, input, gz)?;
        Ok(Value::list(value_records, call.head))
    }

    /// Structured data to fastq
    pub fn to_fastq(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        nuon_to_fastq(call, input)
    }

    /// These B(S)AM functions are quite slow at the moment.
    pub fn from_bam(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        from_bam_inner(call, input)
    }
    /// These B(S)AM functions are quite slow at the moment.
    pub fn from_sam(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        from_sam_inner(call, input)
    }

    /// Parse a CRAM file.
    pub fn from_cram(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        from_cram_inner(call, input)
    }

    /// Parse a BCF.
    pub fn from_bcf(
        &self,
        call: &EvaluatedCall,
        input: &Value,
        gz: Compression,
    ) -> Result<Value, LabeledError> {
        from_bcf_inner(call, input, gz)
    }
    /// Parse a VCF.
    pub fn from_vcf(
        &self,
        call: &EvaluatedCall,
        input: &Value,
        gz: Compression,
    ) -> Result<Value, LabeledError> {
        from_vcf_inner(call, input, gz)
    }

    /// Parse a GFF.
    pub fn from_gff(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let value_records = from_gff_inner(call, input)?;
        Ok(Value::list(value_records, call.head))
    }

    /// Parse a GFA.
    pub fn from_gfa(
        &self,
        call: &EvaluatedCall,
        input: &Value,
        gz: Compression,
    ) -> Result<Value, LabeledError> {
        from_gfa_inner(call, input, gz)
    }

    /// Parse a BED.
    pub fn from_bed(&self, call: &EvaluatedCall, input: Value) -> Result<Value, LabeledError> {
        from_bed_inner(call, input).map(|e| Value::list(e, call.head))
    }
}
