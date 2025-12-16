use super::{ColumnDef, Statement};

use toasty_core::{
    driver::Capability,
    schema::db::{Table, TableId},
    stmt,
};

#[derive(Debug, Clone)]
pub struct CreateTable {
    /// Name of the table
    pub table: TableId,

    /// Column definitions
    pub columns: Vec<ColumnDef>,

    /// Primary key clause
    pub primary_key: Option<Box<stmt::Expr>>,

    /// Whether or not to add an `IF NOT EXISTS` clause.
    pub if_not_exists: bool,
}

impl Statement {
    /// Creates a table.
    ///
    /// This function _does not_ add an `IF NOT EXISTS` clause.
    pub fn create_table(table: &Table, capability: &Capability) -> Self {
        CreateTable {
            table: table.id,
            columns: table
                .columns
                .iter()
                .map(|column| ColumnDef::from_schema(column, &capability.storage_types))
                .collect(),
            primary_key: Some(Box::new(stmt::Expr::record(
                table
                    .primary_key
                    .columns
                    .iter()
                    .map(|col| stmt::Expr::column(*col)),
            ))),
            if_not_exists: false,
        }
        .into()
    }

    /// Creates a table.
    ///
    /// This function _does_ add an `IF NOT EXISTS` clause.
    pub fn create_table_if_not_exists(table: &Table, capability: &Capability) -> Self {
        CreateTable {
            table: table.id,
            columns: table
                .columns
                .iter()
                .map(|column| ColumnDef::from_schema(column, &capability.storage_types))
                .collect(),
            primary_key: Some(Box::new(stmt::Expr::record(
                table
                    .primary_key
                    .columns
                    .iter()
                    .map(|col| stmt::Expr::column(*col)),
            ))),
            if_not_exists: true,
        }
        .into()
    }
}

impl From<CreateTable> for Statement {
    fn from(value: CreateTable) -> Self {
        Self::CreateTable(value)
    }
}
