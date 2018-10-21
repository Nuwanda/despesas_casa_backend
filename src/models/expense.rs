#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: i64,
}

impl Expense {
    pub fn is_earlier_version(&self, other: &Expense) -> bool {
        let mut includes_source: bool = false;
        let mut includes_destination: bool = false;

        if self.from == other.from || self.from == other.to {
            includes_source = true;
        }
        if self.to == other.from || self.to == other.to {
            includes_destination = true;
        }

        includes_source && includes_destination
    }

    pub fn update_from_other(&mut self, other: &Expense) {
        let amount = if self.from == other.from {
            self.amount + other.amount
        } else {
            self.amount + other.amount * -1
        };

        if amount > 0 {
            self.amount = amount;
        } else {
            let temp = self.from.clone();
            self.from = self.to.clone();
            self.to = temp;
            self.amount = amount * -1;
        }
    }
}
