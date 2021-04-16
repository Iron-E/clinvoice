use
{
	core::fmt::{Display, Formatter, Result},

	super::InvoiceDate,

	chrono::{DateTime, Local},
};

impl Display for InvoiceDate
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "Issued on {}; ", DateTime::<Local>::from(self.issued))?;

		if let Some(date) = self.paid
		{
			return write!(formatter, "Paid on {}", DateTime::<Local>::from(date));
		}

		write!(formatter, "Outstanding")
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::InvoiceDate,
		crate::chrono::Utc,
	};

	#[test]
	fn display()
	{
		let date = InvoiceDate
		{
			issued: Utc::now(),
			paid: None,
		};

		let other_date = InvoiceDate
		{
			issued: Utc::now(),
			paid: Some(Utc::now()),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", date), format!("Issued on {}; Outstanding", date.issued));
		assert_eq!(format!("{}", other_date), format!("Issued on {}; Paid on {}", other_date.issued, other_date.paid.unwrap()));
		println!("\n>>>>> InvoiceDate::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);
	}
}
