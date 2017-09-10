// src/models/transaction_model.rs

/// Transaction Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::chrono::{DateTime, Duration, Local, TimeZone};
use ::chrono::Utc as UTC;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Transaction
#[derive(Clone, Debug)]
pub struct TransactionModel {
    pub id: Option<ObjectId>,
    pub money_map_id: Option<ObjectId>,
    pub account_id: Option<ObjectId>,
    pub datetime: Option<DateTime<UTC>>,
    pub payee: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<ObjectId>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub status: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PubTransactionModel {
    pub id: Option<String>,
    pub money_map_id: Option<String>,
    pub account_id: Option<String>,
    pub datetime: Option<String>,
    pub payee: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub status: Option<String>
}

// Transaction Model Methods
impl TransactionModel{

    /// Create TransactionModel from PubTransactionModel
    ///
    /// # Arguments
    /// `account` - PubTransactionModel
    ///
    /// # Returns
    /// `TransactionModel`
    pub fn new(transaction: &PubTransactionModel) -> TransactionModel{
        TransactionModel{
            id: match transaction.get_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            money_map_id: match transaction.get_money_map_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            account_id: match transaction.get_account_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            datetime: None,
            payee: transaction.get_payee(),
            description: transaction.get_description(),
            category_id: match transaction.get_category_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            amount: transaction.get_amount(),
            transaction_type: transaction.get_transaction_type(),
            status: transaction.get_status()
        }
    }

    /// Get ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<ObjectId>` - ID
    pub fn get_id(&self) -> Option<ObjectId>{
        self.id.clone()
    }

    /// Set ID
    ///
    /// # Arguments
    /// `self`
    pub fn set_id(&mut self, id: ObjectId) {
        self.id = Some(id);
    }

    /// Get Money Map ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<ObjectId>` - Money Map ID
    pub fn get_money_map_id(&self) -> Option<ObjectId>{
        self.money_map_id.clone()
    }

    /// Set Money Map ID
    ///
    /// # Arguments
    /// `self`
    /// `mm_id` Money Map ID
    pub fn set_money_map_id(&mut self, mm_id: Option<ObjectId>){
        self.money_map_id = mm_id
    }

    /// Get Account ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<ObjectId>` - Account ID
    pub fn get_account_id(&self) -> Option<ObjectId>{
        self.account_id.clone()
    }

    /// Set Account ID
    ///
    /// # Arguments
    /// `self`
    /// `acc_id` Account ID
    pub fn set_account_id(&mut self, acc_id: Option<ObjectId>){
        self.account_id = acc_id
    }

    /// Get Datetime
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<DateTime<UTC>>` - Datetime
    pub fn get_datetime(&self) -> Option<DateTime<UTC>>{
        self.datetime.clone()
    }

    /// Set Datetime
    ///
    /// # Arguments
    /// `self`
    /// `datetime`
    pub fn set_datetime(&mut self, datetime: Option<DateTime<UTC>>){
        self.datetime = datetime;
    }

    /// Get Payee
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Entity
    pub fn get_payee(&self) -> Option<String>{
        self.payee.clone()
    }

    /// Get Description
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - description
    pub fn get_description(&self) -> Option<String>{
        self.description.clone()
    }

    /// Get Category ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<ObjectId>` - Category ID
    pub fn get_category_id(&self) -> Option<ObjectId>{
        self.category_id.clone()
    }

