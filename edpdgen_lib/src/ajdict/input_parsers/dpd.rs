use crate::ajdict::AjDictPaliWord;
use crate::input::dpd::DpdPaliWord;
use crate::input::make_sort_key;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "dpd_word_data",
            include_str!("templates/dpd_word_data.html"),
        )])
        .expect("Unexpected failure adding template");
        tera
    };
}

#[derive(Serialize)]
struct WordDataViewModel<'a> {
    word: &'a DpdPaliWord,
}

impl AjDictPaliWord for DpdPaliWord {
    fn id(&self) -> &str {
        &self.pali1
    }

    fn sort_key(&self) -> String {
        make_sort_key(&self.id())
    }

    fn word_data_entry(&self) -> Result<String, String> {
        let vm = WordDataViewModel { word: &self };

        let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
        TEMPLATES
            .render("dpd_word_data", &context)
            .map(|x| x.replace("\r", "").replace("\n", ""))
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ajdict::input_parsers::load_words;
    use crate::resolve_file_in_manifest_dir;
    use crate::tests::TestLogger;
    use std::path::PathBuf;
    use test_case::test_case;

    pub fn get_csv_path() -> PathBuf {
        resolve_file_in_manifest_dir("Pali_English_Dictionary_10_rows-full.csv")
            .expect("must exist!")
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

        let word_data = recs
            .nth(rec_number)
            .map(|r| r.word_data_entry().expect("unexpected"))
            .expect("unexpected");

        insta::assert_snapshot!(word_data);
    }
}
