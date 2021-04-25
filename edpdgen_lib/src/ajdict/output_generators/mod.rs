use crate::ajdict::AjDictPaliWord;
use crate::{DictionaryFile, EdpdLogger};

fn create_dict_entries(
    words: impl Iterator<Item = impl AjDictPaliWord>,
    logger: &dyn EdpdLogger,
) -> Result<Vec<String>, String> {
    logger.info(&"Creating dict entries.".to_string());

    let mut dict_entries: Vec<String> = Vec::new();
    for (n, word) in words.into_iter().enumerate() {
        dict_entries.push(word.word_data_entry()?);

        if n % 1_000 == 0 && n != 0 {
            logger.info(&format!(
                "... created {:05} dict entries, ending with '{}'.",
                n,
                word.id()
            ));
        }
    }

    logger.info(&format!(
        "... done creating {} dict entries.",
        dict_entries.len()
    ));

    Ok(dict_entries)
}

fn create_txt_data(dict_entries: Vec<String>, logger: &dyn EdpdLogger) -> Vec<u8> {
    logger.info(&format!(
        "Creating dict data for {} txt entries.",
        &dict_entries.len()
    ));

    let txt: Vec<u8> = dict_entries.iter().fold(Vec::new(), |mut acc, e| {
        acc.append(&mut e.to_owned().into_bytes());
        acc.push(0x0d);
        acc.push(0x0a);
        acc
    });

    logger.info(&format!(
        "... done creating {} idx entries.",
        &dict_entries.len()
    ));

    txt
}

pub fn create_dictionary(
    words: impl Iterator<Item = impl AjDictPaliWord>,
    logger: &dyn EdpdLogger,
) -> Result<Vec<DictionaryFile>, String> {
    let dict_entries = create_dict_entries(words, logger)?;
    let txt = create_txt_data(dict_entries, logger);

    Ok(vec![DictionaryFile {
        extension: "ajd.txt".to_string(),
        data: txt,
    }])
}

#[cfg(test)]
mod tests {
    // use crate::{resolve_file_in_manifest_dir, InputFormat, OutputFormat, DictionaryInfo};
    // use crate::ajdict::AjDictPaliWord;
    //
    // #[derive(Debug, Deserialize)]
    // struct TestPaliWord {
    //     id: String,
    //     sort_key: String,
    //     group_id: String,
    //     toc_id: String,
    //     toc_entry: String,
    //     word_data_entry: String,
    // }
    //
    // impl AjDictPaliWord for TestPaliWord {
    //     fn id(&self) -> &str {
    //         &self.id
    //     }
    //
    //     fn sort_key(&self) -> String {
    //         self.sort_key.clone()
    //     }
    //
    //     fn word_data_entry(&self) -> Result<String, String> {
    //         Ok(format!("{}",self.word_data_entry))
    //     }
    // }
    //
    // fn read_pali_words<'a>() -> impl Iterator<Item = impl AjDictPaliWord> + 'a {
    //     let path = resolve_file_in_manifest_dir(
    //         "src/stardict/output_generators/test_data/pali_words1.csv",
    //     )
    //     .expect("must exist!");
    //
    //     let file = std::fs::File::open(path).expect("must exist");
    //     let rdr = csv::ReaderBuilder::new().from_reader(file);
    //
    //     rdr.into_deserialize::<TestPaliWord>().map(|w| w.expect(""))
    // }
    //
    // fn create_dict_info<'a>() -> DictionaryInfo<'a> {
    //     DictionaryInfo {
    //         name: "Digital Pāli Tools Dictionary (DPD)",
    //         input_format: &InputFormat::Dps,
    //         output_format: &OutputFormat::StarDict,
    //         author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
    //         description: "The next generation comprehensive digital Pāli dictionary.",
    //         accent_color: "orange",
    //         time_stamp: "xxxx",
    //         ico: &[],
    //         feedback_form_url: "http://feedback.form/???",
    //         host_url: "this is the host",
    //         host_version: "host version",
    //         inflections_db_path: None,
    //     }
    // }

    // #[test]
    // fn create_dict_test() {
    //     let words = read_pali_words();
    //     let igen = TestInflectionGenerator::new();
    //
    //     let (dict_data, idx_entries) =
    //         create_dict(&create_dict_info(), words, &igen, &TestLogger::new()).expect("Unexpected");
    //     let dict_entries: Vec<String> = idx_entries
    //         .iter()
    //         .map(|ie| {
    //             let word_bytes =
    //                 &dict_data[ie.data_offset as usize..(ie.data_offset + ie.data_size) as usize];
    //             std::str::from_utf8(word_bytes)
    //                 .expect("unexpected")
    //                 .to_owned()
    //         })
    //         .collect();
    //
    //     insta::assert_debug_snapshot!(dict_entries);
    //     insta::assert_yaml_snapshot!(idx_entries);
    // }
}
