use crate::inflection_generator::InflectionGenerator;
use crate::stardict::input_parsers::PaliWord;
use crate::stardict::{glib, StarDictFile};
use crate::{DictionaryInfo, EdpdLogger};
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
    synonym_words: Vec<String>,
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
    name: &'a str,
    word_count: usize,
    syn_word_count: usize,
    idx_file_size: usize,
    author: &'a str,
    description: &'a str,
    time_stamp: &'a str,
}

fn log_return_error(
    w: &dyn PaliWord,
    short_msg: &str,
    e: String,
    logger: &dyn EdpdLogger,
) -> String {
    logger.warning(&format!(
        "Failed to generate {} for '{}'. Error: {}.",
        short_msg,
        w.id(),
        e
    ));
    e
}

fn get_ids_and_html_for_word_group(
    dict_info: &DictionaryInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<(Vec<String>, String), String> {
    let mut word_info: Vec<(String, String, String, String)> = words
        .map(|w| {
            (
                w.sort_key(),
                w.id().to_string(),
                w.toc_entry(&dict_info.input_format)
                    .unwrap_or_else(|e| log_return_error(&w, "table of contents", e, logger)),
                w.word_data_entry(
                    &dict_info.input_format,
                    dict_info.feedback_form_url,
                    dict_info.host_url,
                    dict_info.host_version,
                    igen,
                )
                .unwrap_or_else(|e| log_return_error(&w, "word data", e, logger)),
            )
        })
        .collect();
    word_info.sort_by(|a, b| a.0.cmp(&b.0));

    let (ids, toc_entries, descriptions) =
        word_info
            .into_iter()
            .fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, e| {
                acc.0.push(e.1);
                acc.1.push(e.2);
                acc.2.push(e.3);
                acc
            });

    let vm = WordGroupViewModel {
        ods_type: &dict_info.input_format.to_string(),
        accent_color: dict_info.accent_color,
        toc_entries: &toc_entries,
        descriptions: &descriptions,
    };

    let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
    let html = TEMPLATES
        .render("word_group", &context)
        .map_err(|e| e.to_string())?;

    Ok((ids, html))
}

type DictData = (Vec<u8>, Vec<IdxEntry>);

