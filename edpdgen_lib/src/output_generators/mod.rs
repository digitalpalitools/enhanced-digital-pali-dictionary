use crate::input_parsers::PaliWord;
use crate::EdpdLogger;
use itertools::Itertools;
use std::cmp::Ordering;

// use std::fs::File;
// use std::io::Write;

use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![(
            "word_group",
            include_str!("templates/word_group.html"),
        )])
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

/**
 * From: https://stackoverflow.com/a/13225961/6196679
 *
 * Compares two strings, ignoring the case of ASCII characters. It treats
 * non-ASCII characters taking in account case differences. This is an
 * attempt to mimic glib's string utility function
 * <a href="http://developer.gnome.org/glib/2.28/glib-String-Utility-Functions.html#g-ascii-strcasecmp">g_ascii_strcasecmp ()</a>.
 *
 * This is a slightly modified version of java.lang.String.CASE_INSENSITIVE_ORDER.compare(String s1, String s2) method.
 *
 * @param str1  string to compare with str2
 * @param str2  string to compare with str1
 * @return      0 if the strings match, a negative value if str1 < str2, or a positive value if str1 > str2
 */
pub fn g_ascii_strcasecmp(str1: &str, str2: &str) -> Ordering {
    let str_vec1: Vec<char> = str1.chars().collect();
    let str_vec2: Vec<char> = str2.chars().collect();
    let n1 = str_vec1.len();
    let n2 = str_vec2.len();
    let c127 = 127 as char;

    let min = n1.min(n2);
    for i in 0..min {
        let c1 = str_vec1[i];
        let c2 = str_vec2[i];
        if c1 != c2 {
            if c1 > c127 || c2 > c127 {
                // If non-ASCII char...
                return c1.cmp(&c2);
            } else {
                let c1uc = c1.to_ascii_uppercase();
                let c2uc = c2.to_ascii_uppercase();
                if c1uc != c2uc {
                    let c1lc = c1.to_ascii_lowercase();
                    let c2lc = c2.to_ascii_lowercase();
                    if c1lc != c2lc {
                        return c1lc.cmp(&c2lc);
                    }
                }
            }
        }
    }

    n1.cmp(&n2)
}

#[derive(Serialize)]
struct WordGroupViewModel<'a> {
    ods_type: &'a str,
    accent_color: &'a str,
    toc_entries: &'a [String],
    descriptions: &'a [String],
}

fn create_html_for_word_group(
    ods_type: &str,
    words: impl Iterator<Item = impl PaliWord>,
    _logger: &impl EdpdLogger,
) -> Result<String, String> {
    let mut word_info: Vec<(String, String, String)> = words
        .map(|w| {
            (
                w.sort_key(),
                w.toc_entry().unwrap_or_else(|e| e),
                w.word_data_entry().unwrap_or_else(|e| e),
            )
        })
        .collect();
    word_info.sort_by(|a, b| a.0.cmp(&b.0));

    let toc_entries: Vec<String> = word_info.iter().map(|w| w.1.to_owned()).collect();
    let descriptions: Vec<String> = word_info.iter().map(|w| w.2.to_owned()).collect();

    let vm = WordGroupViewModel {
        ods_type,
        accent_color: "orange",
        toc_entries: &toc_entries,
        descriptions: &descriptions,
    };

    let context = Context::from_serialize(&vm).map_err(|e| e.to_string())?;
    TEMPLATES
        .render("word_group", &context)
        .map_err(|e| e.to_string())
}

fn create_dict(
    ods_type: &str,
    words: impl Iterator<Item = impl PaliWord>,
    logger: &impl EdpdLogger,
) -> Result<(Vec<u8>, Vec<IdxEntry>), String> {
    let word_groups = words.group_by(|pw| pw.group_id());

    let mut dict_buffer: Vec<u8> = Vec::new();
    let mut idx_words: Vec<IdxEntry> = Vec::new();
    for (n, (key, word_group)) in (&word_groups).into_iter().enumerate() {
        let html_str = create_html_for_word_group(ods_type, word_group, logger)?;
        let mut html_bytes = html_str.into_bytes();

        // //if key.eq("ajapada") {//"abbahe") {
        // logger.info(&format!("Writing {}...", key));
        // let mut dict_file = File::create(format!("d:/delme/dicts-rust/words/{}.txt", key)).map_err(|e| e.to_string()).unwrap();
        // dict_file.write_all(&html_bytes).unwrap();
        // //}

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
    }

    Ok((dict_buffer, idx_words))
}

fn create_idx(idx_entries: &mut Vec<IdxEntry>, logger: &impl EdpdLogger) -> Vec<u8> {
    idx_entries.sort_by(|w1, w2| g_ascii_strcasecmp(&w1.word, &w2.word));

    let idx: Vec<u8> = idx_entries.iter().fold(Vec::new(), |mut acc, e| {
        acc.append(&mut e.word.to_owned().into_bytes());
        acc.push(0u8);
        acc.extend_from_slice(&e.data_offset.to_be_bytes());
        acc.extend_from_slice(&e.data_size.to_be_bytes());
        acc
    });

    logger.info(&format!("Created {} idx entries.", &idx_entries.len()));
    idx
}

pub fn write_dictionary(
    ods_type: &str,
    words: impl Iterator<Item = impl PaliWord>,
    logger: &impl EdpdLogger,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let (dict, mut idx_entries) = create_dict(ods_type, words, logger)?;

    let idx = create_idx(&mut idx_entries, logger);

    Ok((dict, idx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_file_in_manifest_dir;
    use crate::tests::TestLogger;

    #[derive(Debug, Deserialize)]
    struct TestPaliWord {
        sort_key: String,
        group_id: String,
        toc_id: String,
        toc_entry: String,
        word_data_entry: String,
    }

    impl PaliWord for TestPaliWord {
        fn sort_key(&self) -> String {
            self.sort_key.clone()
        }

        fn group_id(&self) -> String {
            self.group_id.clone()
        }

        fn toc_id(&self) -> String {
            self.toc_id.clone()
        }

        fn toc_entry(&self) -> Result<String, String> {
            Ok(self.toc_entry.clone())
        }

        fn word_data_entry(&self) -> Result<String, String> {
            Ok(self.word_data_entry.clone())
        }
    }

    fn read_pali_words<'a>() -> impl Iterator<Item = impl PaliWord> + 'a {
        let path = resolve_file_in_manifest_dir("src/output_generators/test_data/pali_words1.csv")
            .expect("must exist!");

        let file = std::fs::File::open(path).expect("must exist");
        let rdr = csv::ReaderBuilder::new().from_reader(file);

        rdr.into_deserialize::<TestPaliWord>().map(|w| w.expect(""))
    }

    #[test]
    fn create_dict_test() {
        let words = read_pali_words();

        let (dict_data, idx_entries) =
            create_dict("dpd", words, &TestLogger::new()).expect("Unexpected");
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

        insta::assert_yaml_snapshot!(idx_entries);
        insta::assert_debug_snapshot!(dict_entries);
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
}
