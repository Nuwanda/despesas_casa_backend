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

        println!("THIS || from: {}, to: {}", self.from, self.to);
        println!("OTHER || from: {}, to: {}", other.from, other.to);
        if self.from == other.from || self.from == other.to {
            includes_source = true;
        }
        if self.to == other.from || self.to == other.from {
            includes_destination = true;
        }

        includes_source && includes_destination
    }

    pub fn update_from_other(&self, other: &Expense) -> Expense {
        let amount = if self.from == other.from {
            self.amount + other.amount
        } else {
            self.amount + other.amount * -1
        };

        if amount > 0 {
            Expense {
                amount,
                ..self.clone()
            }
        } else {
            Expense {
                from: self.to.clone(),
                to: self.from.clone(),
                amount: amount * -1,
                ..self.clone()
            }
        }
    }
}
