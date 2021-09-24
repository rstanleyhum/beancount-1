use beancount_parser::parse;
use beancount_core::{Directive, Transaction};
use beancount_core::AccountType::Income;
use beancount_core::Date;
use beancount_core::IncompleteAmount;
use beancount_core::Posting;
use beancount_core::Account;
use beancount_core::AccountType;
use beancount_core::metadata::Meta;
use std::borrow::Cow;
use rust_decimal::Decimal;

fn add_taxes_transaction(t: &mut Transaction) {
    let stan_tax_account = Account {
        ty: AccountType::Expenses,
        parts: vec![Cow::Owned("Taxes-Stan".to_string())]
    };
    let jess_tax_account = Account {
        ty: AccountType::Expenses,
        parts: vec![Cow::Owned("Taxes-Jess".to_string())]
    };
    let jess_taxable: Vec<IncompleteAmount> = t.postings.iter().filter(|p| p.account.parts.contains(&Cow::Borrowed("Income-Jess")))
        .map(|p| p.units.clone()).collect();
    let stan_taxable: Vec<IncompleteAmount> = t.postings.iter().filter(|p| p.account.parts.contains(&Cow::Borrowed("Income-Stan")))
        .map(|p| p.units.clone()).collect(); 
    let jess_taxes: Vec<IncompleteAmount> = jess_taxable.iter().map(|i| {
        let mut two = Decimal::new(20, 1);
        two.set_sign_negative(true);
        let mut result = i.clone();
        if let Some(amt) = result.num {
            result.num = Some((amt/two).round_dp(2))
        }
        result
    }).collect();
    let stan_taxes: Vec<IncompleteAmount> = stan_taxable.iter().map(|i| {
        let mut two = Decimal::new(20, 1);
        two.set_sign_negative(true);
        let mut result = i.clone();
        if let Some(amt) = result.num {
            result.num = Some((amt/two).round_dp(2))
        }
        result
    }).collect();
    let jess_postings: Vec<Posting> = jess_taxes.iter().map(|i| {
        Posting {
            account: jess_tax_account.clone(),
            units: i.clone(),
            cost: None,
            price: None,
            flag: None,
            meta: Meta::new() 
        }
    }).collect();
    let stan_postings: Vec<Posting> = stan_taxes.iter().map(|i| {
        Posting {
            account: stan_tax_account.clone(),
            units: i.clone(),
            cost: None,
            price: None,
            flag: None,
            meta: Meta::new() 
        }
    }).collect();
    t.postings.extend(jess_postings);
    t.postings.extend(stan_postings);
}

fn display_transaction(t: &Transaction) {
    let line = match &t.payee {
        Some(payee) => format!("{} * \"{}\" \"{}\"", t.date, payee, t.narration),
        None => format!("{} * \"{}\"", t.date, t.narration)
    };
    println!("{}", line);
    &t.postings.iter().for_each(|p| {
        println!("    {} {}", p.account, p.units)
    });
    println!();
}


fn check_income_transaction(d: &Directive) -> bool {
    match d {
        Directive::Transaction(t) =>
            t.date > Date::from_str_unchecked("2020-12-31") &&
            t.postings.iter()
                .any(|p| 
                    p.account.ty == Income && 
                    (p.account.parts.contains(&Cow::Borrowed("Income-Jess")) || 
                     p.account.parts.contains(&Cow::Borrowed("Income-Stan")) )),
        _ => false,
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).ok_or("filename argument")?;
    let unparsed_file = std::fs::read_to_string(filename)?;

    let ledger = parse(&unparsed_file)?;
    let diter = ledger.directives.into_iter();
    diter.filter(|d| check_income_transaction(d))    
        .for_each(|e| 
            match e {
                Directive::Transaction(mut t) => {
                    add_taxes_transaction(&mut t);
                    display_transaction(&t);
                }
            _ => ()});
    Ok(())
}

fn main() {
    match run() {
        Err(e) => println!("Error: {}", e),
        _ => {}
    }
}
