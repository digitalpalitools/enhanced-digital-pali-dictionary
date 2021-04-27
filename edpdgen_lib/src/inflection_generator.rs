use crate::EdpdLogger;
use pls_core::inflections::{
    generate_all_inflections, generate_inflection_table, host::PlsInflectionsHost,
};
use rusqlite::{Connection, Row, NO_PARAMS};
use std::env;

lazy_static! {
    static ref PLS_INFLECTION_GENERATOR_PREFIX: String =
        env::var("__PLS_INFLECTION_GENERATOR_PREFIX__").unwrap_or_else(|_e| "".to_string());
}

struct SqlAccess {
    connection: Connection,
}

impl SqlAccess {
    fn get_row_cells(&self, row: &Row) -> Vec<String> {
        let cells: Vec<String> = row
            .column_names()
            .iter()
            .map(|&cn| {
                let cell: String = match row.get(cn) {
                    Ok(c) => c,
                    Err(e) => e.to_string(),
                };
                cell
            })
            .collect();

        cells
    }

    fn exec_sql_core(&self, sql: &str) -> rusqlite::Result<Vec<Vec<Vec<String>>>, rusqlite::Error> {
        let mut result: Vec<Vec<Vec<String>>> = Vec::new();
        for s in sql.split(';').filter(|s| !s.trim().is_empty()) {
            let mut stmt = self.connection.prepare(&s)?;
            let mut rows = stmt.query(NO_PARAMS)?;

            let mut table: Vec<Vec<String>> = Vec::new();
            while let Some(row) = rows.next()? {
                table.push(self.get_row_cells(row));
            }
            result.push(table)
        }

        Ok(result)
    }
}

struct PlsHost<'a> {
    sql_access: SqlAccess,
    logger: &'a dyn EdpdLogger,
}

impl<'a> PlsInflectionsHost<'a> for PlsHost<'a> {
    fn get_locale(&self) -> &'a str {
        "en"
    }

    fn get_version(&self) -> &'a str {
        env!("CARGO_PKG_VERSION")
    }

    fn get_url(&self) -> &'a str {
        env!("CARGO_PKG_NAME")
    }

    fn transliterate(&self, s: &str) -> Result<String, String> {
        Ok(s.to_string())
    }

    fn exec_sql_query_core(&self, sql: &str) -> Result<String, String> {
        let table = self
            .sql_access
            .exec_sql_core(&sql)
            .map_err(|x| x.to_string())?;
        serde_json::to_string(&table).map_err(|x| x.to_string())
    }

    fn log_warning(&self, msg: &str) {
        self.logger.warning(msg)
    }
}

pub trait InflectionGenerator {
    fn check_inflection_db(&self, logger: &dyn EdpdLogger) -> Result<(), String>;
    fn generate_inflection_table_html(&self, pali1: &str) -> String;
    fn generate_all_inflections(&self, pali1: &str) -> Vec<String>;
}

pub(crate) struct NullInflectionGenerator {}

impl NullInflectionGenerator {
    pub fn new() -> NullInflectionGenerator {
        NullInflectionGenerator {}
    }
}

impl InflectionGenerator for NullInflectionGenerator {
    fn check_inflection_db(&self, _logger: &dyn EdpdLogger) -> Result<(), String> {
        Ok(())
    }

    fn generate_inflection_table_html(&self, _pali1: &str) -> String {
        "".to_string()
    }

    fn generate_all_inflections(&self, _pali1: &str) -> Vec<String> {
        Vec::new()
    }
}

pub struct PlsInflectionGenerator<'a> {
    inflection_host: PlsHost<'a>,
}

impl<'a> PlsInflectionGenerator<'a> {
    pub fn new(
        inflections_db_path: &'a str,
        logger: &'a dyn EdpdLogger,
    ) -> Result<PlsInflectionGenerator<'a>, String> {
        logger.info(&format!(
            "Open inflections db: '{}'...",
            inflections_db_path
        ));
        let connection = Connection::open(inflections_db_path)
            .map_err(|e| format!("Cannot open db '{}'. Error: {}.", inflections_db_path, e))?;

        let igen = PlsInflectionGenerator {
            inflection_host: PlsHost {
                sql_access: SqlAccess { connection },
                logger,
            },
        };

        Ok(igen)
    }
}

impl<'a> InflectionGenerator for PlsInflectionGenerator<'a> {
    fn check_inflection_db(&self, logger: &dyn EdpdLogger) -> Result<(), String> {
        match self
            .inflection_host
            .exec_sql_query("select * from _version")
        {
            Ok(ver_info) => {
                if ver_info.len() != 1 && ver_info[0].len() != 1 && ver_info[0][0].len() != 3 {
                    let msg =
                        "Invalid inflection db. Unexpected data in _version table.".to_string();
                    logger.error(&msg);
                    return Err(msg);
                }

                let commit_id = &ver_info[0][0][0];
                let repository = &ver_info[0][0][2];
                if commit_id.len() != 40 && repository != "digitalpalitools/inflection-generator" {
                    let msg = format!(
                        "Invalid inflection db: commid_id: {}, repository: {}.",
                        commit_id, repository
                    );
                    logger.error(&msg);
                    return Err(msg);
                }

                logger.info(&format!(
                    "... loaded version: https://github.com/{}#{}.",
                    repository,
                    &commit_id[0..10]
                ));
                Ok(())
            }
            Err(e) => {
                let msg = format!("Unable to load inflection db. Error: {}.", e);
                logger.error(&msg);
                Err(e)
            }
        }
    }

    fn generate_inflection_table_html(&self, pali1: &str) -> String {
        if is_black_listed_word(pali1) {
            return "".to_string();
        }

        match generate_inflection_table(pali1, false, &self.inflection_host) {
            Ok(t) => t,
            Err(e) => {
                self.inflection_host.logger.warning(&format!(
                    "Unable to generate inflection table '{}'. Error: {}.",
                    pali1, e
                ));
                "".to_string()
            }
        }
    }

    fn generate_all_inflections(&self, pali1: &str) -> Vec<String> {
        if is_black_listed_word(pali1) {
            return vec![];
        }

        match generate_all_inflections(pali1, &self.inflection_host) {
            Ok(inflections) => inflections,
            Err(e) => {
                self.inflection_host.logger.warning(&format!(
                    "Unable to generate inflections for '{}'. Error: {}.",
                    pali1, e
                ));
                Vec::new()
            }
        }
    }
}

fn is_black_listed_word(pali1: &str) -> bool {
    let prefix: &str = &PLS_INFLECTION_GENERATOR_PREFIX;
    !prefix.is_empty() && !pali1.starts_with(prefix)
}
