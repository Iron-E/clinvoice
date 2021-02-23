use
{
	super::LocationView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for LocationView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		let mut output = self.name.clone();
		let mut outer = &self.outer;

		while let Some(o) = outer
		{
			output.push_str(", ");
			output.push_str(&o.name);

			outer = &o.outer;
		}

		return write!(formatter, "{}", output);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::LocationView,
		crate::Id,
		std::time::Instant,
	};

	/// # Summary
	///
	/// The main method.
	#[test]
	fn test_display()
	{
		let earth_view = LocationView
		{
			name: "Earth".into(),
			id: Id::new_v4(),
			outer: None,
		};

		let usa_view = LocationView
		{
			name: "USA".into(),
			id: Id::new_v4(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView
		{
			name: "Arizona".into(),
			id: Id::new_v4(),
			outer: Some(usa_view.into())
		};

		let phoenix_view = LocationView
		{
			name: "Phoenix".into(),
			id: Id::new_v4(),
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView
		{
			name: "1337 Some Street".into(),
			id: Id::new_v4(),
			outer: Some(phoenix_view.into()),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", street_view), "1337 Some Street, Phoenix, Arizona, USA, Earth");
		println!("\n>>>>> LocationView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
