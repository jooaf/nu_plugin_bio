use crate::bio_format::Compression;
use crate::bio::Bio;
use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, SimplePluginCommand};
use nu_protocol::LabeledError;
use nu_protocol::{Category, Signature, Type, Value};

pub struct BioPlugin;

impl Plugin for BioPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FromFasta),
            Box::new(FromFastaGz),
            Box::new(FromFa),
            Box::new(FromFaGz),
            Box::new(FromFastq),
            Box::new(FromFastqGz),
            Box::new(FromFq),
            Box::new(FromFqGz),
            Box::new(ToFasta),
            Box::new(ToFastq),
            Box::new(FromBam),
            Box::new(FromSam),
            Box::new(FromCram),
            Box::new(FromBcf),
            Box::new(FromBcfGz),
            Box::new(FromVcf),
            Box::new(FromVcfGz),
            Box::new(FromGff),
            Box::new(FromGfa),
            Box::new(FromGfaGz),
            Box::new(FromBed),
        ]
    }
}

pub struct FromFasta;

impl SimplePluginCommand for FromFasta {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fasta"
    }

    fn description(&self) -> &str {
        "Parse a fasta file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fasta header description",
                Some('d'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fasta(call, input, Compression::Uncompressed)
    }
}

pub struct FromFastaGz;

impl SimplePluginCommand for FromFastaGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fasta.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped fasta file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fasta header description",
                Some('d'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fasta(call, input, Compression::Gzipped)
    }
}

pub struct FromFa;

impl SimplePluginCommand for FromFa {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fa"
    }

    fn description(&self) -> &str {
        "Parse a fasta file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fasta header description",
                Some('d'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fasta(call, input, Compression::Uncompressed)
    }
}

pub struct FromFaGz;

impl SimplePluginCommand for FromFaGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fa.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped fasta file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fasta header description",
                Some('d'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fasta(call, input, Compression::Gzipped)
    }
}

pub struct FromFastq;

impl SimplePluginCommand for FromFastq {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fastq"
    }

    fn description(&self) -> &str {
        "Parse a fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fastq(call, input, Compression::Uncompressed)
    }
}

pub struct FromFastqGz;

impl SimplePluginCommand for FromFastqGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fastq.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fastq(call, input, Compression::Gzipped)
    }
}

pub struct FromFq;

impl SimplePluginCommand for FromFq {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fq"
    }

    fn description(&self) -> &str {
        "Parse a fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fastq(call, input, Compression::Uncompressed)
    }
}

pub struct FromFqGz;

impl SimplePluginCommand for FromFqGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from fq.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_fastq(call, input, Compression::Gzipped)
    }
}

pub struct ToFasta;

impl SimplePluginCommand for ToFasta {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "to fasta"
    }

    fn description(&self) -> &str {
        "Print a parsed fasta object to a string"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Table(vec![].into()), Type::String)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.to_fasta(call, input)
    }
}

pub struct ToFastq;

impl SimplePluginCommand for ToFastq {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "to fastq"
    }

    fn description(&self) -> &str {
        "Print out a fastq from structured nuon"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Table(vec![].into()), Type::String)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.to_fastq(call, input)
    }
}

pub struct FromBam;

impl SimplePluginCommand for FromBam {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from bam"
    }

    fn description(&self) -> &str {
        "Parse a BAM file.\nReturns a record containing the header and the body of the BAM file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_bam(call, input)
    }
}

pub struct FromSam;

impl SimplePluginCommand for FromSam {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from sam"
    }

    fn description(&self) -> &str {
        "Parse a SAM file.\nReturns a record containing the header and the body of the SAM file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_sam(call, input)
    }
}

pub struct FromCram;

impl SimplePluginCommand for FromCram {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from cram"
    }

    fn description(&self) -> &str {
        "Parse a CRAM file into SAM output.\nReturns a record containing the header and the body of the CRAM file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_cram(call, input)
    }
}

pub struct FromBcf;

impl SimplePluginCommand for FromBcf {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from bcf"
    }

    fn description(&self) -> &str {
        "Parse a BCF file.\nReturns a record containing the header and the body of the BCF file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_bcf(call, input, Compression::Uncompressed)
    }
}

pub struct FromBcfGz;

impl SimplePluginCommand for FromBcfGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from bcf.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped BCF file.\nReturns a record containing the header and the body of the BCF file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_bcf(call, input, Compression::Gzipped)
    }
}

pub struct FromVcf;

impl SimplePluginCommand for FromVcf {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from vcf"
    }

    fn description(&self) -> &str {
        "Parse a VCF file.\nReturns a record containing the header and the body of the VCF file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_vcf(call, input, Compression::Uncompressed)
    }
}

pub struct FromVcfGz;

impl SimplePluginCommand for FromVcfGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from vcf.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped VCF file.\nReturns a record containing the header and the body of the VCF file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_vcf(call, input, Compression::Gzipped)
    }
}

pub struct FromGff;

impl SimplePluginCommand for FromGff {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from gff"
    }

    fn description(&self) -> &str {
        "Parse a GFF file.\nReturns a table."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_gff(call, input)
    }
}

pub struct FromGfa;

impl SimplePluginCommand for FromGfa {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from gfa"
    }

    fn description(&self) -> &str {
        "Parse a GFA file.\nReturns a record containing the header, segments, links, containments, and paths."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_gfa(call, input, Compression::Uncompressed)
    }
}

pub struct FromGfaGz;

impl SimplePluginCommand for FromGfaGz {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from gfa.gz"
    }

    fn description(&self) -> &str {
        "Parse a gzipped GFA file.\nReturns a record containing the header, segments, links, containments, and paths."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_gfa(call, input, Compression::Gzipped)
    }
}

pub struct FromBed;

impl SimplePluginCommand for FromBed {
    type Plugin = BioPlugin;

    fn name(&self) -> &str {
        "from bed"
    }

    fn description(&self) -> &str {
        "Parse a BED file."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Binary, Type::Table(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        _plugin: &BioPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let bio = Bio;
        bio.from_bed(call, input.clone())
    }
}
