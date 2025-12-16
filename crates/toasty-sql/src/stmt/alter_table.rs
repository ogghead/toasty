use super::{ColumnDef, Statement};

use toasty_core::schema::db::{Table, TableId};

#[derive(Debug, Clone)]
pub enum Alteration {
    AddColumn(ColumnDef),
}

#[derive(Debug, Clone)]
pub struct AlterTable {
    /// Name of the table
    pub table: TableId,

    pub alteration: Alteration,
}

impl Statement {
    /// Alter a table definition by adding a new columns.
    ///
    /// This function _does_ add an `IF NOT EXISTS` clause.
    pub fn alter_table(table: &Table, alteration: Alteration) -> Self {
        AlterTable {
            table: table.id,
            alteration,
        }
        .into()
    }
}

impl From<AlterTable> for Statement {
    fn from(value: AlterTable) -> Self {
        Self::AlterTable(value)
    }
}
