use crate::wallet::Wallet;
use crate::{helpful_stuff, wallet};

// output point struct
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct OutPoint {
    pub tx_id: String,
    pub index: u32,
}

// transaction input struct
#[derive(Clone, Debug)]
pub struct TxInput {
    pub prev_tx_id: String,
    pub output_index: u32,
    pub pub_key: String,
    pub signature: String,
}

// transaction output struct
#[derive(Clone, Debug)]
pub struct TxOutput {
    pub amount: u128,
    pub recipient: String,
}

// transaction struct
#[derive(Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: u64,
}

impl Transaction {
    // use this when creating a new transaction
    pub fn new(
        sender_wallet: &Wallet,
        receiver_addr: &str,
        amount: u128,
        spendable: Vec<(OutPoint, TxOutput)>,
    ) -> Result<Self, String> {
        if amount == 0 {
            return Err("amount must be > 0".into());
        }

        if !helpful_stuff::verify_address(receiver_addr) {
            return Err("error when verifying receiver address".into());
        }

        let mut selected = Vec::new();
        let mut total: u128 = 0;
        for (outpoint, output) in spendable {
            total = total.checked_add(output.amount).ok_or("overflow")?;
            selected.push(outpoint);
            if total >= amount {
                break;
            }
        }
        if total < amount {
            return Err("Insufficient funds".into());
        }

        let mut outputs = vec![TxOutput {
            amount,
            recipient: String::from(receiver_addr),
        }];
        let change = total - amount;
        if change > 0 {
            outputs.push(TxOutput {
                amount: change,
                recipient: sender_wallet.address.clone(),
            });
        }

        let inputs = selected
            .into_iter()
            .map(|op| TxInput {
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

    // this returns a String that then gets signed by the wallet priv_key
    pub fn signing_payload(&self) -> String {
        let mut s = String::new();
        for i in &self.inputs {
            s += &format!("in:{}:{};", i.prev_tx_id, i.output_index);
        }
        for o in &self.outputs {
            s += &format!("out:{}:{};", o.recipient, o.amount);
        }
        s += &format!("t:{}", self.timestamp);
        s
    }

    pub fn id(&self) -> String {
        helpful_stuff::hash(&self.signing_payload())
    }

    // this signs all the data that needs to be signed in the transaction
    pub fn sign(&mut self, wallet: &Wallet) {
        let signature = wallet.get_signature(self.signing_payload().as_str());
        for input in &mut self.inputs {
            input.signature = signature.clone();
        }
    }
}
