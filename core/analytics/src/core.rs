use common::error::{AppError, Result};
use lazy_static::lazy_static;
use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::BTreeSet;

lazy_static! {
    static ref DIALECT: GenericDialect = GenericDialect {};
}

pub fn extract_table_names(query: &str) -> Result<BTreeSet<String>> {
    let parsed: Vec<Statement> =
        Parser::parse_sql(&*DIALECT, query).map_err(|e| AppError::Validation(e.to_string()))?;
    let mut acc: BTreeSet<String> = BTreeSet::new();
    parsed.into_iter().for_each(|stmt| {
        acc.extend(stmt.extract_table_names().unwrap());
    });
    Ok(acc)
}

trait ExtractTableNames {
    fn extract_table_names(&self) -> Result<BTreeSet<String>>;
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for Statement {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        match self {
            Statement::Query(q) => q.extract_table_names(),
            Statement::Update {
                table, selection, ..
            } => {
                let mut names = table.extract_table_names()?;
                if let Some(selection) = selection {
                    names.extend(selection.extract_table_names()?);
                }
                Ok(names)
            }
            _ => Err(AppError::Validation("not implemented yet".into())),
        }
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for Query {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        let Query { body, .. } = self;
        body.extract_table_names()
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for SetExpr {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        match self {
            SetExpr::Select(s) => s.extract_table_names(),
            SetExpr::Query(q) => q.extract_table_names(),
            _ => Err(AppError::Validation("not implemented yet".into())),
        }
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for Select {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        let Select { from, .. } = self;
        let mut acc = BTreeSet::new();
        from.iter().for_each(|table_with_joins| {
            acc.extend(table_with_joins.extract_table_names().unwrap());
        });
        Ok(acc)
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for TableWithJoins {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        let TableWithJoins {
            relation, joins, ..
        } = self;
        let mut acc: BTreeSet<String> = relation.extract_table_names()?;
        joins.iter().for_each(|join| {
            acc.extend(join.extract_table_names().unwrap());
        });
        Ok(acc)
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for Join {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        let Join { relation, .. } = self;
        relation.extract_table_names()
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for Expr {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        match self {
            Expr::InSubquery { subquery, .. } => subquery.extract_table_names(),
            _ => Err(AppError::Validation("not implemented yet".into())),
        }
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for TableFactor {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        match self {
            TableFactor::Table { name, .. } => {
                let mut acc = BTreeSet::new();
                name.clone().0.into_iter().for_each(|object_name| {
                    acc.extend(object_name.extract_table_names().unwrap());
                });
                Ok(acc)
            }
            TableFactor::Derived { subquery, .. } => subquery.extract_table_names(),
            _ => Err(AppError::Validation("not implemented yet".into())),
        }
    }
}

// WARNING: This implementation is not full-covering and may not work as expected. It is only used for demonstration purposes.
impl ExtractTableNames for ObjectNamePart {
    fn extract_table_names(&self) -> Result<BTreeSet<String>> {
        match self {
            ObjectNamePart::Identifier(ident) => Ok(BTreeSet::from([ident.value.to_string()])),
            _ => Err(AppError::Validation("not implemented yet".into())),
        }
    }
}
