#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///The `BenchmarkKind`, differentiating between library and binary benchmarks
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `BenchmarkKind`, differentiating between library and binary
/// benchmarks",
///  "oneOf": [
///    {
///      "description": "A library benchmark",
///      "type": "string",
///      "const": "LibraryBenchmark"
///    },
///    {
///      "description": "A binary benchmark",
///      "type": "string",
///      "const": "BinaryBenchmark"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BenchmarkKind {
    ///A library benchmark
    LibraryBenchmark,
    ///A binary benchmark
    BinaryBenchmark,
}
impl ::std::convert::From<&Self> for BenchmarkKind {
    fn from(value: &BenchmarkKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BenchmarkKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LibraryBenchmark => f.write_str("LibraryBenchmark"),
            Self::BinaryBenchmark => f.write_str("BinaryBenchmark"),
        }
    }
}
impl ::std::str::FromStr for BenchmarkKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LibraryBenchmark" => Ok(Self::LibraryBenchmark),
            "BinaryBenchmark" => Ok(Self::BinaryBenchmark),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BenchmarkKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BenchmarkKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BenchmarkKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The `BenchmarkSummary` containing all the information of a single benchmark run
///
///This includes produced files, recorded callgrind events, performance regressions ...
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "title": "BenchmarkSummary",
///  "description": "The `BenchmarkSummary` containing all the information of a single
/// benchmark run\n\nThis includes produced files, recorded callgrind events, performance
/// regressions ...",
///  "type": "object",
///  "required": [
///    "baselines",
///    "benchmark_exe",
///    "benchmark_file",
///    "function_name",
///    "kind",
///    "module_path",
///    "package_dir",
///    "profiles",
///    "project_root",
///    "version"
///  ],
///  "properties": {
///    "baselines": {
///      "description": "The baselines if any. An absent first baseline indicates that new
/// output was produced. An\nabsent second baseline indicates the usage of the usual \"*.old\"
/// output.",
///      "type": "array",
///      "items": [
///        {
///          "type": [
///            "string",
///            "null"
///          ]
///        },
///        {
///          "type": [
///            "string",
///            "null"
///          ]
///        }
///      ],
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "benchmark_exe": {
///      "description": "The path to the binary which is executed by valgrind. In case of a
/// library benchmark this\nis the compiled benchmark file. In case of a binary benchmark this
/// is the path to the\ncommand.",
///      "type": "string"
///    },
///    "benchmark_file": {
///      "description": "The path to the benchmark file",
///      "type": "string"
///    },
///    "details": {
///      "description": "More details describing this benchmark run",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "function_name": {
///      "description": "The name of the function under test",
///      "type": "string"
///    },
///    "id": {
///      "description": "The user provided id of this benchmark",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "kind": {
///      "description": "Whether this summary describes a library or binary benchmark",
///      "allOf": [
///        {
///          "$ref": "#/definitions/BenchmarkKind"
///        }
///      ]
///    },
///    "module_path": {
///      "description": "The rust path in the form `bench_file::group::bench`",
///      "type": "string"
///    },
///    "package_dir": {
///      "description": "The directory of the package",
///      "type": "string"
///    },
///    "profiles": {
///      "description": "The summary of other valgrind tool runs",
///      "allOf": [
///        {
///          "$ref": "#/definitions/Profiles"
///        }
///      ]
///    },
///    "project_root": {
///      "description": "The project's root directory",
///      "type": "string"
///    },
///    "summary_output": {
///      "description": "The destination and kind of the summary file",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/SummaryOutput"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "version": {
///      "description": "The version of this format. Only backwards incompatible changes cause
/// an increase of the\nversion",
///      "type": "string"
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BenchmarkSummary {
    ///The baselines if any. An absent first baseline indicates that new output was produced.
    /// An absent second baseline indicates the usage of the usual "*.old" output.
    pub baselines: (
        ::std::option::Option<::std::string::String>,
        ::std::option::Option<::std::string::String>,
    ),
    ///The path to the binary which is executed by valgrind. In case of a library benchmark
    /// this is the compiled benchmark file. In case of a binary benchmark this is the path
    /// to the command.
    pub benchmark_exe: ::std::string::String,
    ///The path to the benchmark file
    pub benchmark_file: ::std::string::String,
    ///More details describing this benchmark run
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<::std::string::String>,
    ///The name of the function under test
    pub function_name: ::std::string::String,
    ///The user provided id of this benchmark
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///Whether this summary describes a library or binary benchmark
    pub kind: BenchmarkKind,
    ///The rust path in the form `bench_file::group::bench`
    pub module_path: ::std::string::String,
    ///The directory of the package
    pub package_dir: ::std::string::String,
    ///The summary of other valgrind tool runs
    pub profiles: Profiles,
    ///The project's root directory
    pub project_root: ::std::string::String,
    ///The destination and kind of the summary file
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary_output: ::std::option::Option<SummaryOutput>,
    ///The version of this format. Only backwards incompatible changes cause an increase of
    /// the version
    pub version: ::std::string::String,
}
impl ::std::convert::From<&BenchmarkSummary> for BenchmarkSummary {
    fn from(value: &BenchmarkSummary) -> Self {
        value.clone()
    }
}
impl BenchmarkSummary {
    pub fn builder() -> builder::BenchmarkSummary {
        Default::default()
    }
}
///All metrics which cachegrind produces and additionally some derived events
///
///Depending on the options passed to Cachegrind, these are the events that Cachegrind can
/// produce. See the [Cachegrind
///documentation](https://valgrind.org/docs/manual/cg-manual.html#cg-manual.cgopts) for details.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "All metrics which cachegrind produces and additionally some derived events\n\nDepending on the options passed to Cachegrind, these are the events that Cachegrind can produce.\nSee the [Cachegrind\ndocumentation](https://valgrind.org/docs/manual/cg-manual.html#cg-manual.cgopts) for details.",
///  "oneOf": [
///    {
///      "description": "The default event. I cache reads (which equals the number of
/// instructions executed)",
///      "type": "string",
///      "const": "Ir"
///    },
///    {
///      "description": "D Cache reads (which equals the number of memory reads)
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "Dr"
///    },
///    {
///      "description": "D Cache writes (which equals the number of memory writes)
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "Dw"
///    },
///    {
///      "description": "I1 cache read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "I1mr"
///    },
///    {
///      "description": "D1 cache read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1mr"
///    },
///    {
///      "description": "D1 cache write misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1mw"
///    },
///    {
///      "description": "LL cache instruction read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "ILmr"
///    },
///    {
///      "description": "LL cache data read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "DLmr"
///    },
///    {
///      "description": "LL cache data write misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "DLmw"
///    },
///    {
///      "description": "I1 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "I1MissRate"
///    },
///    {
///      "description": "LL/L2 instructions cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLiMissRate"
///    },
///    {
///      "description": "D1 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1MissRate"
///    },
///    {
///      "description": "LL/L2 data cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLdMissRate"
///    },
///    {
///      "description": "LL/L2 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLMissRate"
///    },
///    {
///      "description": "Derived event showing the L1 hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "L1hits"
///    },
///    {
///      "description": "Derived event showing the LL hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLhits"
///    },
///    {
///      "description": "Derived event showing the RAM hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "RamHits"
///    },
///    {
///      "description": "L1 cache hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "L1HitRate"
///    },
///    {
///      "description": "LL/L2 cache hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLHitRate"
///    },
///    {
///      "description": "RAM hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "RamHitRate"
///    },
///    {
///      "description": "Derived event showing the total amount of cache reads and writes
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "TotalRW"
///    },
///    {
///      "description": "Derived event showing estimated CPU cycles (--cache-sim=yes)",
///      "type": "string",
///      "const": "EstimatedCycles"
///    },
///    {
///      "description": "Conditional branches executed (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bc"
///    },
///    {
///      "description": "Conditional branches mispredicted (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bcm"
///    },
///    {
///      "description": "Indirect branches executed (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bi"
///    },
///    {
///      "description": "Indirect branches mispredicted (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bim"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CachegrindMetric {
    ///The default event. I cache reads (which equals the number of instructions executed)
    Ir,
    ///D Cache reads (which equals the number of memory reads) (--cache-sim=yes)
    Dr,
    ///D Cache writes (which equals the number of memory writes) (--cache-sim=yes)
    Dw,
    ///I1 cache read misses (--cache-sim=yes)
    I1mr,
    ///D1 cache read misses (--cache-sim=yes)
    D1mr,
    ///D1 cache write misses (--cache-sim=yes)
    D1mw,
    ///LL cache instruction read misses (--cache-sim=yes)
    ILmr,
    ///LL cache data read misses (--cache-sim=yes)
    DLmr,
    ///LL cache data write misses (--cache-sim=yes)
    DLmw,
    ///I1 cache miss rate (--cache-sim=yes)
    I1MissRate,
    ///LL/L2 instructions cache miss rate (--cache-sim=yes)
    LLiMissRate,
    ///D1 cache miss rate (--cache-sim=yes)
    D1MissRate,
    ///LL/L2 data cache miss rate (--cache-sim=yes)
    LLdMissRate,
    ///LL/L2 cache miss rate (--cache-sim=yes)
    #[serde(rename = "LLMissRate")]
    LlMissRate,
    ///Derived event showing the L1 hits (--cache-sim=yes)
    L1hits,
    ///Derived event showing the LL hits (--cache-sim=yes)
    LLhits,
    ///Derived event showing the RAM hits (--cache-sim=yes)
    RamHits,
    ///L1 cache hit rate (--cache-sim=yes)
    L1HitRate,
    ///LL/L2 cache hit rate (--cache-sim=yes)
    #[serde(rename = "LLHitRate")]
    LlHitRate,
    ///RAM hit rate (--cache-sim=yes)
    RamHitRate,
    ///Derived event showing the total amount of cache reads and writes (--cache-sim=yes)
    #[serde(rename = "TotalRW")]
    TotalRw,
    ///Derived event showing estimated CPU cycles (--cache-sim=yes)
    EstimatedCycles,
    ///Conditional branches executed (--branch-sim=yes)
    Bc,
    ///Conditional branches mispredicted (--branch-sim=yes)
    Bcm,
    ///Indirect branches executed (--branch-sim=yes)
    Bi,
    ///Indirect branches mispredicted (--branch-sim=yes)
    Bim,
}
impl ::std::convert::From<&Self> for CachegrindMetric {
    fn from(value: &CachegrindMetric) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CachegrindMetric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ir => f.write_str("Ir"),
            Self::Dr => f.write_str("Dr"),
            Self::Dw => f.write_str("Dw"),
            Self::I1mr => f.write_str("I1mr"),
            Self::D1mr => f.write_str("D1mr"),
            Self::D1mw => f.write_str("D1mw"),
            Self::ILmr => f.write_str("ILmr"),
            Self::DLmr => f.write_str("DLmr"),
            Self::DLmw => f.write_str("DLmw"),
            Self::I1MissRate => f.write_str("I1MissRate"),
            Self::LLiMissRate => f.write_str("LLiMissRate"),
            Self::D1MissRate => f.write_str("D1MissRate"),
            Self::LLdMissRate => f.write_str("LLdMissRate"),
            Self::LlMissRate => f.write_str("LLMissRate"),
            Self::L1hits => f.write_str("L1hits"),
            Self::LLhits => f.write_str("LLhits"),
            Self::RamHits => f.write_str("RamHits"),
            Self::L1HitRate => f.write_str("L1HitRate"),
            Self::LlHitRate => f.write_str("LLHitRate"),
            Self::RamHitRate => f.write_str("RamHitRate"),
            Self::TotalRw => f.write_str("TotalRW"),
            Self::EstimatedCycles => f.write_str("EstimatedCycles"),
            Self::Bc => f.write_str("Bc"),
            Self::Bcm => f.write_str("Bcm"),
            Self::Bi => f.write_str("Bi"),
            Self::Bim => f.write_str("Bim"),
        }
    }
}
impl ::std::str::FromStr for CachegrindMetric {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Ir" => Ok(Self::Ir),
            "Dr" => Ok(Self::Dr),
            "Dw" => Ok(Self::Dw),
            "I1mr" => Ok(Self::I1mr),
            "D1mr" => Ok(Self::D1mr),
            "D1mw" => Ok(Self::D1mw),
            "ILmr" => Ok(Self::ILmr),
            "DLmr" => Ok(Self::DLmr),
            "DLmw" => Ok(Self::DLmw),
            "I1MissRate" => Ok(Self::I1MissRate),
            "LLiMissRate" => Ok(Self::LLiMissRate),
            "D1MissRate" => Ok(Self::D1MissRate),
            "LLdMissRate" => Ok(Self::LLdMissRate),
            "LLMissRate" => Ok(Self::LlMissRate),
            "L1hits" => Ok(Self::L1hits),
            "LLhits" => Ok(Self::LLhits),
            "RamHits" => Ok(Self::RamHits),
            "L1HitRate" => Ok(Self::L1HitRate),
            "LLHitRate" => Ok(Self::LlHitRate),
            "RamHitRate" => Ok(Self::RamHitRate),
            "TotalRW" => Ok(Self::TotalRw),
            "EstimatedCycles" => Ok(Self::EstimatedCycles),
            "Bc" => Ok(Self::Bc),
            "Bcm" => Ok(Self::Bcm),
            "Bi" => Ok(Self::Bi),
            "Bim" => Ok(Self::Bim),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CachegrindMetric {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CachegrindMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CachegrindMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The metrics collected by DHAT
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The metrics collected by DHAT",
///  "oneOf": [
///    {
///      "description": "In ad-hoc mode, Total units measured over the entire execution",
///      "type": "string",
///      "const": "TotalUnits"
///    },
///    {
///      "description": "Total ad-hoc events over the entire execution",
///      "type": "string",
///      "const": "TotalEvents"
///    },
///    {
///      "description": "Total bytes allocated over the entire execution",
///      "type": "string",
///      "const": "TotalBytes"
///    },
///    {
///      "description": "Total heap blocks allocated over the entire execution",
///      "type": "string",
///      "const": "TotalBlocks"
///    },
///    {
///      "description": "The bytes alive at t-gmax, the time when the heap size reached its
/// global maximum",
///      "type": "string",
///      "const": "AtTGmaxBytes"
///    },
///    {
///      "description": "The blocks alive at t-gmax",
///      "type": "string",
///      "const": "AtTGmaxBlocks"
///    },
///    {
///      "description": "The amount of bytes at the end of the execution.\n\nThis is the amount
/// of bytes which were not explicitly freed.",
///      "type": "string",
///      "const": "AtTEndBytes"
///    },
///    {
///      "description": "The amount of blocks at the end of the execution.\n\nThis is the
/// amount of heap blocks which were not explicitly freed.",
///      "type": "string",
///      "const": "AtTEndBlocks"
///    },
///    {
///      "description": "The amount of bytes read during the entire execution",
///      "type": "string",
///      "const": "ReadsBytes"
///    },
///    {
///      "description": "The amount of bytes written during the entire execution",
///      "type": "string",
///      "const": "WritesBytes"
///    },
///    {
///      "description": "The total lifetimes of all heap blocks allocated",
///      "type": "string",
///      "const": "TotalLifetimes"
///    },
///    {
///      "description": "The maximum amount of bytes",
///      "type": "string",
///      "const": "MaximumBytes"
///    },
///    {
///      "description": "The maximum amount of heap blocks",
///      "type": "string",
///      "const": "MaximumBlocks"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DhatMetric {
    ///In ad-hoc mode, Total units measured over the entire execution
    TotalUnits,
    ///Total ad-hoc events over the entire execution
    TotalEvents,
    ///Total bytes allocated over the entire execution
    TotalBytes,
    ///Total heap blocks allocated over the entire execution
    TotalBlocks,
    ///The bytes alive at t-gmax, the time when the heap size reached its global maximum
    AtTGmaxBytes,
    ///The blocks alive at t-gmax
    AtTGmaxBlocks,
    ///The amount of bytes at the end of the execution.
    ///
    ///This is the amount of bytes which were not explicitly freed.
    AtTEndBytes,
    ///The amount of blocks at the end of the execution.
    ///
    ///This is the amount of heap blocks which were not explicitly freed.
    AtTEndBlocks,
    ///The amount of bytes read during the entire execution
    ReadsBytes,
    ///The amount of bytes written during the entire execution
    WritesBytes,
    ///The total lifetimes of all heap blocks allocated
    TotalLifetimes,
    ///The maximum amount of bytes
    MaximumBytes,
    ///The maximum amount of heap blocks
    MaximumBlocks,
}
impl ::std::convert::From<&Self> for DhatMetric {
    fn from(value: &DhatMetric) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DhatMetric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TotalUnits => f.write_str("TotalUnits"),
            Self::TotalEvents => f.write_str("TotalEvents"),
            Self::TotalBytes => f.write_str("TotalBytes"),
            Self::TotalBlocks => f.write_str("TotalBlocks"),
            Self::AtTGmaxBytes => f.write_str("AtTGmaxBytes"),
            Self::AtTGmaxBlocks => f.write_str("AtTGmaxBlocks"),
            Self::AtTEndBytes => f.write_str("AtTEndBytes"),
            Self::AtTEndBlocks => f.write_str("AtTEndBlocks"),
            Self::ReadsBytes => f.write_str("ReadsBytes"),
            Self::WritesBytes => f.write_str("WritesBytes"),
            Self::TotalLifetimes => f.write_str("TotalLifetimes"),
            Self::MaximumBytes => f.write_str("MaximumBytes"),
            Self::MaximumBlocks => f.write_str("MaximumBlocks"),
        }
    }
}
impl ::std::str::FromStr for DhatMetric {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TotalUnits" => Ok(Self::TotalUnits),
            "TotalEvents" => Ok(Self::TotalEvents),
            "TotalBytes" => Ok(Self::TotalBytes),
            "TotalBlocks" => Ok(Self::TotalBlocks),
            "AtTGmaxBytes" => Ok(Self::AtTGmaxBytes),
            "AtTGmaxBlocks" => Ok(Self::AtTGmaxBlocks),
            "AtTEndBytes" => Ok(Self::AtTEndBytes),
            "AtTEndBlocks" => Ok(Self::AtTEndBlocks),
            "ReadsBytes" => Ok(Self::ReadsBytes),
            "WritesBytes" => Ok(Self::WritesBytes),
            "TotalLifetimes" => Ok(Self::TotalLifetimes),
            "MaximumBytes" => Ok(Self::MaximumBytes),
            "MaximumBlocks" => Ok(Self::MaximumBlocks),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DhatMetric {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DhatMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DhatMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The differences between two `Metrics` as percentage and factor
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The differences between two `Metrics` as percentage and factor",
///  "type": "object",
///  "required": [
///    "diff_pct",
///    "factor"
///  ],
///  "properties": {
///    "diff_pct": {
///      "description": "The percentage of the difference between two `Metrics` serialized as
/// string to preserve\ninfinity values and avoid `null` in json",
///      "type": "string"
///    },
///    "factor": {
///      "description": "The factor of the difference between two `Metrics` serialized as
/// string to preserve\ninfinity values and void `null` in json",
///      "type": "string"
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Diffs {
    ///The percentage of the difference between two `Metrics` serialized as string to preserve
    ///infinity values and avoid `null` in json
    pub diff_pct: ::std::string::String,
    ///The factor of the difference between two `Metrics` serialized as string to preserve
    ///infinity values and void `null` in json
    pub factor: ::std::string::String,
}
impl ::std::convert::From<&Diffs> for Diffs {
    fn from(value: &Diffs) -> Self {
        value.clone()
    }
}
impl Diffs {
    pub fn builder() -> builder::Diffs {
        Default::default()
    }
}
///Represent values that have either a `Left` or `Right` value or `Both` values
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Represent values that have either a `Left` or `Right` value or `Both`
/// values",
///  "oneOf": [
///    {
///      "description": "Represents a value from both sides",
///      "type": "object",
///      "required": [
///        "Both"
///      ],
///      "properties": {
///        "Both": {
///          "type": "array",
///          "items": [
///            {
///              "$ref": "#/definitions/ProfileInfo"
///            },
///            {
///              "$ref": "#/definitions/ProfileInfo"
///            }
///          ],
///          "maxItems": 2,
///          "minItems": 2
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Represents a value from the left side",
///      "type": "object",
///      "required": [
///        "Left"
///      ],
///      "properties": {
///        "Left": {
///          "$ref": "#/definitions/ProfileInfo"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Represents a value from the right side",
///      "type": "object",
///      "required": [
///        "Right"
///      ],
///      "properties": {
///        "Right": {
///          "$ref": "#/definitions/ProfileInfo"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum EitherOrBoth {
    ///Represents a value from both sides
    Both(ProfileInfo, ProfileInfo),
    ///Represents a value from the left side
    Left(ProfileInfo),
    ///Represents a value from the right side
    Right(ProfileInfo),
}
impl ::std::convert::From<&Self> for EitherOrBoth {
    fn from(value: &EitherOrBoth) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<(ProfileInfo, ProfileInfo)> for EitherOrBoth {
    fn from(value: (ProfileInfo, ProfileInfo)) -> Self {
        Self::Both(value.0, value.1)
    }
}
///Represent values that have either a `Left` or `Right` value or `Both` values
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Represent values that have either a `Left` or `Right` value or `Both`
/// values",
///  "oneOf": [
///    {
///      "description": "Represents a value from both sides",
///      "type": "object",
///      "required": [
///        "Both"
///      ],
///      "properties": {
///        "Both": {
///          "type": "array",
///          "items": [
///            {
///              "$ref": "#/definitions/Metric"
///            },
///            {
///              "$ref": "#/definitions/Metric"
///            }
///          ],
///          "maxItems": 2,
///          "minItems": 2
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Represents a value from the left side",
///      "type": "object",
///      "required": [
///        "Left"
///      ],
///      "properties": {
///        "Left": {
///          "$ref": "#/definitions/Metric"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Represents a value from the right side",
///      "type": "object",
///      "required": [
///        "Right"
///      ],
///      "properties": {
///        "Right": {
///          "$ref": "#/definitions/Metric"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum EitherOrBoth2 {
    ///Represents a value from both sides
    Both(Metric, Metric),
    ///Represents a value from the left side
    Left(Metric),
    ///Represents a value from the right side
    Right(Metric),
}
impl ::std::convert::From<&Self> for EitherOrBoth2 {
    fn from(value: &EitherOrBoth2) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<(Metric, Metric)> for EitherOrBoth2 {
    fn from(value: (Metric, Metric)) -> Self {
        Self::Both(value.0, value.1)
    }
}
///The error metrics from a tool which reports errors
///
///The tools which report only errors are `helgrind`, `drd` and `memcheck`. The order in which
/// the variants are defined in this enum determines the order of the metrics in the benchmark
/// terminal output.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The error metrics from a tool which reports errors\n\nThe tools which
/// report only errors are `helgrind`, `drd` and `memcheck`. The order in which the\nvariants
/// are defined in this enum determines the order of the metrics in the benchmark
/// terminal\noutput.",
///  "oneOf": [
///    {
///      "description": "The amount of detected unsuppressed errors",
///      "type": "string",
///      "const": "Errors"
///    },
///    {
///      "description": "The amount of detected unsuppressed error contexts",
///      "type": "string",
///      "const": "Contexts"
///    },
///    {
///      "description": "The amount of suppressed errors",
///      "type": "string",
///      "const": "SuppressedErrors"
///    },
///    {
///      "description": "The amount of suppressed error contexts",
///      "type": "string",
///      "const": "SuppressedContexts"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ErrorMetric {
    ///The amount of detected unsuppressed errors
    Errors,
    ///The amount of detected unsuppressed error contexts
    Contexts,
    ///The amount of suppressed errors
    SuppressedErrors,
    ///The amount of suppressed error contexts
    SuppressedContexts,
}
impl ::std::convert::From<&Self> for ErrorMetric {
    fn from(value: &ErrorMetric) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ErrorMetric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Errors => f.write_str("Errors"),
            Self::Contexts => f.write_str("Contexts"),
            Self::SuppressedErrors => f.write_str("SuppressedErrors"),
            Self::SuppressedContexts => f.write_str("SuppressedContexts"),
        }
    }
}
impl ::std::str::FromStr for ErrorMetric {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Errors" => Ok(Self::Errors),
            "Contexts" => Ok(Self::Contexts),
            "SuppressedErrors" => Ok(Self::SuppressedErrors),
            "SuppressedContexts" => Ok(Self::SuppressedContexts),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ErrorMetric {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ErrorMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ErrorMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///All `EventKind`s callgrind produces and additionally some derived events
///
///Depending on the options passed to Callgrind, these are the events that Callgrind can
/// produce. See the [Callgrind
///documentation](https://valgrind.org/docs/manual/cl-manual.html#cl-manual.options) for details.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "All `EventKind`s callgrind produces and additionally some derived events\n\nDepending on the options passed to Callgrind, these are the events that Callgrind can produce.\nSee the [Callgrind\ndocumentation](https://valgrind.org/docs/manual/cl-manual.html#cl-manual.options) for details.",
///  "oneOf": [
///    {
///      "description": "The default event. I cache reads (which equals the number of
/// instructions executed)",
///      "type": "string",
///      "const": "Ir"
///    },
///    {
///      "description": "D Cache reads (which equals the number of memory reads)
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "Dr"
///    },
///    {
///      "description": "D Cache writes (which equals the number of memory writes)
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "Dw"
///    },
///    {
///      "description": "I1 cache read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "I1mr"
///    },
///    {
///      "description": "D1 cache read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1mr"
///    },
///    {
///      "description": "D1 cache write misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1mw"
///    },
///    {
///      "description": "LL cache instruction read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "ILmr"
///    },
///    {
///      "description": "LL cache data read misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "DLmr"
///    },
///    {
///      "description": "LL cache data write misses (--cache-sim=yes)",
///      "type": "string",
///      "const": "DLmw"
///    },
///    {
///      "description": "I1 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "I1MissRate"
///    },
///    {
///      "description": "LL/L2 instructions cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLiMissRate"
///    },
///    {
///      "description": "D1 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "D1MissRate"
///    },
///    {
///      "description": "LL/L2 data cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLdMissRate"
///    },
///    {
///      "description": "LL/L2 cache miss rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLMissRate"
///    },
///    {
///      "description": "Derived event showing the L1 hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "L1hits"
///    },
///    {
///      "description": "Derived event showing the LL hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLhits"
///    },
///    {
///      "description": "Derived event showing the RAM hits (--cache-sim=yes)",
///      "type": "string",
///      "const": "RamHits"
///    },
///    {
///      "description": "L1 cache hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "L1HitRate"
///    },
///    {
///      "description": "LL/L2 cache hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "LLHitRate"
///    },
///    {
///      "description": "RAM hit rate (--cache-sim=yes)",
///      "type": "string",
///      "const": "RamHitRate"
///    },
///    {
///      "description": "Derived event showing the total amount of cache reads and writes
/// (--cache-sim=yes)",
///      "type": "string",
///      "const": "TotalRW"
///    },
///    {
///      "description": "Derived event showing estimated CPU cycles (--cache-sim=yes)",
///      "type": "string",
///      "const": "EstimatedCycles"
///    },
///    {
///      "description": "The number of system calls done (--collect-systime=yes)",
///      "type": "string",
///      "const": "SysCount"
///    },
///    {
///      "description": "The elapsed time spent in system calls (--collect-systime=yes)",
///      "type": "string",
///      "const": "SysTime"
///    },
///    {
///      "description": "The cpu time spent during system calls (--collect-systime=nsec)",
///      "type": "string",
///      "const": "SysCpuTime"
///    },
///    {
///      "description": "The number of global bus events (--collect-bus=yes)",
///      "type": "string",
///      "const": "Ge"
///    },
///    {
///      "description": "Conditional branches executed (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bc"
///    },
///    {
///      "description": "Conditional branches mispredicted (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bcm"
///    },
///    {
///      "description": "Indirect branches executed (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bi"
///    },
///    {
///      "description": "Indirect branches mispredicted (--branch-sim=yes)",
///      "type": "string",
///      "const": "Bim"
///    },
///    {
///      "description": "Dirty miss because of instruction read (--simulate-wb=yes)",
///      "type": "string",
///      "const": "ILdmr"
///    },
///    {
///      "description": "Dirty miss because of data read (--simulate-wb=yes)",
///      "type": "string",
///      "const": "DLdmr"
///    },
///    {
///      "description": "Dirty miss because of data write (--simulate-wb=yes)",
///      "type": "string",
///      "const": "DLdmw"
///    },
///    {
///      "description": "Counter showing bad temporal locality for L1 caches (--cachuse=yes)",
///      "type": "string",
///      "const": "AcCost1"
///    },
///    {
///      "description": "Counter showing bad temporal locality for LL caches (--cachuse=yes)",
///      "type": "string",
///      "const": "AcCost2"
///    },
///    {
///      "description": "Counter showing bad spatial locality for L1 caches (--cachuse=yes)",
///      "type": "string",
///      "const": "SpLoss1"
///    },
///    {
///      "description": "Counter showing bad spatial locality for LL caches (--cachuse=yes)",
///      "type": "string",
///      "const": "SpLoss2"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventKind {
    ///The default event. I cache reads (which equals the number of instructions executed)
    Ir,
    ///D Cache reads (which equals the number of memory reads) (--cache-sim=yes)
    Dr,
    ///D Cache writes (which equals the number of memory writes) (--cache-sim=yes)
    Dw,
    ///I1 cache read misses (--cache-sim=yes)
    I1mr,
    ///D1 cache read misses (--cache-sim=yes)
    D1mr,
    ///D1 cache write misses (--cache-sim=yes)
    D1mw,
    ///LL cache instruction read misses (--cache-sim=yes)
    ILmr,
    ///LL cache data read misses (--cache-sim=yes)
    DLmr,
    ///LL cache data write misses (--cache-sim=yes)
    DLmw,
    ///I1 cache miss rate (--cache-sim=yes)
    I1MissRate,
    ///LL/L2 instructions cache miss rate (--cache-sim=yes)
    LLiMissRate,
    ///D1 cache miss rate (--cache-sim=yes)
    D1MissRate,
    ///LL/L2 data cache miss rate (--cache-sim=yes)
    LLdMissRate,
    ///LL/L2 cache miss rate (--cache-sim=yes)
    #[serde(rename = "LLMissRate")]
    LlMissRate,
    ///Derived event showing the L1 hits (--cache-sim=yes)
    L1hits,
    ///Derived event showing the LL hits (--cache-sim=yes)
    LLhits,
    ///Derived event showing the RAM hits (--cache-sim=yes)
    RamHits,
    ///L1 cache hit rate (--cache-sim=yes)
    L1HitRate,
    ///LL/L2 cache hit rate (--cache-sim=yes)
    #[serde(rename = "LLHitRate")]
    LlHitRate,
    ///RAM hit rate (--cache-sim=yes)
    RamHitRate,
    ///Derived event showing the total amount of cache reads and writes (--cache-sim=yes)
    #[serde(rename = "TotalRW")]
    TotalRw,
    ///Derived event showing estimated CPU cycles (--cache-sim=yes)
    EstimatedCycles,
    ///The number of system calls done (--collect-systime=yes)
    SysCount,
    ///The elapsed time spent in system calls (--collect-systime=yes)
    SysTime,
    ///The cpu time spent during system calls (--collect-systime=nsec)
    SysCpuTime,
    ///The number of global bus events (--collect-bus=yes)
    Ge,
    ///Conditional branches executed (--branch-sim=yes)
    Bc,
    ///Conditional branches mispredicted (--branch-sim=yes)
    Bcm,
    ///Indirect branches executed (--branch-sim=yes)
    Bi,
    ///Indirect branches mispredicted (--branch-sim=yes)
    Bim,
    ///Dirty miss because of instruction read (--simulate-wb=yes)
    ILdmr,
    ///Dirty miss because of data read (--simulate-wb=yes)
    DLdmr,
    ///Dirty miss because of data write (--simulate-wb=yes)
    DLdmw,
    ///Counter showing bad temporal locality for L1 caches (--cachuse=yes)
    AcCost1,
    ///Counter showing bad temporal locality for LL caches (--cachuse=yes)
    AcCost2,
    ///Counter showing bad spatial locality for L1 caches (--cachuse=yes)
    SpLoss1,
    ///Counter showing bad spatial locality for LL caches (--cachuse=yes)
    SpLoss2,
}
impl ::std::convert::From<&Self> for EventKind {
    fn from(value: &EventKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ir => f.write_str("Ir"),
            Self::Dr => f.write_str("Dr"),
            Self::Dw => f.write_str("Dw"),
            Self::I1mr => f.write_str("I1mr"),
            Self::D1mr => f.write_str("D1mr"),
            Self::D1mw => f.write_str("D1mw"),
            Self::ILmr => f.write_str("ILmr"),
            Self::DLmr => f.write_str("DLmr"),
            Self::DLmw => f.write_str("DLmw"),
            Self::I1MissRate => f.write_str("I1MissRate"),
            Self::LLiMissRate => f.write_str("LLiMissRate"),
            Self::D1MissRate => f.write_str("D1MissRate"),
            Self::LLdMissRate => f.write_str("LLdMissRate"),
            Self::LlMissRate => f.write_str("LLMissRate"),
            Self::L1hits => f.write_str("L1hits"),
            Self::LLhits => f.write_str("LLhits"),
            Self::RamHits => f.write_str("RamHits"),
            Self::L1HitRate => f.write_str("L1HitRate"),
            Self::LlHitRate => f.write_str("LLHitRate"),
            Self::RamHitRate => f.write_str("RamHitRate"),
            Self::TotalRw => f.write_str("TotalRW"),
            Self::EstimatedCycles => f.write_str("EstimatedCycles"),
            Self::SysCount => f.write_str("SysCount"),
            Self::SysTime => f.write_str("SysTime"),
            Self::SysCpuTime => f.write_str("SysCpuTime"),
            Self::Ge => f.write_str("Ge"),
            Self::Bc => f.write_str("Bc"),
            Self::Bcm => f.write_str("Bcm"),
            Self::Bi => f.write_str("Bi"),
            Self::Bim => f.write_str("Bim"),
            Self::ILdmr => f.write_str("ILdmr"),
            Self::DLdmr => f.write_str("DLdmr"),
            Self::DLdmw => f.write_str("DLdmw"),
            Self::AcCost1 => f.write_str("AcCost1"),
            Self::AcCost2 => f.write_str("AcCost2"),
            Self::SpLoss1 => f.write_str("SpLoss1"),
            Self::SpLoss2 => f.write_str("SpLoss2"),
        }
    }
}
impl ::std::str::FromStr for EventKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Ir" => Ok(Self::Ir),
            "Dr" => Ok(Self::Dr),
            "Dw" => Ok(Self::Dw),
            "I1mr" => Ok(Self::I1mr),
            "D1mr" => Ok(Self::D1mr),
            "D1mw" => Ok(Self::D1mw),
            "ILmr" => Ok(Self::ILmr),
            "DLmr" => Ok(Self::DLmr),
            "DLmw" => Ok(Self::DLmw),
            "I1MissRate" => Ok(Self::I1MissRate),
            "LLiMissRate" => Ok(Self::LLiMissRate),
            "D1MissRate" => Ok(Self::D1MissRate),
            "LLdMissRate" => Ok(Self::LLdMissRate),
            "LLMissRate" => Ok(Self::LlMissRate),
            "L1hits" => Ok(Self::L1hits),
            "LLhits" => Ok(Self::LLhits),
            "RamHits" => Ok(Self::RamHits),
            "L1HitRate" => Ok(Self::L1HitRate),
            "LLHitRate" => Ok(Self::LlHitRate),
            "RamHitRate" => Ok(Self::RamHitRate),
            "TotalRW" => Ok(Self::TotalRw),
            "EstimatedCycles" => Ok(Self::EstimatedCycles),
            "SysCount" => Ok(Self::SysCount),
            "SysTime" => Ok(Self::SysTime),
            "SysCpuTime" => Ok(Self::SysCpuTime),
            "Ge" => Ok(Self::Ge),
            "Bc" => Ok(Self::Bc),
            "Bcm" => Ok(Self::Bcm),
            "Bi" => Ok(Self::Bi),
            "Bim" => Ok(Self::Bim),
            "ILdmr" => Ok(Self::ILdmr),
            "DLdmr" => Ok(Self::DLdmr),
            "DLdmw" => Ok(Self::DLdmw),
            "AcCost1" => Ok(Self::AcCost1),
            "AcCost2" => Ok(Self::AcCost2),
            "SpLoss1" => Ok(Self::SpLoss1),
            "SpLoss2" => Ok(Self::SpLoss2),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The callgrind `FlamegraphSummary` records all created paths for an [`EventKind`] specific
