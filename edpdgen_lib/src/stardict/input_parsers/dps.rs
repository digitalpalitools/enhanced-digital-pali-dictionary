use crate::input::dps::DpsPaliWord;
use crate::input::make_sort_key;
use crate::stardict::input_parsers::{make_group_id, make_toc_id};
use crate::stardict::StarDictPaliWord;
use pls_core_extras::inflection_generator::InflectionGenerator;
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

#[derive(Serialize)]
struct WordDataViewModel<'a> {
    word: &'a DpsPaliWord,
    toc_id: &'a str,
    dict_short_name: &'a str,
    feedback_form_url: &'a str,
    host_url: &'a str,
    host_version: &'a str,
    inflection_table: &'a str,
}

impl StarDictPaliWord for DpsPaliWord {
    fn id(&self) -> &str {
        &self.pali
    }

    fn sort_key(&self) -> String {
        make_sort_key(self.id())
    }

    fn group_id(&self) -> String {
        make_group_id(self.id())
    }

    fn toc_id(&self, dict_short_name: &str) -> String {
        make_toc_id(self.id(), dict_short_name)
    }

    fn toc_entry(&self, dict_short_name: &str, concise: bool) -> Result<String, String> {
        let mut context = Context::new();
        context.insert("toc_id", &self.toc_id(dict_short_name));
        context.insert("pali", &self.pali);
        context.insert("fin", &self.fin);
        context.insert("pos", &self.pos);
        context.insert("in_english", &self.in_english);
        context.insert("concise", &concise);

        TEMPLATES
            .render("dps_toc_summary", &context)
            .map_err(|e| e.to_string())
    }

    fn word_data_entry(
        &self,
        dict_short_name: &str,
        feedback_form_url: &str,
        host_url: &str,
        host_version: &str,
        igen: &dyn InflectionGenerator,
        concise: bool,
    ) -> Result<String, String> {
        if concise {
            Ok("".to_string())
        } else {
            let vm = WordDataViewModel {
                word: self,
                toc_id: &self.toc_id(dict_short_name),
                dict_short_name,
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

    #[test_case(0, false)]
    #[test_case(1, false)]
    #[test_case(2, false)]
    #[test_case(3, false)]
    #[test_case(4, false)]
    #[test_case(0, true)]
    #[test_case(1, true)]
    #[test_case(2, true)]
    #[test_case(3, true)]
    #[test_case(4, true)]

    fn toc_summary_tests(rec_number: usize, concise: bool) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpsPaliWord>(&get_csv_path(), &l).expect("unexpected");

        let toc_summary = recs
            .nth(rec_number)
            .map(|r| r.toc_entry("dps", concise).expect("unexpected"))
            .expect("unexpected");

        insta::assert_snapshot!(toc_summary);
    }

    #[test_case(0, false)]
    #[test_case(1, false)]
    #[test_case(2, false)]
    #[test_case(3, false)]
    #[test_case(4, false)]
    #[test_case(4, true)]
    fn word_data_tests(rec_number: usize, concise: bool) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpsPaliWord>(&get_csv_path(), &l).expect("unexpected");
        let igen = TestInflectionGenerator::new();

        let word_data = recs
            .nth(rec_number)
            .map(|r| {
                r.word_data_entry("dps", "fb_url", "host url", "host version", &igen, concise)
                    .expect("unexpected")
            })
            .expect("unexpected");

        insta::assert_snapshot!(word_data);
    }
}
