// src/common/utilities/currency.rs

/// Utilities for currency

#[derive(Debug)]
pub struct Dollars{
	pub cents: i64
}

impl Dollars{

	/// Create a Dollars struct from an f64 (decimal) value
	///
	/// # Arguments
	/// * `amount` - the dollar amount
	///
	/// # Returns
	/// `Dollars`
	pub fn new(amount: f64) -> Dollars{
		Dollars{
			cents: (amount * 100.0).round() as i64
		}
	}

	/// Convert the Dollars value in cents (i64) to a dollar decimal representation (f64)
	///
	/// # Returns
	/// `f64` Decimal dollar representation
	pub fn to_dollars(&self) -> f64{
		self.cents as f64 / 100.0
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn new() {

		// Test 1
		let amount1:f64 = 6.34;
		let dollars1 = Dollars::new(amount1);
		assert_eq!(dollars1.cents, 634);

		let amount2:f64 = 9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.cents, 929);

		let amount2:f64 = 1546.48;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.cents, 154648);

		let amount2:f64 = -18.84;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.cents, -1884);
	}

	#[test]
	fn to_dollars() {

		// Test 1
		let amount1:f64 = 6.34;
		let dollars1 = Dollars::new(amount1);
		assert_eq!(dollars1.to_dollars(), amount1);

		let amount2:f64 = 9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);

		let amount2:f64 = 84915.48;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);

		let amount2:f64 = -9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);
	}
}