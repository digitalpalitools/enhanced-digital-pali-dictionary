use crate::inflection_generator::InflectionGenerator;
use crate::stardict::input_parsers::{make_group_id, make_sort_key, make_toc_id, PaliWord};
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "dps_toc_summary",
            include_str!("templates/dps_toc_summary.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.add_raw_templates(vec![(
            "dps_word_data",
            include_str!("templates/dps_word_data.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

// NOTE: Keep the order deliberately randomized as we support column reordering.
#[derive(Debug, Serialize, Deserialize)]
pub struct DpsPaliWord {
    #[serde(rename = "Pāli1")]
    pali: String,
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
    #[serde(rename = "Meaning in native language")]
    in_russian: String,
    #[serde(rename = "Pāli Root")]
    pali_root: String,
    #[serde(rename = "Base")]
    base: String,
    #[serde(rename = "Construction")]
    construction: String,
    #[serde(rename = "Sanskrit")]
    sanskrit: String,
    #[serde(rename = "Sk Root")]
    sanskrit_root: String,
    #[serde(rename = "Commentary")]
    commentary: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Source1")]
    source1: String,
    #[serde(rename = "Example1")]
    example1: String,
    #[serde(rename = "Sutta1")]
    sutta1: String,
    #[serde(rename = "Source 2")]
    source2: String,
    #[serde(rename = "Example 2")]
    example2: String,
    #[serde(rename = "Sutta2")]
    sutta2: String,
    #[serde(rename = "Chapter")]
    chapter: String,
    #[serde(rename = "Test")]
    test: String,
    #[serde(rename = "Variant")]
    variant: String,
}

#[derive(Serialize)]
struct WordDataViewModel<'a> {
    word: &'a DpsPaliWord,
    toc_id: &'a str,
    short_name: &'a str,
    feedback_form_url: &'a str,
    host_url: &'a str,
    host_version: &'a str,
    inflection_table: &'a str,
}

impl PaliWord for DpsPaliWord {
    fn id(&self) -> &str {
        &self.pali
    }

    fn sort_key(&self) -> String {
        make_sort_key(&self.id())
    }

    fn group_id(&self) -> String {
        make_group_id(&self.id())
    }

    fn toc_id(&self, short_name: &str) -> String {
        make_toc_id(&self.id(), short_name)
    }

    fn toc_entry(&self, short_name: &str) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("toc_id", &self.toc_id(short_name));
        context.insert("pali", &self.pali);
        context.insert("pos", &self.pos);
        context.insert("in_english", &self.in_english);

        TEMPLATES
            .render("dps_toc_summary", &context)
            .map_err(|e| e.to_string())
    }

    fn word_data_entry(
        &self,
        short_name: &str,
        feedback_form_url: &str,
        host_url: &str,
        host_version: &str,
        igen: &dyn InflectionGenerator,
    ) -> Result<String, String> {
        let vm = WordDataViewModel {
            word: &self,
            toc_id: &self.toc_id(short_name),
            short_name,
            feedback_form_url,
            host_url,
            host_version,
            inflection_table: &igen.generate_inflection_table_html(&self.pali),
        };

        let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
        TEMPLATES
            .render("dps_word_data", &context)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_file_in_manifest_dir;
    use crate::stardict::input_parsers::load_words;
    use crate::tests::{TestInflectionGenerator, TestLogger};
    use std::path::PathBuf;
    use test_case::test_case;

    pub fn get_csv_path() -> PathBuf {
        resolve_file_in_manifest_dir("dps_sample.csv").expect("must exist!")
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    fn toc_summary_tests(rec_number: usize) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpsPaliWord>(&get_csv_path(), &l).expect("unexpected");

        let toc_summary = recs
            .nth(rec_number)
            .map(|r| r.toc_entry("dpy").expect("unexpected"))
            .expect("unexpected");

        insta::assert_snapshot!(toc_summary);
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    fn word_data_tests(rec_number: usize) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpsPaliWord>(&get_csv_path(), &l).expect("unexpected");
        let igen = TestInflectionGenerator::new();

        let word_data = recs
            .nth(rec_number)
            .map(|r| {
                r.word_data_entry("dpy", "fb_url", "host url", "host version", &igen)
                    .expect("unexpected")
            })
            .expect("unexpected");

        insta::assert_snapshot!(word_data);
    }
}
