// use autoswappr_backend::{Db};
use sqlx::{PgPool};

#[derive(Debug, Clone)]
pub struct TransactionLog {
    wallet_address: String, 
    from_token: String, 
    to_token: String, 
    percentage: u8, 
    amount_from: u64, 
    amount_to: u64
}

impl TransactionLog{
    fn new (
        wallet_address: &str, 
        from_token: &str,
        to_token: &str,
        percentage: u8, 
        amount_from: u64, 
        amount_to: u64
    ) -> Self {
        Self {
            wallet_address: wallet_address.to_string(), 
            from_token: from_token.to_string(),
            to_token: to_token.to_string(),
            percentage, 
            amount_from, 
            amount_to
        }
    }

    pub fn validate (
        &mut self
    ) -> Result<(), String>{
        self.validate_address(&self.wallet_address)?;
        self.validate_percentage(self.percentage)?;
        self.validate_amount(self.amount_to)?;
        self.validate_amount(self.amount_from)?;
        Ok(())
    }
    fn validate_address (&self, address: &str) -> Result<(), String> {
        if address.starts_with("0x") && 
        address.len() == 42 && 
        address[2..].chars().all(|c| c.is_ascii_hexdigit()){
            println!("length: {:?}", address.len());
            return Ok(());
        }
        Err("Invalid Address".to_owned())
    }
    fn validate_percentage (&self, percentage: u8) -> Result<(), String> {
        if percentage < 0 {
            return Err(String::from("Invalid percentage"));
        }else if percentage > 100 {
            return Err(String::from("Invalid percentage"));
        }
        Ok(())
    }
    fn validate_amount (&self, amount: u64) -> Result<(), String> {
        Ok(())
    }
    async fn save (
        &mut self,
        db: &PgPool 
    ) -> Result <(), String>{
        self.validate().map_err(|_| "Transaction log is invalid").unwrap();
        sqlx::query(
            r#"INSERT INTO transactions_log (wallet_address, from_token, to_token, percentage, amount_from, amount_to)
        VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(self.wallet_address)
        .bind(self.from_token)
        .bind(self.to_token)
        .bind(self.percentage)
        .bind(self.amount_from)
        .bind(self.amount_to)
        .execute(&db)
        .await.map_err(|_|"Error saving transaction log")?;
        Ok(())
    }
}

pub async fn log_transaction(
    wallet_address: &str,
    from_token: &str,
    to_token: &str,
    percentage: u8,
    amount_from: u64,
    amount_to: u64,
    db: &PgPool
) -> Result<TransactionLog, String>
 {
    let mut tx = TransactionLog::new(wallet_address, from_token, to_token, percentage, amount_from, amount_to);
    tx.save(&db).await?;
    Ok(tx)
}