    /// Get Amount
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<f64>` - Amount
    pub fn get_amount(&self) -> Option<f64>{
        self.amount.clone()
    }

    /// Get Transaction Type
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Transaction Type
    pub fn get_transaction_type(&self) -> Option<String>{
        self.transaction_type.clone()
    }

    /// Get Status
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Status
    pub fn get_status(&self) -> Option<String>{
        self.status.clone()
    }

    /// Set Status
    ///
    /// # Arguments
    /// `self`
    /// `status` The Status
    pub fn set_status(&mut self, status: Option<String>){
        self.status = status;
    }

    /// Validate Transaction
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `ValidationResult` - validation result
    pub fn validate(&self) -> ValidationResult{
        // Valid Options
        let transaction_types = vec!["credit".to_string(), "debit".to_string()];

        //validate Transaction
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.payee.clone()){
            validation_result.add_error("payee".to_string(), "Payee is required.".to_string());
        }
        if !Validators::not_empty(self.amount.clone()){
            validation_result.add_error("amount".to_string(), "Amount is required.".to_string());
        }else{
            // Validate amount
            let amount = self.amount.unwrap();
            if amount < 0.0 {
                validation_result.add_error("amount".to_string(), "Amount must be non-negative.".to_string());
            }else{
                // Validate number is valid currency amount
                let amount_s = amount.to_string();

                if !Validators::matches(&amount_s, r"^(\d+)(\.\d(\d)?)?$"){
                    validation_result.add_error("amount".to_string(), "Amount is not valid.".to_string());
                }
            }
        }
        if !Validators::not_empty_string(self.transaction_type.clone()){
            validation_result.add_error("transaction_type".to_string(), "Transaction Type is required.".to_string());
        }else{
            // Validate transaction_type
            if !transaction_types.contains(&self.transaction_type.clone().unwrap()){
                validation_result.add_error("transaction_type".to_string(), "Transaction Type is not valid.".to_string());
            }
        }

        validation_result
    }//end validate

    /// Validate Existing Transaction
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `ValidationResult` - validation result
    pub fn validate_existing(&self) -> ValidationResult{
        // Valid Options
        let transaction_types = vec!["credit".to_string(), "debit".to_string()];

        //validate Transaction
        let mut validation_result = ValidationResult::new();
        if Validators::empty(&self.id){
            validation_result.add_error("id".to_string(), "Transaction ID is required.".to_string());
        }
        if !Validators::empty(&self.payee){
            if !Validators::not_empty_string(self.payee.clone()){
                validation_result.add_error("payee".to_string(), "Payee is required.".to_string());
            }
        }
        if Validators::not_empty(self.amount.clone()){
            // Validate amount
            let amount = self.amount.unwrap();
            if amount < 0.0 {
                validation_result.add_error("amount".to_string(), "Amount must be non-negative.".to_string());
            }else{
                // Validate number is valid currency amount
                let amount_s = amount.to_string();

                if !Validators::matches(&amount_s, r"^(\d+)(\.\d(\d)?)?$"){
                    validation_result.add_error("amount".to_string(), "Amount is not valid.".to_string());
                }
            }
        }
        if !Validators::empty(&self.transaction_type){
            if !Validators::not_empty_string(self.transaction_type.clone()){
                validation_result.add_error("transaction_type".to_string(), "Transaction Type is required.".to_string());
            }else{
                // Validate transaction_type
                if !transaction_types.contains(&self.transaction_type.clone().unwrap()){
                    validation_result.add_error("transaction_type".to_string(), "Transaction Type is not valid.".to_string());
                }
            }
        }
        if !Validators::empty(&self.money_map_id){
            validation_result.add_error("money_map_id".to_string(), "Money Map ID cannot be modified.".to_string());
        }
        if !Validators::empty(&self.account_id){
            validation_result.add_error("account_id".to_string(), "Account ID cannot be modified.".to_string());
        }

        validation_result
    }//end validate_existing
}


// Pub Transaction Model Methods
impl PubTransactionModel{

    /// Create PubTransactionModel from TransactionModel
    ///
    /// # Arguments
    /// `account` - TransactionModel
    ///
    /// # Returns
    /// 'PubTransactionModel'
    pub fn new(transaction: TransactionModel) -> PubTransactionModel{
        PubTransactionModel{
            id: match transaction.get_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            money_map_id: match transaction.get_money_map_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            account_id: match transaction.get_account_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            datetime: match transaction.get_datetime(){
                Some(timestamp) => {
                    Some(timestamp.to_string())
                },
                None => None
            },
            payee: transaction.get_payee(),
            description: transaction.get_description(),
            category_id: match transaction.get_category_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            amount: transaction.get_amount(),
            transaction_type: transaction.get_transaction_type(),
            status: transaction.get_status()
        }
    }

    /// Get ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// 'Option<String>' - ID
    pub fn get_id(&self) -> Option<String>{
        self.id.clone()
    }

    /// Get Money Map ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Money Map ID
    pub fn get_money_map_id(&self) -> Option<String>{
        self.money_map_id.clone()
    }

    /// Set Money Map ID
    ///
    /// # Arguments
    /// `self`
    /// `mm_id` Money Map ID
    pub fn set_money_map_id(&mut self, mm_id: Option<String>){
        self.money_map_id = mm_id
    }

    /// Get Account ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Account ID
    pub fn get_account_id(&self) -> Option<String>{
        self.account_id.clone()
    }

    /// Set Account ID
    ///
    /// # Arguments
    /// `self`
    /// `acc_id` Account ID
    pub fn set_account_id(&mut self, acc_id: Option<String>){
        self.account_id = acc_id
    }

    /// Get Datetime
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Datetime
    pub fn get_datetime(&self) -> Option<String>{
        self.datetime.clone()
    }

    /// Get Payee
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Entity
    pub fn get_payee(&self) -> Option<String>{
        self.payee.clone()
    }

    /// Get Description
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - description
    pub fn get_description(&self) -> Option<String>{
        self.description.clone()
    }

    /// Get Category ID
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Category ID
    pub fn get_category_id(&self) -> Option<String>{
        self.category_id.clone()
    }

    /// Get Amount
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<f64>` - Amount
    pub fn get_amount(&self) -> Option<f64>{
        self.amount.clone()
    }

    /// Get Transaction Type
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Transaction Type
    pub fn get_transaction_type(&self) -> Option<String>{
        self.transaction_type.clone()
    }

    /// Get Status
    ///
    /// # Arguments
    /// `self`
    ///
    /// # Returns
    /// `Option<String>` - Status
    pub fn get_status(&self) -> Option<String>{
        self.status.clone()
    }
}
