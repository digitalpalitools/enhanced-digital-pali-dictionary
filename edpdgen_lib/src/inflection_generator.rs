use pls_core::inflections::{generate_inflection_table, PlsInflectionsHost};
use rusqlite::{Connection, Row, NO_PARAMS};

struct PlsHost {}

impl<'a> PlsInflectionsHost<'a> for PlsHost {
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
        let table = exec_sql_core(&sql).map_err(|x| x.to_string())?;
        serde_json::to_string(&table).map_err(|x| x.to_string())
    }
}

fn get_row_cells(row: &Row) -> Vec<String> {
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

fn exec_sql_core(sql: &str) -> rusqlite::Result<Vec<Vec<Vec<String>>>, rusqlite::Error> {
    let conn = Connection::open("./inflections.db")?;
    let mut result: Vec<Vec<Vec<String>>> = Vec::new();
    for s in sql.split(';').filter(|s| !s.trim().is_empty()) {
        let mut stmt = conn.prepare(&s)?;
        let mut rows = stmt.query(NO_PARAMS)?;

        let mut table: Vec<Vec<String>> = Vec::new();
        while let Some(row) = rows.next()? {
            table.push(get_row_cells(row));
        }
        result.push(table)
    }

    Ok(result)
}

pub trait InflectionGenerator {
    fn generate_inflection_table_html(&self, pali1: &str) -> String;
}

pub(crate) struct NullInflectionGenerator {}

impl NullInflectionGenerator {
    pub fn new() -> NullInflectionGenerator {
        NullInflectionGenerator {}
    }
}

impl InflectionGenerator for NullInflectionGenerator {
    fn generate_inflection_table_html(&self, _pali1: &str) -> String {
        "<div><strong>?????</strong></div>".to_string()
    }
}

pub(crate) struct PlsInflectionGenerator<'a> {
    inflection_host: &'a dyn PlsInflectionsHost<'a>,
}

impl<'a> PlsInflectionGenerator<'a> {
    pub fn new() -> PlsInflectionGenerator<'a> {
        PlsInflectionGenerator {
            inflection_host: &PlsHost {},
        }
    }
}

impl<'a> InflectionGenerator for PlsInflectionGenerator<'a> {
    fn generate_inflection_table_html(&self, pali1: &str) -> String {
        generate_inflection_table(pali1, self.inflection_host).unwrap_or_else(|e| {
            format!(
                "<div>Unable to generate inflection tables. Error: <strong>{}</strong></div>",
                e
            )
        })
    }
}
