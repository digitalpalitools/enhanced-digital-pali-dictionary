use crate::inflection_generator::InflectionGenerator;
use crate::input_parsers::PaliWord;
use crate::EdpdLogger;
use crate::{glib, StarDictFile, StartDictInfo};
use itertools::Itertools;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "word_group",
            include_str!("templates/word_group.html"),
        )])
        .expect("Unexpected failure adding template");
        tera.add_raw_templates(vec![("ifo_file", include_str!("templates/ifo_file.txt"))])
            .expect("Unexpected failure adding template");
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

#[derive(Debug, Serialize)]
struct IdxEntry {
    word: String,
    data_offset: i32,
    data_size: i32,
}

#[derive(Serialize)]
struct WordGroupViewModel<'a> {
    ods_type: &'a str,
    accent_color: &'a str,
    toc_entries: &'a [String],
    descriptions: &'a [String],
}

#[derive(Serialize)]
struct IfoViewModel<'a> {
    version: &'a str,
    name: &'a str,
    word_count: usize,
    idx_file_size: usize,
    author: &'a str,
    description: &'a str,
    time_stamp: &'a str,
}

fn create_html_for_word_group(
    dict_info: &StartDictInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
) -> Result<String, String> {
    let mut word_info: Vec<(String, String, String)> = words
        .map(|w| {
            (
                w.sort_key(),
                w.toc_entry(dict_info.short_name).unwrap_or_else(|e| e),
                w.word_data_entry(
                    dict_info.short_name,
                    dict_info.feedback_form_url,
                    dict_info.host_url,
                    dict_info.host_version,
                    igen,
                )
                .unwrap_or_else(|e| e),
            )
        })
        .collect();
    word_info.sort_by(|a, b| a.0.cmp(&b.0));

    let toc_entries: Vec<String> = word_info.iter().map(|w| w.1.to_owned()).collect();
    let descriptions: Vec<String> = word_info.iter().map(|w| w.2.to_owned()).collect();

    let vm = WordGroupViewModel {
        ods_type: dict_info.short_name,
        accent_color: dict_info.accent_color,
        toc_entries: &toc_entries,
        descriptions: &descriptions,
    };

    let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
    TEMPLATES
        .render("word_group", &context)
        .map_err(|e| e.to_string())
}

