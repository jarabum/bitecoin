use crate::{helpful_stuff, wallet::Wallet};

// main transaction struct
#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<InputTx>,
    pub outputs: Vec<OutputTx>,
    pub timestamp: u64,
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
        };

        transaction.sign(sender_wallet);
        Ok(transaction)
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

        payload += &format!("t:{}", self.timestamp);
        payload
    }

    // this signs every input transaction
    pub fn sign(&mut self, wallet: &Wallet) {
        let signature = wallet.get_signature(&self.signing_payload().as_str());
        for input in &mut self.inputs {
            input.signature = signature.clone();
        }
    }
}