///flamegraph
///
///Either the `regular_path`, `old_path` or the `diff_path` are present. Never can all of them
/// be absent.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The callgrind `FlamegraphSummary` records all created paths for an
/// [`EventKind`] specific\nflamegraph\n\nEither the `regular_path`, `old_path` or the
/// `diff_path` are present. Never can all of them be\nabsent.",
///  "type": "object",
///  "required": [
///    "event_kind"
///  ],
///  "properties": {
///    "base_path": {
///      "description": "If present, the path to the file of the old regular (non-differential)
/// flamegraph",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "diff_path": {
///      "description": "If present, the path to the file of the differential flamegraph",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "event_kind": {
///      "description": "The `EventKind` of the flamegraph",
///      "allOf": [
///        {
///          "$ref": "#/definitions/EventKind"
///        }
///      ]
///    },
///    "regular_path": {
///      "description": "If present, the path to the file of the regular (non-differential)
/// flamegraph",
///      "type": [
///        "string",
///        "null"
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FlamegraphSummary {
    ///If present, the path to the file of the old regular (non-differential) flamegraph
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base_path: ::std::option::Option<::std::string::String>,
    ///If present, the path to the file of the differential flamegraph
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diff_path: ::std::option::Option<::std::string::String>,
    ///The `EventKind` of the flamegraph
    pub event_kind: EventKind,
    ///If present, the path to the file of the regular (non-differential) flamegraph
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub regular_path: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FlamegraphSummary> for FlamegraphSummary {
    fn from(value: &FlamegraphSummary) -> Self {
        value.clone()
    }
}
impl FlamegraphSummary {
    pub fn builder() -> builder::FlamegraphSummary {
        Default::default()
    }
}
///The metric measured by valgrind or derived from one or more other metrics
///
///The valgrind metrics measured by any of its tools are `u64`. However, to be able to
/// represent derived metrics like cache miss/hit rates it is inevitable to have a type which
/// can store a `u64` or a `f64`. When doing math with metrics, the original type should be
/// preserved as far as possible by using `u64` operations. A float metric should be a last
/// resort.
///
///Float operations with a `Metric` that stores a `u64` introduce a precision loss and are to
/// be avoided. Especially comparison between a `u64` metric and `f64` metric are not exact
/// because the `u64` has to be converted to a `f64`. Also, if adding/multiplying two `u64`
/// metrics would result in an overflow the metric saturates at `u64::MAX`. This choice was
/// made to preserve precision and the original type (instead of for example adding the two
/// `u64` by converting both of them to `f64`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The metric measured by valgrind or derived from one or more other
/// metrics\n\nThe valgrind metrics measured by any of its tools are `u64`. However, to be able
/// to represent\nderived metrics like cache miss/hit rates it is inevitable to have a type
/// which can store a\n`u64` or a `f64`. When doing math with metrics, the original type should
/// be preserved as far as\npossible by using `u64` operations. A float metric should be a last
/// resort.\n\nFloat operations with a `Metric` that stores a `u64` introduce a precision loss
/// and are to be\navoided. Especially comparison between a `u64` metric and `f64` metric are
/// not exact because the\n`u64` has to be converted to a `f64`. Also, if adding/multiplying
/// two `u64` metrics would result\nin an overflow the metric saturates at `u64::MAX`. This
/// choice was made to preserve precision\nand the original type (instead of for example adding
/// the two `u64` by converting both of them to\n`f64`).",
///  "oneOf": [
///    {
///      "description": "An integer `Metric`",
///      "type": "object",
///      "required": [
///        "Int"
///      ],
///      "properties": {
///        "Int": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "A float `Metric`",
///      "type": "object",
///      "required": [
///        "Float"
///      ],
///      "properties": {
///        "Float": {
///          "type": "number",
///          "format": "double"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum Metric {
    ///An integer `Metric`
    Int(u64),
    ///A float `Metric`
    Float(f64),
}
impl ::std::convert::From<&Self> for Metric {
    fn from(value: &Metric) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u64> for Metric {
    fn from(value: u64) -> Self {
        Self::Int(value)
    }
}
impl ::std::convert::From<f64> for Metric {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
///The different metrics distinguished by tool and if it is an error checking tool as
/// `ErrorMetric`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The different metrics distinguished by tool and if it is an error checking
/// tool as `ErrorMetric`",
///  "oneOf": [
///    {
///      "description": "The `None` kind if there are no metrics for a tool",
///      "type": "string",
///      "const": "None"
///    },
///    {
///      "description": "The Callgrind metric kind",
///      "type": "object",
///      "required": [
///        "Callgrind"
///      ],
///      "properties": {
///        "Callgrind": {
///          "$ref": "#/definitions/EventKind"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The Cachegrind metric kind",
///      "type": "object",
///      "required": [
///        "Cachegrind"
///      ],
///      "properties": {
///        "Cachegrind": {
///          "$ref": "#/definitions/CachegrindMetric"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The DHAT metric kind",
///      "type": "object",
///      "required": [
///        "Dhat"
///      ],
///      "properties": {
///        "Dhat": {
///          "$ref": "#/definitions/DhatMetric"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The Memcheck metric kind",
///      "type": "object",
///      "required": [
///        "Memcheck"
///      ],
///      "properties": {
///        "Memcheck": {
///          "$ref": "#/definitions/ErrorMetric"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The Helgrind metric kind",
///      "type": "object",
///      "required": [
///        "Helgrind"
///      ],
///      "properties": {
///        "Helgrind": {
///          "$ref": "#/definitions/ErrorMetric"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The DRD metric kind",
///      "type": "object",
///      "required": [
///        "DRD"
///      ],
///      "properties": {
///        "DRD": {
///          "$ref": "#/definitions/ErrorMetric"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum MetricKind {
    ///The `None` kind if there are no metrics for a tool
    None,
    ///The Callgrind metric kind
    Callgrind(EventKind),
    ///The Cachegrind metric kind
    Cachegrind(CachegrindMetric),
    ///The DHAT metric kind
    Dhat(DhatMetric),
    ///The Memcheck metric kind
    Memcheck(ErrorMetric),
    ///The Helgrind metric kind
    Helgrind(ErrorMetric),
    ///The DRD metric kind
    #[serde(rename = "DRD")]
    Drd(ErrorMetric),
}
impl ::std::convert::From<&Self> for MetricKind {
    fn from(value: &MetricKind) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<EventKind> for MetricKind {
    fn from(value: EventKind) -> Self {
        Self::Callgrind(value)
    }
}
impl ::std::convert::From<CachegrindMetric> for MetricKind {
    fn from(value: CachegrindMetric) -> Self {
        Self::Cachegrind(value)
    }
}
impl ::std::convert::From<DhatMetric> for MetricKind {
    fn from(value: DhatMetric) -> Self {
        Self::Dhat(value)
    }
}
///The `MetricsDiff` describes the difference between a `new` and `old` metric as percentage
/// and factor.
///
///Only if both metrics are present there is also a `Diffs` present. Otherwise, it just stores
/// the `new` or `old` metric.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `MetricsDiff` describes the difference between a `new` and `old`
/// metric as percentage and\nfactor.\n\nOnly if both metrics are present there is also a
/// `Diffs` present. Otherwise, it just stores the\n`new` or `old` metric.",
///  "type": "object",
///  "required": [
///    "metrics"
///  ],
///  "properties": {
///    "diffs": {
///      "description": "If both metrics are present there is also a `Diffs` present",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Diffs"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "metrics": {
///      "description": "Either the `new`, `old` or both metrics",
///      "allOf": [
///        {
///          "$ref": "#/definitions/EitherOrBoth2"
///        }
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MetricsDiff {
    ///If both metrics are present there is also a `Diffs` present
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub diffs: ::std::option::Option<Diffs>,
    ///Either the `new`, `old` or both metrics
    pub metrics: EitherOrBoth2,
}
impl ::std::convert::From<&MetricsDiff> for MetricsDiff {
    fn from(value: &MetricsDiff) -> Self {
        value.clone()
    }
}
impl MetricsDiff {
    pub fn builder() -> builder::MetricsDiff {
        Default::default()
    }
}
///The `MetricsSummary` contains all differences between two tool run segments
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `MetricsSummary` contains all differences between two tool run
/// segments",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/definitions/MetricsDiff"
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MetricsSummary(pub ::std::collections::HashMap<::std::string::String, MetricsDiff>);
impl ::std::ops::Deref for MetricsSummary {
    type Target = ::std::collections::HashMap<::std::string::String, MetricsDiff>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, MetricsDiff> {
        &self.0
    }
}
impl ::std::convert::From<MetricsSummary>
    for ::std::collections::HashMap<::std::string::String, MetricsDiff>
{
    fn from(value: MetricsSummary) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MetricsSummary> for MetricsSummary {
    fn from(value: &MetricsSummary) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, MetricsDiff>>
    for MetricsSummary
{
    fn from(value: ::std::collections::HashMap<::std::string::String, MetricsDiff>) -> Self {
        Self(value)
    }
}
///The `MetricsSummary` contains all differences between two tool run segments
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `MetricsSummary` contains all differences between two tool run
/// segments",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/definitions/MetricsDiff"
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MetricsSummary2(pub ::std::collections::HashMap<::std::string::String, MetricsDiff>);
impl ::std::ops::Deref for MetricsSummary2 {
    type Target = ::std::collections::HashMap<::std::string::String, MetricsDiff>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, MetricsDiff> {
        &self.0
    }
}
impl ::std::convert::From<MetricsSummary2>
    for ::std::collections::HashMap<::std::string::String, MetricsDiff>
{
    fn from(value: MetricsSummary2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MetricsSummary2> for MetricsSummary2 {
    fn from(value: &MetricsSummary2) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, MetricsDiff>>
    for MetricsSummary2
{
    fn from(value: ::std::collections::HashMap<::std::string::String, MetricsDiff>) -> Self {
        Self(value)
    }
}
///The `MetricsSummary` contains all differences between two tool run segments
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `MetricsSummary` contains all differences between two tool run
/// segments",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/definitions/MetricsDiff"
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MetricsSummary3(pub ::std::collections::HashMap<::std::string::String, MetricsDiff>);
impl ::std::ops::Deref for MetricsSummary3 {
    type Target = ::std::collections::HashMap<::std::string::String, MetricsDiff>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, MetricsDiff> {
        &self.0
    }
}
impl ::std::convert::From<MetricsSummary3>
    for ::std::collections::HashMap<::std::string::String, MetricsDiff>
{
    fn from(value: MetricsSummary3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MetricsSummary3> for MetricsSummary3 {
    fn from(value: &MetricsSummary3) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, MetricsDiff>>
    for MetricsSummary3
{
    fn from(value: ::std::collections::HashMap<::std::string::String, MetricsDiff>) -> Self {
        Self(value)
    }
}
///The `MetricsSummary` contains all differences between two tool run segments
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `MetricsSummary` contains all differences between two tool run
/// segments",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/definitions/MetricsDiff"
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MetricsSummary4(pub ::std::collections::HashMap<::std::string::String, MetricsDiff>);
impl ::std::ops::Deref for MetricsSummary4 {
    type Target = ::std::collections::HashMap<::std::string::String, MetricsDiff>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, MetricsDiff> {
        &self.0
    }
}
impl ::std::convert::From<MetricsSummary4>
    for ::std::collections::HashMap<::std::string::String, MetricsDiff>
{
    fn from(value: MetricsSummary4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MetricsSummary4> for MetricsSummary4 {
    fn from(value: &MetricsSummary4) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, MetricsDiff>>
    for MetricsSummary4
{
    fn from(value: ::std::collections::HashMap<::std::string::String, MetricsDiff>) -> Self {
        Self(value)
    }
}
///The `ToolSummary` containing all information about a valgrind tool run
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `ToolSummary` containing all information about a valgrind tool run",
///  "type": "object",
///  "required": [
///    "flamegraphs",
///    "log_paths",
///    "out_paths",
///    "summaries",
///    "tool"
///  ],
///  "properties": {
///    "flamegraphs": {
///      "description": "Details and information about the created flamegraphs if any",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/FlamegraphSummary"
///      }
///    },
///    "log_paths": {
///      "description": "The paths to the `*.log` files. All tools produce at least one log
/// file",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "out_paths": {
///      "description": "The paths to the `*.out` files. Not all tools produce an output in
/// addition to the log\nfiles",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "summaries": {
///      "description": "The metrics and details about the tool run",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ProfileData"
///        }
///      ]
///    },
///    "tool": {
///      "description": "The Valgrind tool like `DHAT`, `Memcheck` etc.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ValgrindTool"
///        }
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Profile {
    ///Details and information about the created flamegraphs if any
    pub flamegraphs: ::std::vec::Vec<FlamegraphSummary>,
    ///The paths to the `*.log` files. All tools produce at least one log file
    pub log_paths: ::std::vec::Vec<::std::string::String>,
    ///The paths to the `*.out` files. Not all tools produce an output in addition to the log
    ///files
    pub out_paths: ::std::vec::Vec<::std::string::String>,
    ///The metrics and details about the tool run
    pub summaries: ProfileData,
    ///The Valgrind tool like `DHAT`, `Memcheck` etc.
    pub tool: ValgrindTool,
}
impl ::std::convert::From<&Profile> for Profile {
    fn from(value: &Profile) -> Self {
        value.clone()
    }
}
impl Profile {
    pub fn builder() -> builder::Profile {
        Default::default()
    }
}
///The `ToolRun` contains all information about a single tool run with possibly multiple
/// segments
///
///The total is always present and summarizes all tool run segments. In the special case of a
///single tool run segment, the total equals the metrics of this segment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `ToolRun` contains all information about a single tool run with
/// possibly multiple segments\n\nThe total is always present and summarizes all tool run
/// segments. In the special case of a\nsingle tool run segment, the total equals the metrics
/// of this segment.",
///  "type": "object",
///  "required": [
///    "parts",
///    "total"
///  ],
///  "properties": {
///    "parts": {
///      "description": "All [`ProfilePart`]s",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProfilePart"
///      }
///    },
///    "total": {
///      "description": "The total over the [`ProfilePart`]s",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ProfileTotal"
///        }
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProfileData {
    ///All [`ProfilePart`]s
    pub parts: ::std::vec::Vec<ProfilePart>,
    ///The total over the [`ProfilePart`]s
    pub total: ProfileTotal,
}
impl ::std::convert::From<&ProfileData> for ProfileData {
    fn from(value: &ProfileData) -> Self {
        value.clone()
    }
}
impl ProfileData {
    pub fn builder() -> builder::ProfileData {
        Default::default()
    }
}
///Some additional and necessary information about the tool run segment
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Some additional and necessary information about the tool run segment",
///  "type": "object",
///  "required": [
///    "command",
///    "path",
///    "pid"
///  ],
///  "properties": {
///    "command": {
///      "description": "The executed command extracted from Valgrind output",
///      "type": "string"
///    },
///    "details": {
///      "description": "More details for example from the logging output of the tool run",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "parent_pid": {
///      "description": "The parent pid of this process",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "part": {
///      "description": "The part of this tool run (only callgrind)",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "path": {
///      "description": "The path to the file from the tool run",
///      "type": "string"
///    },
///    "pid": {
///      "description": "The pid of this process",
///      "type": "integer",
///      "format": "int32"
///    },
///    "thread": {
///      "description": "The thread of this tool run (only callgrind)",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProfileInfo {
    ///The executed command extracted from Valgrind output
    pub command: ::std::string::String,
    ///More details for example from the logging output of the tool run
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<::std::string::String>,
    ///The parent pid of this process
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_pid: ::std::option::Option<i32>,
    ///The part of this tool run (only callgrind)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub part: ::std::option::Option<u64>,
    ///The path to the file from the tool run
    pub path: ::std::string::String,
    ///The pid of this process
    pub pid: i32,
    ///The thread of this tool run (only callgrind)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thread: ::std::option::Option<u32>,
}
impl ::std::convert::From<&ProfileInfo> for ProfileInfo {
    fn from(value: &ProfileInfo) -> Self {
        value.clone()
    }
}
impl ProfileInfo {
    pub fn builder() -> builder::ProfileInfo {
        Default::default()
    }
}
///A single segment of a tool run and if present the comparison with the "old" segment
///
///A tool run can produce multiple segments, for example for each process and subprocess with
///(--trace-children).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "A single segment of a tool run and if present the comparison with the
/// \"old\" segment\n\nA tool run can produce multiple segments, for example for each process
/// and subprocess with\n(--trace-children).",
///  "type": "object",
///  "required": [
///    "details",
///    "metrics_summary"
///  ],
///  "properties": {
///    "details": {
///      "description": "Details like command, pid, ppid, thread number etc. (see
/// [`ProfileInfo`])",
///      "allOf": [
///        {
///          "$ref": "#/definitions/EitherOrBoth"
///        }
///      ]
///    },
///    "metrics_summary": {
///      "description": "The [`ToolMetricSummary`]",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ToolMetricSummary"
///        }
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProfilePart {
    ///Details like command, pid, ppid, thread number etc. (see [`ProfileInfo`])
    pub details: EitherOrBoth,
    ///The [`ToolMetricSummary`]
    pub metrics_summary: ToolMetricSummary,
}
impl ::std::convert::From<&ProfilePart> for ProfilePart {
    fn from(value: &ProfilePart) -> Self {
        value.clone()
    }
}
impl ProfilePart {
    pub fn builder() -> builder::ProfilePart {
        Default::default()
    }
}
///The total metrics over all [`ProfilePart`]s and if detected any [`ToolRegression`]
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The total metrics over all [`ProfilePart`]s and if detected any
/// [`ToolRegression`]",
///  "type": "object",
///  "required": [
///    "regressions",
///    "summary"
///  ],
///  "properties": {
///    "regressions": {
///      "description": "The detected regressions if any",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ToolRegression"
///      }
///    },
///    "summary": {
///      "description": "The summary of metrics of the tool",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ToolMetricSummary"
///        }
///      ]
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProfileTotal {
    ///The detected regressions if any
    pub regressions: ::std::vec::Vec<ToolRegression>,
    ///The summary of metrics of the tool
    pub summary: ToolMetricSummary,
}
impl ::std::convert::From<&ProfileTotal> for ProfileTotal {
    fn from(value: &ProfileTotal) -> Self {
        value.clone()
    }
}
impl ProfileTotal {
    pub fn builder() -> builder::ProfileTotal {
        Default::default()
    }
}
///The collection of all generated [`Profile`]s
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The collection of all generated [`Profile`]s",
///  "type": "array",
///  "items": {
///    "$ref": "#/definitions/Profile"
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Profiles(pub ::std::vec::Vec<Profile>);
impl ::std::ops::Deref for Profiles {
    type Target = ::std::vec::Vec<Profile>;
    fn deref(&self) -> &::std::vec::Vec<Profile> {
        &self.0
    }
}
impl ::std::convert::From<Profiles> for ::std::vec::Vec<Profile> {
    fn from(value: Profiles) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Profiles> for Profiles {
    fn from(value: &Profiles) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Profile>> for Profiles {
    fn from(value: ::std::vec::Vec<Profile>) -> Self {
        Self(value)
    }
}
///The format (json, ...) in which the summary file should be saved or printed
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The format (json, ...) in which the summary file should be saved or
/// printed",
///  "oneOf": [
///    {
///      "description": "The format in a space optimal json representation without newlines",
///      "type": "string",
///      "const": "Json"
///    },
///    {
///      "description": "The format in pretty printed json",
///      "type": "string",
///      "const": "PrettyJson"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SummaryFormat {
    ///The format in a space optimal json representation without newlines
    Json,
    ///The format in pretty printed json
    PrettyJson,
}
impl ::std::convert::From<&Self> for SummaryFormat {
    fn from(value: &SummaryFormat) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SummaryFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Json => f.write_str("Json"),
            Self::PrettyJson => f.write_str("PrettyJson"),
        }
    }
}
impl ::std::str::FromStr for SummaryFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Json" => Ok(Self::Json),
            "PrettyJson" => Ok(Self::PrettyJson),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SummaryFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SummaryFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SummaryFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Manage the summary output file with this `SummaryOutput`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Manage the summary output file with this `SummaryOutput`",
///  "type": "object",
///  "required": [
///    "format",
///    "path"
///  ],
///  "properties": {
///    "format": {
///      "description": "The [`SummaryFormat`]",
///      "allOf": [
///        {
///          "$ref": "#/definitions/SummaryFormat"
///        }
///      ]
///    },
///    "path": {
///      "description": "The path to the destination file of this summary",
///      "type": "string"
///    }
///  }
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SummaryOutput {
    ///The [`SummaryFormat`]
    pub format: SummaryFormat,
    ///The path to the destination file of this summary
    pub path: ::std::string::String,
}
impl ::std::convert::From<&SummaryOutput> for SummaryOutput {
    fn from(value: &SummaryOutput) -> Self {
        value.clone()
    }
}
impl SummaryOutput {
    pub fn builder() -> builder::SummaryOutput {
        Default::default()
    }
}
///The `ToolMetricSummary` contains the `MetricsSummary` distinguished by tool and metric
/// kinds
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The `ToolMetricSummary` contains the `MetricsSummary` distinguished by
/// tool and metric kinds",
///  "oneOf": [
///    {
///      "description": "If there are no metrics extracted (currently massif, bbv)",
///      "type": "string",
///      "const": "None"
///    },
///    {
///      "description": "The error summary of tools which reports errors (memcheck, helgrind,
/// drd)",
///      "type": "object",
///      "required": [
///        "ErrorTool"
///      ],
///      "properties": {
///        "ErrorTool": {
///          "$ref": "#/definitions/MetricsSummary"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The dhat summary",
///      "type": "object",
///      "required": [
///        "Dhat"
///      ],
///      "properties": {
///        "Dhat": {
///          "$ref": "#/definitions/MetricsSummary2"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The callgrind summary",
///      "type": "object",
///      "required": [
///        "Callgrind"
///      ],
///      "properties": {
///        "Callgrind": {
///          "$ref": "#/definitions/MetricsSummary3"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The cachegrind summary",
///      "type": "object",
///      "required": [
///        "Cachegrind"
///      ],
///      "properties": {
///        "Cachegrind": {
///          "$ref": "#/definitions/MetricsSummary4"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum ToolMetricSummary {
    ///If there are no metrics extracted (currently massif, bbv)
    None,
    ///The error summary of tools which reports errors (memcheck, helgrind, drd)
    ErrorTool(MetricsSummary),
    ///The dhat summary
    Dhat(MetricsSummary2),
    ///The callgrind summary
    Callgrind(MetricsSummary3),
    ///The cachegrind summary
    Cachegrind(MetricsSummary4),
}
impl ::std::convert::From<&Self> for ToolMetricSummary {
    fn from(value: &ToolMetricSummary) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<MetricsSummary> for ToolMetricSummary {
    fn from(value: MetricsSummary) -> Self {
        Self::ErrorTool(value)
    }
}
impl ::std::convert::From<MetricsSummary2> for ToolMetricSummary {
    fn from(value: MetricsSummary2) -> Self {
        Self::Dhat(value)
    }
}
impl ::std::convert::From<MetricsSummary3> for ToolMetricSummary {
    fn from(value: MetricsSummary3) -> Self {
        Self::Callgrind(value)
    }
}
impl ::std::convert::From<MetricsSummary4> for ToolMetricSummary {
    fn from(value: MetricsSummary4) -> Self {
        Self::Cachegrind(value)
    }
}
///A detected performance regression depending on the limit either `Soft` or `Hard`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "A detected performance regression depending on the limit either `Soft` or
/// `Hard`",
///  "oneOf": [
///    {
///      "description": "A performance regression triggered by a soft limit",
///      "type": "object",
///      "required": [
///        "Soft"
///      ],
///      "properties": {
///        "Soft": {
///          "type": "object",
///          "required": [
///            "diff_pct",
///            "limit",
///            "metric",
///            "new",
///            "old"
///          ],
///          "properties": {
///            "diff_pct": {
///              "description": "The difference between new and old in percent. Serialized as
/// string to preserve\ninfinity values and avoid null in json.",
///              "type": "string"
///            },
///            "limit": {
///              "description": "The value of the limit which was exceeded to cause a
/// performance regression. Serialized\nas string to preserve infinity values and avoid null in
/// json.",
///              "type": "string"
///            },
///            "metric": {
///              "description": "The metric kind per tool",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/MetricKind"
///                }
///              ]
///            },
///            "new": {
///              "description": "The value of the new benchmark run",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/Metric"
///                }
///              ]
///            },
///            "old": {
///              "description": "The value of the old benchmark run",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/Metric"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "A performance regression triggered by a hard limit",
///      "type": "object",
///      "required": [
///        "Hard"
///      ],
///      "properties": {
///        "Hard": {
///          "type": "object",
///          "required": [
///            "diff",
///            "limit",
///            "metric",
///            "new"
///          ],
///          "properties": {
///            "diff": {
///              "description": "The difference between new and the limit",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/Metric"
///                }
///              ]
///            },
///            "limit": {
///              "description": "The limit",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/Metric"
///                }
///              ]
///            },
///            "metric": {
///              "description": "The metric kind per tool",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/MetricKind"
///                }
///              ]
///            },
///            "new": {
///              "description": "The value of the benchmark run",
///              "allOf": [
///                {
///                  "$ref": "#/definitions/Metric"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum ToolRegression {
    ///A performance regression triggered by a soft limit
    Soft {
        ///The difference between new and old in percent. Serialized as string to preserve
        ///infinity values and avoid null in json.
        diff_pct: ::std::string::String,
        ///The value of the limit which was exceeded to cause a performance regression.
        /// Serialized as string to preserve infinity values and avoid null in json.
        limit: ::std::string::String,
        ///The metric kind per tool
        metric: MetricKind,
        ///The value of the new benchmark run
        new: Metric,
        ///The value of the old benchmark run
        old: Metric,
    },
    ///A performance regression triggered by a hard limit
    Hard {
        ///The difference between new and the limit
        diff: Metric,
        ///The limit
        limit: Metric,
        ///The metric kind per tool
        metric: MetricKind,
        ///The value of the benchmark run
        new: Metric,
    },
}
impl ::std::convert::From<&Self> for ToolRegression {
    fn from(value: &ToolRegression) -> Self {
        value.clone()
    }
}
///The valgrind tools which can be run
///
///Note the default changes from `Callgrind` to `Cachegrind` if the `cachegrind` feature is
///selected.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "The valgrind tools which can be run\n\nNote the default changes from
/// `Callgrind` to `Cachegrind` if the `cachegrind` feature is\nselected.",
///  "oneOf": [
///    {
///      "description": "[Callgrind: a call-graph generating cache and branch prediction profiler](https://valgrind.org/docs/manual/cl-manual.html)",
///      "type": "string",
///      "const": "Callgrind"
///    },
///    {
///      "description": "[Cachegrind: a high-precision tracing profiler](https://valgrind.org/docs/manual/cg-manual.html)",
///      "type": "string",
///      "const": "Cachegrind"
///    },
///    {
///      "description": "[DHAT: a dynamic heap analysis tool](https://valgrind.org/docs/manual/dh-manual.html)",
///      "type": "string",
///      "const": "DHAT"
///    },
///    {
///      "description": "[Memcheck: a memory error detector](https://valgrind.org/docs/manual/mc-manual.html)",
///      "type": "string",
///      "const": "Memcheck"
///    },
///    {
///      "description": "[Helgrind: a thread error detector](https://valgrind.org/docs/manual/hg-manual.html)",
///      "type": "string",
///      "const": "Helgrind"
///    },
///    {
///      "description": "[DRD: a thread error detector](https://valgrind.org/docs/manual/drd-manual.html)",
///      "type": "string",
///      "const": "DRD"
///    },
///    {
///      "description": "[Massif: a heap profiler](https://valgrind.org/docs/manual/ms-manual.html)",
///      "type": "string",
///      "const": "Massif"
///    },
///    {
///      "description": "[BBV: an experimental basic block vector generation tool](https://valgrind.org/docs/manual/bbv-manual.html)",
///      "type": "string",
///      "const": "BBV"
///    }
///  ]
/// }
/// ```
/// </details>
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ValgrindTool {
    ///[Callgrind: a call-graph generating cache and branch prediction profiler](https://valgrind.org/docs/manual/cl-manual.html)
    Callgrind,
    ///[Cachegrind: a high-precision tracing profiler](https://valgrind.org/docs/manual/cg-manual.html)
    Cachegrind,
    ///[DHAT: a dynamic heap analysis tool](https://valgrind.org/docs/manual/dh-manual.html)
    #[serde(rename = "DHAT")]
    Dhat,
    ///[Memcheck: a memory error detector](https://valgrind.org/docs/manual/mc-manual.html)
    Memcheck,
    ///[Helgrind: a thread error detector](https://valgrind.org/docs/manual/hg-manual.html)
    Helgrind,
    ///[DRD: a thread error detector](https://valgrind.org/docs/manual/drd-manual.html)
    #[serde(rename = "DRD")]
    Drd,
    ///[Massif: a heap profiler](https://valgrind.org/docs/manual/ms-manual.html)
    Massif,
    ///[BBV: an experimental basic block vector generation tool](https://valgrind.org/docs/manual/bbv-manual.html)
    #[serde(rename = "BBV")]
    Bbv,
}
impl ::std::convert::From<&Self> for ValgrindTool {
    fn from(value: &ValgrindTool) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ValgrindTool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Callgrind => f.write_str("Callgrind"),
            Self::Cachegrind => f.write_str("Cachegrind"),
            Self::Dhat => f.write_str("DHAT"),
            Self::Memcheck => f.write_str("Memcheck"),
            Self::Helgrind => f.write_str("Helgrind"),
            Self::Drd => f.write_str("DRD"),
            Self::Massif => f.write_str("Massif"),
            Self::Bbv => f.write_str("BBV"),
        }
    }
}
impl ::std::str::FromStr for ValgrindTool {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Callgrind" => Ok(Self::Callgrind),
            "Cachegrind" => Ok(Self::Cachegrind),
            "DHAT" => Ok(Self::Dhat),
            "Memcheck" => Ok(Self::Memcheck),
            "Helgrind" => Ok(Self::Helgrind),
            "DRD" => Ok(Self::Drd),
            "Massif" => Ok(Self::Massif),
            "BBV" => Ok(Self::Bbv),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ValgrindTool {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ValgrindTool {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ValgrindTool {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BenchmarkSummary {
        baselines: ::std::result::Result<
            (
                ::std::option::Option<::std::string::String>,
                ::std::option::Option<::std::string::String>,
            ),
            ::std::string::String,
        >,
        benchmark_exe: ::std::result::Result<::std::string::String, ::std::string::String>,
        benchmark_file: ::std::result::Result<::std::string::String, ::std::string::String>,
        details: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        function_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::BenchmarkKind, ::std::string::String>,
        module_path: ::std::result::Result<::std::string::String, ::std::string::String>,
        package_dir: ::std::result::Result<::std::string::String, ::std::string::String>,
        profiles: ::std::result::Result<super::Profiles, ::std::string::String>,
        project_root: ::std::result::Result<::std::string::String, ::std::string::String>,
        summary_output: ::std::result::Result<
            ::std::option::Option<super::SummaryOutput>,
            ::std::string::String,
        >,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BenchmarkSummary {
        fn default() -> Self {
            Self {
                baselines: Err("no value supplied for baselines".to_string()),
                benchmark_exe: Err("no value supplied for benchmark_exe".to_string()),
                benchmark_file: Err("no value supplied for benchmark_file".to_string()),
                details: Ok(Default::default()),
                function_name: Err("no value supplied for function_name".to_string()),
                id: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                module_path: Err("no value supplied for module_path".to_string()),
                package_dir: Err("no value supplied for package_dir".to_string()),
                profiles: Err("no value supplied for profiles".to_string()),
                project_root: Err("no value supplied for project_root".to_string()),
                summary_output: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl BenchmarkSummary {
        pub fn baselines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<(
                ::std::option::Option<::std::string::String>,
                ::std::option::Option<::std::string::String>,
            )>,
            T::Error: ::std::fmt::Display,
        {
            self.baselines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for baselines: {}", e));
            self
        }
        pub fn benchmark_exe<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.benchmark_exe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for benchmark_exe: {}", e));
            self
        }
        pub fn benchmark_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.benchmark_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for benchmark_file: {}", e));
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
        pub fn function_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function_name: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BenchmarkKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn module_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.module_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for module_path: {}", e));
            self
        }
        pub fn package_dir<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package_dir = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package_dir: {}", e));
            self
        }
        pub fn profiles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Profiles>,
            T::Error: ::std::fmt::Display,
        {
            self.profiles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profiles: {}", e));
            self
        }
        pub fn project_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_root: {}", e));
            self
        }
        pub fn summary_output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummaryOutput>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary_output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary_output: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BenchmarkSummary> for super::BenchmarkSummary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BenchmarkSummary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                baselines: value.baselines?,
                benchmark_exe: value.benchmark_exe?,
                benchmark_file: value.benchmark_file?,
                details: value.details?,
                function_name: value.function_name?,
                id: value.id?,
                kind: value.kind?,
                module_path: value.module_path?,
                package_dir: value.package_dir?,
                profiles: value.profiles?,
                project_root: value.project_root?,
                summary_output: value.summary_output?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::BenchmarkSummary> for BenchmarkSummary {
        fn from(value: super::BenchmarkSummary) -> Self {
            Self {
                baselines: Ok(value.baselines),
                benchmark_exe: Ok(value.benchmark_exe),
                benchmark_file: Ok(value.benchmark_file),
                details: Ok(value.details),
                function_name: Ok(value.function_name),
                id: Ok(value.id),
                kind: Ok(value.kind),
                module_path: Ok(value.module_path),
                package_dir: Ok(value.package_dir),
                profiles: Ok(value.profiles),
                project_root: Ok(value.project_root),
                summary_output: Ok(value.summary_output),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Diffs {
        diff_pct: ::std::result::Result<::std::string::String, ::std::string::String>,
        factor: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Diffs {
        fn default() -> Self {
            Self {
                diff_pct: Err("no value supplied for diff_pct".to_string()),
                factor: Err("no value supplied for factor".to_string()),
            }
        }
    }
    impl Diffs {
        pub fn diff_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.diff_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diff_pct: {}", e));
            self
        }
        pub fn factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.factor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for factor: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Diffs> for super::Diffs {
        type Error = super::error::ConversionError;
        fn try_from(value: Diffs) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                diff_pct: value.diff_pct?,
                factor: value.factor?,
            })
        }
    }
    impl ::std::convert::From<super::Diffs> for Diffs {
        fn from(value: super::Diffs) -> Self {
            Self {
                diff_pct: Ok(value.diff_pct),
                factor: Ok(value.factor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FlamegraphSummary {
        base_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        diff_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_kind: ::std::result::Result<super::EventKind, ::std::string::String>,
        regular_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FlamegraphSummary {
        fn default() -> Self {
            Self {
                base_path: Ok(Default::default()),
                diff_path: Ok(Default::default()),
                event_kind: Err("no value supplied for event_kind".to_string()),
                regular_path: Ok(Default::default()),
            }
        }
    }
    impl FlamegraphSummary {
        pub fn base_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.base_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_path: {}", e));
            self
        }
        pub fn diff_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.diff_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diff_path: {}", e));
            self
        }
        pub fn event_kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventKind>,
            T::Error: ::std::fmt::Display,
        {
            self.event_kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_kind: {}", e));
            self
        }
        pub fn regular_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.regular_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for regular_path: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FlamegraphSummary> for super::FlamegraphSummary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FlamegraphSummary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_path: value.base_path?,
                diff_path: value.diff_path?,
                event_kind: value.event_kind?,
                regular_path: value.regular_path?,
            })
        }
    }
    impl ::std::convert::From<super::FlamegraphSummary> for FlamegraphSummary {
        fn from(value: super::FlamegraphSummary) -> Self {
            Self {
                base_path: Ok(value.base_path),
                diff_path: Ok(value.diff_path),
                event_kind: Ok(value.event_kind),
                regular_path: Ok(value.regular_path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MetricsDiff {
        diffs: ::std::result::Result<::std::option::Option<super::Diffs>, ::std::string::String>,
        metrics: ::std::result::Result<super::EitherOrBoth2, ::std::string::String>,
    }
    impl ::std::default::Default for MetricsDiff {
        fn default() -> Self {
            Self {
                diffs: Ok(Default::default()),
                metrics: Err("no value supplied for metrics".to_string()),
            }
        }
    }
    impl MetricsDiff {
        pub fn diffs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Diffs>>,
            T::Error: ::std::fmt::Display,
        {
            self.diffs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diffs: {}", e));
            self
        }
        pub fn metrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EitherOrBoth2>,
            T::Error: ::std::fmt::Display,
        {
            self.metrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metrics: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MetricsDiff> for super::MetricsDiff {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MetricsDiff,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                diffs: value.diffs?,
                metrics: value.metrics?,
            })
        }
    }
    impl ::std::convert::From<super::MetricsDiff> for MetricsDiff {
        fn from(value: super::MetricsDiff) -> Self {
            Self {
                diffs: Ok(value.diffs),
                metrics: Ok(value.metrics),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Profile {
        flamegraphs:
            ::std::result::Result<::std::vec::Vec<super::FlamegraphSummary>, ::std::string::String>,
        log_paths:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        out_paths:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        summaries: ::std::result::Result<super::ProfileData, ::std::string::String>,
        tool: ::std::result::Result<super::ValgrindTool, ::std::string::String>,
    }
    impl ::std::default::Default for Profile {
        fn default() -> Self {
            Self {
                flamegraphs: Err("no value supplied for flamegraphs".to_string()),
                log_paths: Err("no value supplied for log_paths".to_string()),
                out_paths: Err("no value supplied for out_paths".to_string()),
                summaries: Err("no value supplied for summaries".to_string()),
                tool: Err("no value supplied for tool".to_string()),
            }
        }
    }
    impl Profile {
        pub fn flamegraphs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FlamegraphSummary>>,
            T::Error: ::std::fmt::Display,
        {
            self.flamegraphs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flamegraphs: {}", e));
            self
        }
        pub fn log_paths<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.log_paths = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_paths: {}", e));
            self
        }
        pub fn out_paths<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.out_paths = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for out_paths: {}", e));
            self
        }
        pub fn summaries<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProfileData>,
            T::Error: ::std::fmt::Display,
        {
            self.summaries = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summaries: {}", e));
            self
        }
        pub fn tool<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ValgrindTool>,
            T::Error: ::std::fmt::Display,
        {
            self.tool = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Profile> for super::Profile {
        type Error = super::error::ConversionError;
        fn try_from(value: Profile) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flamegraphs: value.flamegraphs?,
                log_paths: value.log_paths?,
                out_paths: value.out_paths?,
                summaries: value.summaries?,
                tool: value.tool?,
            })
        }
    }
    impl ::std::convert::From<super::Profile> for Profile {
        fn from(value: super::Profile) -> Self {
            Self {
                flamegraphs: Ok(value.flamegraphs),
                log_paths: Ok(value.log_paths),
                out_paths: Ok(value.out_paths),
                summaries: Ok(value.summaries),
                tool: Ok(value.tool),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProfileData {
        parts: ::std::result::Result<::std::vec::Vec<super::ProfilePart>, ::std::string::String>,
        total: ::std::result::Result<super::ProfileTotal, ::std::string::String>,
    }
    impl ::std::default::Default for ProfileData {
        fn default() -> Self {
            Self {
                parts: Err("no value supplied for parts".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl ProfileData {
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProfilePart>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {}", e));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProfileTotal>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProfileData> for super::ProfileData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProfileData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                parts: value.parts?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::ProfileData> for ProfileData {
        fn from(value: super::ProfileData) -> Self {
            Self {
                parts: Ok(value.parts),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProfileInfo {
        command: ::std::result::Result<::std::string::String, ::std::string::String>,
        details: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parent_pid: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        part: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        pid: ::std::result::Result<i32, ::std::string::String>,
        thread: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    }
    impl ::std::default::Default for ProfileInfo {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                details: Ok(Default::default()),
                parent_pid: Ok(Default::default()),
                part: Ok(Default::default()),
                path: Err("no value supplied for path".to_string()),
                pid: Err("no value supplied for pid".to_string()),
                thread: Ok(Default::default()),
            }
        }
    }
    impl ProfileInfo {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {}", e));
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
        pub fn parent_pid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_pid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_pid: {}", e));
            self
        }
        pub fn part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn pid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.pid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pid: {}", e));
            self
        }
        pub fn thread<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.thread = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProfileInfo> for super::ProfileInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProfileInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                details: value.details?,
                parent_pid: value.parent_pid?,
                part: value.part?,
                path: value.path?,
                pid: value.pid?,
                thread: value.thread?,
            })
        }
    }
    impl ::std::convert::From<super::ProfileInfo> for ProfileInfo {
        fn from(value: super::ProfileInfo) -> Self {
            Self {
                command: Ok(value.command),
                details: Ok(value.details),
                parent_pid: Ok(value.parent_pid),
                part: Ok(value.part),
                path: Ok(value.path),
                pid: Ok(value.pid),
                thread: Ok(value.thread),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProfilePart {
        details: ::std::result::Result<super::EitherOrBoth, ::std::string::String>,
        metrics_summary: ::std::result::Result<super::ToolMetricSummary, ::std::string::String>,
    }
    impl ::std::default::Default for ProfilePart {
        fn default() -> Self {
            Self {
                details: Err("no value supplied for details".to_string()),
                metrics_summary: Err("no value supplied for metrics_summary".to_string()),
            }
        }
    }
    impl ProfilePart {
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EitherOrBoth>,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
        pub fn metrics_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolMetricSummary>,
            T::Error: ::std::fmt::Display,
        {
            self.metrics_summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metrics_summary: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProfilePart> for super::ProfilePart {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProfilePart,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                details: value.details?,
                metrics_summary: value.metrics_summary?,
            })
        }
    }
    impl ::std::convert::From<super::ProfilePart> for ProfilePart {
        fn from(value: super::ProfilePart) -> Self {
            Self {
                details: Ok(value.details),
                metrics_summary: Ok(value.metrics_summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProfileTotal {
        regressions:
            ::std::result::Result<::std::vec::Vec<super::ToolRegression>, ::std::string::String>,
        summary: ::std::result::Result<super::ToolMetricSummary, ::std::string::String>,
    }
    impl ::std::default::Default for ProfileTotal {
        fn default() -> Self {
            Self {
                regressions: Err("no value supplied for regressions".to_string()),
                summary: Err("no value supplied for summary".to_string()),
            }
        }
    }
    impl ProfileTotal {
        pub fn regressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ToolRegression>>,
            T::Error: ::std::fmt::Display,
        {
            self.regressions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for regressions: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolMetricSummary>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProfileTotal> for super::ProfileTotal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProfileTotal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                regressions: value.regressions?,
                summary: value.summary?,
            })
        }
    }
    impl ::std::convert::From<super::ProfileTotal> for ProfileTotal {
        fn from(value: super::ProfileTotal) -> Self {
            Self {
                regressions: Ok(value.regressions),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryOutput {
        format: ::std::result::Result<super::SummaryFormat, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SummaryOutput {
        fn default() -> Self {
            Self {
                format: Err("no value supplied for format".to_string()),
                path: Err("no value supplied for path".to_string()),
            }
        }
    }
    impl SummaryOutput {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SummaryFormat>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SummaryOutput> for super::SummaryOutput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummaryOutput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                path: value.path?,
            })
        }
    }
    impl ::std::convert::From<super::SummaryOutput> for SummaryOutput {
        fn from(value: super::SummaryOutput) -> Self {
            Self {
                format: Ok(value.format),
                path: Ok(value.path),
            }
        }
    }
}
