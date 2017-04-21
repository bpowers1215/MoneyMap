// src/common/utilities/currency.rs

/// Utilities for currency

// Import
// External
use std::ops::Add;
use std::cmp::PartialEq;

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

impl Add for Dollars {
	type Output = Dollars;

	/// Add two dollar amounts. The method for the + operator.
	///
	/// # Arguments
	/// * `other` - second dollar amount
	///
	/// # Returns
	/// `Dollars`
	fn add(self, other: Dollars) -> Dollars {
		Dollars {
			cents: self.cents + other.cents
		}
	}
}

impl PartialEq for Dollars {

	/// Compare two dollar amountss. The method for the == operator.
	///
	/// # Arguments
	/// * `other` - second dollar amount
	///
	/// # Returns
	/// `Dollars`
	fn eq(&self, other: &Dollars) -> bool {
		self.cents == other.cents
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

		// Test 2
		let amount2:f64 = 9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.cents, 929);

		// Test 3
		let amount2:f64 = 1546.48;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.cents, 154648);

		// Test 4
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

		// Test 2
		let amount2:f64 = 9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);

		// Test 3
		let amount2:f64 = 84915.48;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);

		// Test 4
		let amount2:f64 = -9.29;
		let dollars2 = Dollars::new(amount2);
		assert_eq!(dollars2.to_dollars(), amount2);
	}

	#[test]
	fn eq() {

		assert_eq!(Dollars::new(14.48) == Dollars::new(14.48), true);
		assert_eq!(Dollars::new(1584.94) == Dollars::new(1584.94), true);
		assert_eq!(Dollars::new(-81.58) == Dollars::new(-81.58), true);
		assert_eq!(Dollars::new(87468.84) == Dollars::new(87468.84), true);

		assert_eq!(Dollars::new(14.48) == Dollars::new(84.41), false);
		assert_eq!(Dollars::new(1584.94) == Dollars::new(1584.93), false);
		assert_eq!(Dollars::new(-81.58) == Dollars::new(81.58), false);
		assert_eq!(Dollars::new(781.48) == Dollars::new(87468.84), false);
	}

	#[test]
	fn add() {

		// Test 1
		let dollars1 = Dollars::new(5.54);
		let dollars2 = Dollars::new(8.94);
		assert_eq!(dollars1 + dollars2, Dollars::new(14.48));
	}
}