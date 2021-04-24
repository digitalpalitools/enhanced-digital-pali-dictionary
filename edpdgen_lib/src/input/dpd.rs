// NOTE: Keep the order deliberately randomized as we support column reordering.
#[derive(Debug, Serialize, Deserialize)]
pub struct DpdPaliWord {
    #[serde(rename = "Pāli1")]
    pub pali1: String,
    #[serde(rename = "Pāli2")]
    pub pali2: String,
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
    #[serde(rename = "Sanskrit")]
    pub sanskrit: String,
    #[serde(rename = "Sk Root")]
    pub sanskrit_root: String,
    #[serde(rename = "Family")]
    pub family: String,
    #[serde(rename = "Pāli Root")]
    pub pali_root: String,
    #[serde(rename = "V")]
    pub v: String,
    #[serde(rename = "Grp")]
    pub grp: String,
    #[serde(rename = "Sgn")]
    pub sgn: String,
    #[serde(rename = "Root Meaning")]
    pub root_meaning: String,
    #[serde(rename = "Base")]
    pub base: String,
    #[serde(rename = "Construction")]
    pub construction: String,
    #[serde(rename = "Derivative")]
    pub derivative: String,
    #[serde(rename = "Suffix")]
    pub suffix: String,
    #[serde(rename = "Compound")]
    pub compound: String,
    #[serde(rename = "Compound Construction")]
    pub compound_construction: String,
    #[serde(rename = "Source1")]
    pub source1: String,
    #[serde(rename = "Sutta1")]
    pub sutta1: String,
    #[serde(rename = "Example1")]
    pub example1: String,
    #[serde(rename = "Source 2")]
    pub source2: String,
    #[serde(rename = "Sutta2")]
    pub sutta2: String,
    #[serde(rename = "Example 2")]
    pub example2: String,
    #[serde(rename = "Antonyms")]
    pub antonyms: String,
    #[serde(rename = "Synonyms – different word")]
    pub synonyms: String,
    #[serde(rename = "Variant – same constr or diff reading")]
    pub variant: String,
    #[serde(rename = "Commentary")]
    pub commentary: String,
    #[serde(rename = "Literal Meaning")]
    pub literal_meaning: String,
    #[serde(rename = "Root In Comps")]
    pub root_in_compound: String,
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "Stem")]
    pub stem: String,
    #[serde(rename = "Pattern")]
    pub pattern: String,
    #[serde(rename = "Buddhadatta")]
    pub buddhadatta: String,
    #[serde(rename = "Cl")]
    pub cl: String,
}
