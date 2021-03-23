#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use regex::{Captures, Regex};
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "toc_summary",
            include_str!("templates/toc_summary.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.add_raw_templates(vec![(
            "word_data",
            include_str!("templates/word_data.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.autoescape_on(vec!["html"]);
        tera
    };
    static ref PALI1_CRACKER: Regex = Regex::new(r"(.*)( )(\d+)$").expect("Malformed regex string");
}

trait PaliWord {
    fn sort_key(&self) -> String;
    fn group_id(&self) -> String;
    fn toc_id(&self) -> String;
    fn include_in_dictionary(&self) -> bool;
    fn toc_summary(&self) -> Result<String, String>;
    fn word_data(&self) -> Result<String, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DpdPaliWord {
    #[serde(rename = "Pāli1")]
    pali1: String,
    #[serde(rename = "Pāli2")]
    pali2: String,
    #[serde(rename = "Fin")]
    fin: String,
    #[serde(rename = "POS")]
    pos: String,
    #[serde(rename = "Grammar")]
    grammar: String,
    #[serde(rename = "Derived from")]
    derived_from: String,
    #[serde(rename = "Neg")]
    neg: String,
    #[serde(rename = "Verb")]
    verb: String,
    #[serde(rename = "Trans")]
    trans: String,
    #[serde(rename = "Case")]
    case: String,
    #[serde(rename = "Meaning IN CONTEXT")]
    in_english: String,
    #[serde(rename = "Sanskrit")]
    sanskrit: String,
    #[serde(rename = "Sk Root")]
    sanskrit_root: String,
    #[serde(rename = "Family")]
    family: String,
    #[serde(rename = "Pāli Root")]
    pali_root: String,
    #[serde(rename = "V")]
    v: String,
    #[serde(rename = "Grp")]
    grp: String,
    #[serde(rename = "Sgn")]
    sgn: String,
    #[serde(rename = "Root Meaning")]
    root_meaning: String,
    #[serde(rename = "Base")]
    base: String,
    #[serde(rename = "Construction")]
    construction: String,
    #[serde(rename = "Derivative")]
    derivative: String,
    #[serde(rename = "Suffix")]
    suffix: String,
    #[serde(rename = "Compound")]
    compound: String,
    #[serde(rename = "Compound Construction")]
    compound_construction: String,
    #[serde(rename = "Source1")]
    source1: String,
    #[serde(rename = "Sutta1")]
    sutta1: String,
    #[serde(rename = "Example1")]
    example1: String,
    #[serde(rename = "Source 2")]
    source2: String,
    #[serde(rename = "Sutta2")]
    sutta2: String,
    #[serde(rename = "Example 2")]
    example2: String,
    #[serde(rename = "Antonyms")]
    antonyms: String,
    #[serde(rename = "Synonyms – different word")]
    synonyms: String,
    #[serde(rename = "Variant – same constr or diff reading")]
    variant: String,
    #[serde(rename = "Commentary")]
    commentary: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Stem")]
    stem: String,
    #[serde(rename = "Pattern")]
    pattern: String,
    #[serde(rename = "Buddhadatta")]
    buddhadatta: String,
    #[serde(rename = "3")]
    two: String,
}

#[derive(Serialize)]
struct WordDataViewModel<'a> {
    word: &'a DpdPaliWord,
    toc_id: &'a str,
    short_name: &'a str,
}

impl PaliWord for DpdPaliWord {
    fn sort_key(&self) -> String {
        let sk = PALI1_CRACKER.replace(&self.pali1, |caps: &Captures| {
            // NOTE: Best case effort. Not sweating it.
            let n = &caps[3].parse::<i32>().unwrap_or(0);
            format!("{} {:03}", &caps[1], n)
        });

        sk.into_owned()
    }

    fn group_id(&self) -> String {
        let gid = PALI1_CRACKER.replace(&self.pali1, |caps: &Captures| caps[1].to_string());

        gid.into_owned()
    }

    fn toc_id(&self) -> String {
        self.pali1.replace(" ", "_") + "_dpd"
    }

    fn include_in_dictionary(&self) -> bool {
        !self.in_english.is_empty()
    }

