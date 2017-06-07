// src/models/account_statement_model.rs

/// Account Statement Model

// Import Modules
// External
use ::chrono::{DateTime, Datelike};
use ::chrono::offset::utc::UTC;
// Utilities
use ::common::utilities as Utilities;
// Models
use ::models::transaction_model::{TransactionModel};

/// Account Statement
#[derive(Clone, Debug)]
pub struct AccountStatementModel {
    pub statement_date: Option<DateTime<UTC>>,
    pub ending_balance: Option<f64>
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct OutAccountStatementModel {
    pub statement_date: Option<String>,
    pub ending_balance: Option<f64>
}

// Account Statement Model Methods
impl AccountStatementModel{

    /// Get Statement Date
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<i64> Timestamp
    pub fn get_statement_date(&self) -> Option<DateTime<UTC>>{
        self.statement_date
    }

    /// Get Ending Balance
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<f64> Ending Balance
    pub fn get_ending_balance(&self) -> Option<f64>{
        self.ending_balance
    }

     /// Generate Account Statement for month, given:
    ///     1. Current Account Balance
    ///     2. List of transactions for month
    ///     3. Month and year to generate account statement
    ///
    /// # Arguments
    /// balance - f64
    /// transactions - Vec<TransactionModel>
    ///
    /// # Returns
    /// `AccountStatementModel` - Account statement
    pub fn generate_account_statement(mut starting_amount: f64, transactions: Vec<TransactionModel>) -> AccountStatementModel{

        //Calculate new account balance
        let ending_balance = AccountStatementModel::calculate_ending_balance(starting_amount, transactions);

        //Create account statement
        AccountStatementModel{
            statement_date: Some(UTC::now()),
            ending_balance: Some(ending_balance)
        }
    }
    
    /// Calculate new ending balance from previous ending balance and list of transactions
    ///
    /// # Arguments
    /// &self
    /// balance - f64
    /// transactions - Vec<TransactionModel>
    ///
    /// # Returns
    /// `f64` - ending account balance
    fn calculate_ending_balance(mut starting_amount: f64, transactions: Vec<TransactionModel>) -> f64 {
        let mut balance = Utilities::currency::Dollars::new(starting_amount);
        for transaction in &transactions {
            if let Some(t_type) = transaction.get_transaction_type(){
                if let Some(t_amount) = transaction.get_amount(){
                    match t_type.as_ref() {
                        "credit" => {
                            balance = balance + Utilities::currency::Dollars::new(t_amount);
                        },
                        "debit" => {
                            balance = balance - Utilities::currency::Dollars::new(t_amount);
                        },
                        _ => {}
                    }
                }
            }
        }
        balance.to_dollars()
    }
}

// Out Account Statement Model Methods
impl OutAccountStatementModel{

    /// Create OutAccountStatementModel from AccountStatementModel
    ///
    /// # Arguments
    /// account - AccountStatementModel
    ///
    /// # Returns
    /// 'OutAccountStatementModel'
    pub fn new(mut account: AccountStatementModel) -> OutAccountStatementModel{
        OutAccountStatementModel{
            statement_date:match account.get_statement_date(){
                Some(timestamp) => {
                    Some(timestamp.to_string())
                },
                None => None
            },
            ending_balance: account.get_ending_balance()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test AccountStatementModel::generate_account_statement
    #[test]
    pub fn generate_account_statement(){
        let mut transactions = Vec::new();
        let now = UTC::now();

        // Test 1
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(1.00),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(2.00),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        let mut account_statement = AccountStatementModel::generate_account_statement(5.00, transactions.clone());
        let mut statement_balance = account_statement.get_ending_balance().unwrap();
        let mut statement_date = account_statement.get_statement_date().unwrap();
        assert_eq!(statement_balance, 6.00);
        assert_eq!(statement_date.year(), now.year());
        assert_eq!(statement_date.month(), now.month());
        assert_eq!(statement_date.day(), now.day());

        // Test 2
        transactions.clear();
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(187.79),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(9712.27),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(78.94),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(849.15),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(7.02),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        account_statement = AccountStatementModel::generate_account_statement(-1531.25, transactions.clone());
        statement_balance = account_statement.get_ending_balance().unwrap();
        statement_date = account_statement.get_statement_date().unwrap();
        assert_eq!(statement_date.year(), now.year());
        assert_eq!(statement_date.month(), now.month());
        assert_eq!(statement_date.day(), now.day());
        assert_eq!(statement_balance, 7230.04);
    }

    /// Test AccountStatementModel::calculate_ending_balance
    #[test]
    fn calculate_ending_balance() {

        let mut transactions = Vec::new();

        // Test 1
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(1.00),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(2.00),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        assert_eq!(6.00, AccountStatementModel::calculate_ending_balance(5.00, transactions.clone()));

        // Test 2
        transactions.clear();
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(1.0),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(2.50),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(3.64),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        assert_eq!(9.29, AccountStatementModel::calculate_ending_balance(2.15, transactions.clone()));

        // Test 3
        transactions.clear();
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(150.75),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(25.31),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(5.63),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(150.00),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        assert_eq!(720.17, AccountStatementModel::calculate_ending_balance(1001.24, transactions.clone()));
    }
}