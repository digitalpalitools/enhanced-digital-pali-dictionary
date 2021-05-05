use crate::ajdict::AjDictPaliWord;
use crate::{DictionaryFile, EdpdLogger};

fn create_dict_entries(
    words: impl Iterator<Item = impl AjDictPaliWord>,
    logger: &dyn EdpdLogger,
) -> Result<(Vec<String>, Vec<String>), String> {
    logger.info(&"Creating dict entries.".to_string());

    let mut dpd_entries: Vec<String> = Vec::new();
    let mut cdpd_entries: Vec<String> = Vec::new();
    for (n, word) in words.into_iter().enumerate() {
        dpd_entries.push(word.word_data_entry()?);
        cdpd_entries.push(word.concise_word_data_entry()?);

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
        dpd_entries.len()
    ));

    Ok((dpd_entries, cdpd_entries))
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
    let (dpd_entries, cdpd_entries) = create_dict_entries(words, logger)?;
    let dpd_txt = create_txt_data(dpd_entries, logger);
    let cdpd_txt = create_txt_data(cdpd_entries, logger);

    Ok(vec![
        DictionaryFile {
            extension: "dpd.ajd.txt".to_string(),
            bom: vec![0xEF, 0xBB, 0xBF],
            data: dpd_txt,
            can_be_empty: false,
        },
        DictionaryFile {
            extension: "cdpd.ajd.txt".to_string(),
            bom: vec![0xEF, 0xBB, 0xBF],
            data: cdpd_txt,
            can_be_empty: false,
        },
    ])
}

#[cfg(test)]
mod tests {
    use crate::ajdict::output_generators::create_txt_data;
    use crate::tests::TestLogger;

    #[test]
    fn create_dict_test() {
        let dict_entries = vec!["a;1;2".to_string(), "b;3;4".to_string()];

        let txt_data = create_txt_data(dict_entries, &TestLogger::new());

        assert_eq!(
            txt_data,
            vec![97, 59, 49, 59, 50, 13, 10, 98, 59, 51, 59, 52, 13, 10]
        );
    }
}
