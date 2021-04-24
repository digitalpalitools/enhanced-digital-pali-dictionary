use crate::inflection_generator::InflectionGenerator;
use crate::stardict::input_parsers::{make_group_id, make_sort_key, make_toc_id, PaliWord};
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "dpd_toc_summary",
            include_str!("templates/dpd_toc_summary.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.add_raw_templates(vec![(
            "dpd_word_data",
            include_str!("templates/dpd_word_data.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

// NOTE: Keep the order deliberately randomized as we support column reordering.
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
    #[serde(rename = "Literal Meaning")]
    literal_meaning: String,
    #[serde(rename = "Root In Comps")]
    root_in_compound: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Stem")]
    stem: String,
    #[serde(rename = "Pattern")]
    pattern: String,
    #[serde(rename = "Buddhadatta")]
    buddhadatta: String,
    #[serde(rename = "Cl")]
    cl: String,
}

#[derive(Serialize)]
struct WordDataViewModel<'a> {
    word: &'a DpdPaliWord,
    toc_id: &'a str,
    short_name: &'a str,
    feedback_form_url: &'a str,
    host_url: &'a str,
    host_version: &'a str,
    inflection_table: &'a str,
}

impl PaliWord for DpdPaliWord {
    fn id(&self) -> &str {
        &self.pali1
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
        context.insert("short_name", short_name);
        context.insert("toc_id", &self.toc_id(short_name));
        context.insert("pali1", &self.pali1);
        context.insert("pos", &self.pos);
        context.insert("in_english", &self.in_english);
        context.insert("buddhadatta", &self.buddhadatta);

        TEMPLATES
            .render("dpd_toc_summary", &context)
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
            inflection_table: &igen.generate_inflection_table_html(&self.pali1),
        };

        let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
        TEMPLATES
            .render("dpd_word_data", &context)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stardict::input_parsers::load_words;
    use crate::stardict::input_parsers::tests::get_csv_path;
    use crate::tests::{TestInflectionGenerator, TestLogger};
    use test_case::test_case;

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
        let l = TestLogger::new();
        let mut recs = load_words::<DpdPaliWord>(&get_csv_path(), &l).expect("unexpected");

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
    #[test_case(5)]
    #[test_case(6)]
    #[test_case(7)]
    #[test_case(8)]
    #[test_case(9)]
    #[test_case(10)]
    #[test_case(11)]
    #[test_case(12)]
    fn word_data_tests(rec_number: usize) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpdPaliWord>(&get_csv_path(), &l).expect("unexpected");
        let igen = TestInflectionGenerator::new();

        let word_data = recs
            .nth(rec_number)
            .map(|r| {
                r.word_data_entry("dpd", "fb_url", "host url", "host version", &igen)
                    .expect("unexpected")
            })
            .expect("unexpected");

        insta::assert_snapshot!(word_data);
    }
}