fn create_dict(
    dict_info: &DictionaryInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<DictData, String> {
    logger.info(&"Creating dict entries.".to_string());
    let word_groups = words.group_by(|pw| pw.group_id());

    let mut dict_buffer: Vec<u8> = Vec::new();
    let mut idx_words: Vec<IdxEntry> = Vec::new();
    for (n, (key, word_group)) in (&word_groups).into_iter().enumerate() {
        let (ids, html_str) = get_ids_and_html_for_word_group(dict_info, word_group, igen, logger)?;

        let synonym_words: Vec<String> = ids
            .into_iter()
            .flat_map(|id| igen.generate_all_inflections(&id))
            .collect();
        let mut html_bytes = html_str.into_bytes();
        idx_words.push(IdxEntry {
            word: key,
            data_offset: if n == 0 {
                0
            } else {
                idx_words[n - 1].data_offset + idx_words[n - 1].data_size
            },
            data_size: html_bytes.len() as i32,
            synonym_words,
        });
        dict_buffer.append(&mut html_bytes);

        if n % 1_000 == 0 && n != 0 {
            logger.info(&format!(
                "... created {:05} dict entries, ending with '{}'.",
                n, idx_words[n].word
            ));
        }
    }

    logger.info(&format!(
        "... done creating {} dict entries.",
        idx_words.len()
    ));
    Ok((dict_buffer, idx_words))
}

fn create_idx(idx_entries: &[IdxEntry], logger: &dyn EdpdLogger) -> Vec<u8> {
    logger.info(&format!("Creating {} idx entries.", &idx_entries.len()));

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

struct SynEntry {
    synonym_word: String,
    original_word_index: i32,
}

fn create_syn(idx_entries: &[IdxEntry], logger: &dyn EdpdLogger) -> (Vec<u8>, usize) {
    let mut syn_entries = idx_entries
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (n, e)| {
            let mut ses: Vec<SynEntry> = e
                .synonym_words
                .iter()
                .map(|sw| SynEntry {
                    synonym_word: sw.to_owned(),
                    original_word_index: n as i32,
                })
                .collect();

            acc.append(&mut ses);

            acc
        });

    let syn_count = syn_entries.len();
    logger.info(&format!("Creating {} syn entries.", syn_count));

    syn_entries.sort_by(|w1, w2| glib::stardict_strcmp(&w1.synonym_word, &w2.synonym_word));
    let syn: Vec<u8> = syn_entries.iter().fold(Vec::new(), |mut acc, e| {
        acc.append(&mut e.synonym_word.to_owned().into_bytes());
        acc.push(0u8);
        acc.extend_from_slice(&e.original_word_index.to_be_bytes());
        acc
    });

    logger.info(&format!("... done creating {} syn entries.", syn_count));

    (syn, syn_count)
}

///
/// See https://github.com/huzheng001/stardict-3/blob/master/dict/doc/StarDictFileFormat
///
pub fn create_dictionary(
    dict_info: &DictionaryInfo,
    words: impl Iterator<Item = impl PaliWord>,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<Vec<StarDictFile>, String> {
    let (dict, mut idx_entries) = create_dict(dict_info, words, igen, logger)?;
    idx_entries.sort_by(|w1, w2| glib::stardict_strcmp(&w1.word, &w2.word));
    let idx = create_idx(&idx_entries, logger);
    let (syn, syn_count) = create_syn(&idx_entries, logger);
    let ifo = create_ifo(dict_info, idx_entries.len(), syn_count, idx.len())?;
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
            extension: "syn".to_string(),
            data: syn,
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
    dict_info: &DictionaryInfo,
    word_count: usize,
    syn_word_count: usize,
    idx_file_size: usize,
) -> Result<Vec<u8>, String> {
    let vm = IfoViewModel {
        name: dict_info.name,
        word_count,
        syn_word_count,
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

fn create_png(dict_info: &DictionaryInfo) -> Vec<u8> {
    let mut png = Vec::new();
    png.extend_from_slice(&dict_info.ico);

    png
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestInflectionGenerator, TestLogger};
    use crate::{resolve_file_in_manifest_dir, InputFormat, OutputFormat};

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

        fn toc_id(&self, _input_format: &InputFormat) -> String {
            self.toc_id.clone()
        }

        fn toc_entry(&self, _input_format: &InputFormat) -> Result<String, String> {
            Ok(self.toc_entry.clone())
        }

        fn word_data_entry(
            &self,
            input_format: &InputFormat,
            feedback_form_url: &str,
            host_url: &str,
            host_version: &str,
            igen: &dyn InflectionGenerator,
        ) -> Result<String, String> {
            Ok(format!(
                "{}-{}-{}-{}-{}-{}",
                self.word_data_entry,
                input_format.to_string(),
                feedback_form_url,
                host_url,
                host_version,
                igen.generate_inflection_table_html(self.id())
            ))
        }
    }

    fn read_pali_words<'a>() -> impl Iterator<Item = impl PaliWord> + 'a {
        let path = resolve_file_in_manifest_dir(
            "src/stardict/output_generators/test_data/pali_words1.csv",
        )
        .expect("must exist!");

        let file = std::fs::File::open(path).expect("must exist");
        let rdr = csv::ReaderBuilder::new().from_reader(file);

        rdr.into_deserialize::<TestPaliWord>().map(|w| w.expect(""))
    }

    fn create_dict_info<'a>() -> DictionaryInfo<'a> {
        DictionaryInfo {
            name: "Digital Pāli Tools Dictionary (DPD)",
            input_format: &InputFormat::DevamittaPaliStudy,
            output_format: &OutputFormat::StarDict,
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
        let idx_entries = vec![
            IdxEntry {
                word: "a".to_string(),
                data_offset: 1,
                data_size: 2,
                synonym_words: vec!["a1".to_string(), "a2".to_string()],
            },
            IdxEntry {
                word: "bc".to_string(),
                data_offset: 0x01000100,
                data_size: 0x00020002,
                synonym_words: vec!["b1".to_string(), "b2".to_string()],
            },
        ];

        let idx = create_idx(&idx_entries, &TestLogger::new());

        assert_eq!(
            idx,
            vec![0x61, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0x62, 0x63, 0, 1, 0, 1, 0, 0, 2, 0, 2]
        )
    }

    #[test]
    fn create_syn_test() {
        let idx_entries = vec![
            IdxEntry {
                word: "a".to_string(),
                data_offset: 1,
                data_size: 2,
                synonym_words: vec!["a1".to_string()],
            },
            IdxEntry {
                word: "bc".to_string(),
                data_offset: 0x01000100,
                data_size: 0x00020002,
                synonym_words: vec!["b1".to_string(), "b2".to_string()],
            },
        ];

        let (syn, syn_count) = create_syn(&idx_entries, &TestLogger::new());

        assert_eq!(
            syn,
            vec![0x61, 0x31, 0, 0, 0, 0, 0, 0x62, 0x31, 0, 0, 0, 0, 1, 0x62, 0x32, 0, 0, 0, 0, 1]
        );
        assert_eq!(syn_count, 3);
    }

    #[test]
    fn create_ifo_test() {
        let ifo = create_ifo(&create_dict_info(), 100, 500, 1000).expect("Unexpected");

        insta::assert_snapshot!(&String::from_utf8(ifo).expect("Unexpected"));
    }
}