    fn toc_summary(&self) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("toc_id", &self.toc_id());
        context.insert("pali1", &self.pali1);
        context.insert("pos", &self.pos);
        context.insert("in_english", &self.in_english);

        TEMPLATES
            .render("toc_summary", &context)
            .map_err(|e| e.to_string())
    }

    fn word_data(&self) -> Result<String, String> {
        let vm = WordDataViewModel {
            word: &self,
            toc_id: &self.toc_id(),
            short_name: "dpd",
        };

        let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
        TEMPLATES
            .render("word_data", &context)
            .map_err(|e| e.to_string())
    }
}

pub fn read_records(
    path: &str,
) -> Result<impl Iterator<Item = Result<DpdPaliWord, String>>, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    let records = rdr.into_deserialize::<DpdPaliWord>().map(|r| {
        let rec: Result<DpdPaliWord, String> = r.map_err(|e| e.to_string());
        rec
    });

    Ok(records)
}

fn get_csv_path() -> String {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("Pali_English_Dictionary_10_rows-full.csv");

    d.to_str().unwrap_or("Not able to resolve path!").to_owned()
}

pub fn run() -> Result<(), String> {
    let mut recs = read_records(&get_csv_path())?;

    let sk = recs
        .nth(11)
        .expect("unexpected")
        .map(|r| r.sort_key())
        .expect("unexpected");

    println!("!>>> {}", sk);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0, "ābādha"; "0 digits")]
    #[test_case(9, "adhikāra 001"; "1 digit")]
    #[test_case(11, "adhikāra 010"; "2 digits")]
    fn test_sort_key(rec_number: usize, expected_sk: &str) {
        let mut recs = read_records(&get_csv_path()).expect("failed to load");

        let sk = recs
            .nth(rec_number)
            .expect("unexpected")
            .map(|r| r.sort_key())
            .expect("unexpected");

        assert_eq!(sk, expected_sk)
    }

    #[test_case(4, "abahulīkata"; "0 digits")]
    #[test_case(5, "abala"; "1 digit")]
    #[test_case(11, "adhikāra"; "2 digits")]
    fn test_group_id(rec_number: usize, expected_gid: &str) {
        let mut recs = read_records(&get_csv_path()).expect("failed to load");

        let gid = recs
            .nth(rec_number)
            .expect("unexpected")
            .map(|r| r.group_id())
            .expect("unexpected");

        assert_eq!(gid, expected_gid);
    }

    #[test_case(4, "abahulīkata_dpd"; "0 digits")]
    #[test_case(5, "abala_1_dpd"; "1 digit")]
    #[test_case(11, "adhikāra_10_dpd"; "2 digits")]
    fn test_toc_id(rec_number: usize, expected_toc_id: &str) {
        let mut recs = read_records(&get_csv_path()).expect("unexpected");

        let toc_id = recs
            .nth(rec_number)
            .expect("unexpected")
            .map(|r| r.toc_id())
            .expect("unexpected");

        assert_eq!(toc_id, expected_toc_id);
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(5)]
    #[test_case(6)]
    #[test_case(7)]
    #[test_case(8)]
    #[test_case(9)]
    #[test_case(10)]
    #[test_case(11)]
    #[test_case(12)]
    fn toc_summary_tests(rec_number: usize) {
        let mut recs = read_records(&get_csv_path()).expect("unexpected");

        let toc_summary = recs
            .nth(rec_number)
            .expect("unexpected")
            .and_then(|r| r.toc_summary())
            .expect("unexpected");

        insta::assert_snapshot!(toc_summary);
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(5)]
    #[test_case(6)]
    #[test_case(7)]
    #[test_case(8)]
    #[test_case(9)]
    #[test_case(10)]
    #[test_case(11)]
    #[test_case(12)]
    fn word_data_tests(rec_number: usize) {
        let mut recs = read_records(&get_csv_path()).expect("unexpected");

        let word_data = recs
            .nth(rec_number)
            .expect("unexpected")
            .and_then(|r| r.word_data())
            .expect("unexpected");

        insta::assert_snapshot!(word_data);
    }
}
