use crate::{helpful_stuff, wallet::Wallet};

// main transaction struct
#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<InputTx>,
    pub outputs: Vec<OutputTx>,
    pub timestamp: u64,
    pub id: String,
    pub coinbase_height: Option<u32>,
}

// input transaction struct
#[derive(Debug)]
pub struct InputTx {
    pub prev_tx_id: String,
    pub output_index: u32,
    pub pub_key: String,
    pub signature: String,
}

// output transaction struct
#[derive(Debug)]
pub struct OutputTx {
    pub recipient: String,
    pub amount: u128,
}

// output point struct (used to tell which output transaction we are using)
#[derive(Debug)]
pub struct OutPoint {
    pub tx_id: String,
    pub index: u32,
}

impl Transaction {
    pub fn new(
        sender_wallet: &Wallet,
        receiver_addr: &str,
        amount: u128,
        fee: u128,
        spendable: Vec<(OutPoint, OutputTx)>,
    ) -> Result<Self, String> {
        if amount == 0 {
            return Err("amount must be > 0".into());
        }

        if !helpful_stuff::verify_address(receiver_addr) {
            return Err("error when verifying receiver address".into());
        }

        let needed = amount.checked_add(fee).ok_or("amount overflow")?;

        let mut selected = Vec::new();
        let mut total: u128 = 0;

        for (outpoint, output) in spendable {
            total = total.checked_add(output.amount).ok_or("overflow")?;
            selected.push(outpoint);
            if total >= needed {
                break;
            }
        }

        if total < needed {
            return Err("insufficient funds".into());
        }

        let mut outputs = vec![OutputTx {
            amount,
            recipient: String::from(receiver_addr),
        }];

        let change = total - needed;

        if change > 0 {
            outputs.push(OutputTx {
                amount: change,
                recipient: sender_wallet.address.to_string(),
            });
        }

        let inputs = selected
            .into_iter()
            .map(|op| InputTx {
                prev_tx_id: op.tx_id,
                output_index: op.index,
                pub_key: sender_wallet.pub_key.clone(),
                signature: String::new(),
            })
            .collect();

        let mut transaction = Transaction {
            inputs,
            outputs,
            timestamp: helpful_stuff::get_time(),
            id: String::new(),
            coinbase_height: None,
        };

        transaction.sign(sender_wallet);
        transaction.id = transaction.get_id();
        Ok(transaction)
    }

    // this returns true if the transaction is a coinbase transaction
    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }

    // this creates a new coinbase transaction
    pub fn new_coinbase(miner_addr: &str, amount: u128, height: u32) -> Self {
        let mut tx = Transaction {
            inputs: vec![], // nothing is being spent
            outputs: vec![OutputTx {
                recipient: miner_addr.to_string(),
                amount,
            }],
            timestamp: helpful_stuff::get_time(),
            id: String::new(),
            coinbase_height: Some(height),
        };

        tx.id = tx.get_id();

        tx
    }

    // this returns a String that then gets signed by the wallet
    pub fn signing_payload(&self) -> String {
        let mut payload = String::new();

        for i in &self.inputs {
            payload += &format!("in:{}:{};", i.prev_tx_id, i.output_index);
        }

        for o in &self.outputs {
            payload += &format!("out:{}:{};", o.recipient, o.amount);
        }

        if let Some(h) = self.coinbase_height {
            payload += &format!("cb:{};", h);
        }

        payload += &format!("t:{}", self.timestamp);
        payload
    }

    // this returns the id of the transaction
    pub fn get_id(&self) -> String {
        helpful_stuff::hash(self.signing_payload().as_str())
    }

    // this signs every input transaction
    pub fn sign(&mut self, wallet: &Wallet) {
        let signature = wallet.sign(self.signing_payload().as_str());
        for input in &mut self.inputs {
            input.signature = signature.clone();
        }
    }
}
