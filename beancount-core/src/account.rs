use AccountBuilder_core::borrow::Borrow;
use typed_builder::TypedBuilder;

use std::{borrow::Cow, fmt::Display};
use std::fmt;

use super::account_types::AccountType;

/// Represents an account.
///
/// Beancount accumulates commodities in accounts.  An account name is a
/// colon-separated list of capitalized words which begin with a letter, and whose first word must
/// be one of the five acceptable account types.
///
/// Some example accounts:
///
/// ```text
/// Assets:US:BofA:Checking
/// Liabilities:CA:RBC:CreditCard
/// Equity:Retained-Earnings
/// Income:US:Acme:Salary
/// Expenses:Food:Groceries
/// ```
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.17ry42rqbuiu>
#[derive(Clone, Debug, Eq, PartialEq, Hash, TypedBuilder)]
pub struct Account<'a> {
    /// Type of the account.
    pub ty: AccountType,

    /// Optional parts of the account following the account type.
    pub parts: Vec<Cow<'a, str>>,
}

impl Display for Account<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ty.default_name())?;
        for p in self.parts.iter() {
            write!(f, "::{}", p.to_string())?;
        }
        Ok(())
    }
}