fn create_dict(
    dict_info: &StartDictInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<(Vec<u8>, Vec<IdxEntry>), String> {
    logger.info(&"Creating dict entries.".to_string());
    let word_groups = words.group_by(|pw| pw.group_id());

    let mut dict_buffer: Vec<u8> = Vec::new();
    let mut idx_words: Vec<IdxEntry> = Vec::new();
    for (n, (key, word_group)) in (&word_groups).into_iter().enumerate() {
        let html_str = create_html_for_word_group(dict_info, word_group, igen)?;
        let mut html_bytes = html_str.into_bytes();

        idx_words.push(IdxEntry {
            word: key,
            data_offset: if n == 0 {
                0
            } else {
                idx_words[n - 1].data_offset + idx_words[n - 1].data_size
            },
            data_size: html_bytes.len() as i32,
        });
        dict_buffer.append(&mut html_bytes);

        if n % 1_000 == 0 && n != 0 {
            logger.info(&format!(
                "... created {:05} dict entries, ending with '{}'.",
                n,
                idx_words[idx_words.len() - 1].word
            ));
        }
    }

    logger.info(&format!(
        "... done creating {} dict entries.",
        idx_words.len()
    ));
    Ok((dict_buffer, idx_words))
}

fn create_idx(idx_entries: &mut Vec<IdxEntry>, logger: &dyn EdpdLogger) -> Vec<u8> {
    logger.info(&format!("Creating {} idx entries.", &idx_entries.len()));
    idx_entries.sort_by(|w1, w2| glib::g_ascii_strcasecmp(&w1.word, &w2.word));

    let idx: Vec<u8> = idx_entries.iter().fold(Vec::new(), |mut acc, e| {
        acc.append(&mut e.word.to_owned().into_bytes());
        acc.push(0u8);
        acc.extend_from_slice(&e.data_offset.to_be_bytes());
        acc.extend_from_slice(&e.data_size.to_be_bytes());
        acc
    });

    logger.info(&format!(
        "... done creating {} idx entries.",
        &idx_entries.len()
    ));
    idx
}

pub fn create_dictionary(
    dict_info: &StartDictInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<Vec<StarDictFile>, String> {
    let (dict, mut idx_entries) = create_dict(dict_info, words, igen, logger)?;
    let idx = create_idx(&mut idx_entries, logger);
    let ifo = create_ifo(dict_info, idx_entries.len(), idx.len())?;
    let png = create_png(dict_info);

    Ok(vec![
        StarDictFile {
            extension: "idx".to_string(),
            data: idx,
        },
        StarDictFile {
            extension: "dict".to_string(),
            data: dict,
        },
        StarDictFile {
            extension: "ifo".to_string(),
            data: ifo,
        },
        StarDictFile {
            extension: "png".to_string(),
            data: png,
        },
    ])
}

fn create_ifo(
    dict_info: &StartDictInfo,
    word_count: usize,
    idx_file_size: usize,
) -> Result<Vec<u8>, String> {
    let vm = IfoViewModel {
        version: env!("CARGO_PKG_VERSION"),
        name: dict_info.name,
        word_count,
        idx_file_size,
        author: dict_info.author,
        description: dict_info.description,
        time_stamp: dict_info.time_stamp,
    };

    let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
    let ifo_str = TEMPLATES
        .render("ifo_file", &context)
        .map_err(|e| e.to_string())?;

    Ok(ifo_str.into_bytes())
}

fn create_png(dict_info: &StartDictInfo) -> Vec<u8> {
    let mut png = Vec::new();
    png.extend_from_slice(&dict_info.ico);

    png
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_file_in_manifest_dir;
    use crate::tests::{TestInflectionGenerator, TestLogger};

    #[derive(Debug, Deserialize)]
    struct TestPaliWord {
        id: String,
        sort_key: String,
        group_id: String,
        toc_id: String,
        toc_entry: String,
        word_data_entry: String,
    }

    impl PaliWord for TestPaliWord {
        fn id(&self) -> &str {
            &self.id
        }

        fn sort_key(&self) -> String {
            self.sort_key.clone()
        }

        fn group_id(&self) -> String {
            self.group_id.clone()
        }

        fn toc_id(&self, _short_name: &str) -> String {
            self.toc_id.clone()
        }

        fn toc_entry(&self, _short_name: &str) -> Result<String, String> {
            Ok(self.toc_entry.clone())
        }

        fn word_data_entry(
            &self,
            short_name: &str,
            feedback_form_url: &str,
            host_url: &str,
            host_version: &str,
            igen: &dyn InflectionGenerator,
        ) -> Result<String, String> {
            Ok(format!(
                "{}-{}-{}-{}-{}-{}",
                self.word_data_entry,
                short_name,
                feedback_form_url,
                host_url,
                host_version,
                igen.generate_inflection_table_html(self.id())
            ))
        }
    }

    fn read_pali_words<'a>() -> impl Iterator<Item = impl PaliWord> + 'a {
        let path = resolve_file_in_manifest_dir("src/output_generators/test_data/pali_words1.csv")
            .expect("must exist!");

        let file = std::fs::File::open(path).expect("must exist");
        let rdr = csv::ReaderBuilder::new().from_reader(file);

        rdr.into_deserialize::<TestPaliWord>().map(|w| w.expect(""))
    }

    fn create_dict_info<'a>() -> StartDictInfo<'a> {
        StartDictInfo {
            name: "Digital Pāli Tools Dictionary (DPD)",
            short_name: "dpz",
            author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
            description: "The next generation comprehensive digital Pāli dictionary.",
            accent_color: "orange",
            time_stamp: "xxxx",
            ico: &[],
            feedback_form_url: "http://feedback.form/???",
            host_url: "this is the host",
            host_version: "host version",
            inflections_db_path: None,
        }
    }

    #[test]
    fn create_dict_test() {
        let words = read_pali_words();
        let igen = TestInflectionGenerator::new();

        let (dict_data, idx_entries) =
            create_dict(&create_dict_info(), words, &igen, &TestLogger::new()).expect("Unexpected");
        let dict_entries: Vec<String> = idx_entries
            .iter()
            .map(|ie| {
                let word_bytes =
                    &dict_data[ie.data_offset as usize..(ie.data_offset + ie.data_size) as usize];
                std::str::from_utf8(word_bytes)
                    .expect("unexpected")
                    .to_owned()
            })
            .collect();

        insta::assert_debug_snapshot!(dict_entries);
        insta::assert_yaml_snapshot!(idx_entries);
    }

    #[test]
    fn create_idx_test() {
        let mut idx_entries = vec![
            IdxEntry {
                word: "a".to_string(),
                data_offset: 1,
                data_size: 2,
            },
            IdxEntry {
                word: "bc".to_string(),
                data_offset: 0x01000100,
                data_size: 0x00020002,
            },
        ];

        let idx = create_idx(&mut idx_entries, &TestLogger::new());

        assert_eq!(
            idx,
            vec![0x61, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0x62, 0x63, 0, 1, 0, 1, 0, 0, 2, 0, 2]
        )
    }

    #[test]
    fn create_ifo_test() {
        let ifo = create_ifo(&create_dict_info(), 100, 1000).expect("Unexpected");

        insta::assert_snapshot!(&String::from_utf8(ifo).expect("Unexpected"));
    }
}
