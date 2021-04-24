// NOTE: Keep the order deliberately randomized as we support column reordering.
#[derive(Debug, Serialize, Deserialize)]
pub struct DpsPaliWord {
    #[serde(rename = "Pāli1")]
    pub pali: String,
    #[serde(rename = "Fin")]
    pub fin: String,
    #[serde(rename = "POS")]
    pub pos: String,
    #[serde(rename = "Grammar")]
    pub grammar: String,
    #[serde(rename = "Derived from")]
    pub derived_from: String,
    #[serde(rename = "Neg")]
    pub neg: String,
    #[serde(rename = "Verb")]
    pub verb: String,
    #[serde(rename = "Trans")]
    pub trans: String,
    #[serde(rename = "Case")]
    pub case: String,
    #[serde(rename = "Meaning IN CONTEXT")]
    pub in_english: String,
    #[serde(rename = "Meaning in native language")]
    pub in_russian: String,
    #[serde(rename = "Pāli Root")]
    pub pali_root: String,
    #[serde(rename = "Base")]
    pub base: String,
    #[serde(rename = "Construction")]
    pub construction: String,
    #[serde(rename = "Sanskrit")]
    pub sanskrit: String,
    #[serde(rename = "Sk Root")]
    pub sanskrit_root: String,
    #[serde(rename = "Commentary")]
    pub commentary: String,
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "Source1")]
    pub source1: String,
    #[serde(rename = "Example1")]
    pub example1: String,
    #[serde(rename = "Sutta1")]
    pub sutta1: String,
    #[serde(rename = "Source 2")]
    pub source2: String,
    #[serde(rename = "Example 2")]
    pub example2: String,
    #[serde(rename = "Sutta2")]
    pub sutta2: String,
    #[serde(rename = "Chapter")]
    pub chapter: String,
    #[serde(rename = "Test")]
    pub test: String,
    #[serde(rename = "Variant")]
    pub variant: String,
}
