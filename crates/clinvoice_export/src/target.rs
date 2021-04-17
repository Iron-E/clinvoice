mod error;
mod from_str;

pub use error::{Error, Result};

use
{
	core::fmt::Write,

	crate::markdown,

	clinvoice_data::
	{
		chrono::{DateTime, Local},
		Job,
		views::{JobView, TimesheetView},
	},
};

/// # Summary
///
/// A target for exporting.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target
{
	/// # Summary
	///
	/// The markdown target. Exports to a `.md` file.
	#[cfg(feature="markdown")]
	Markdown,
}

impl Target
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	///
	/// # TODO
	///
	/// Show better employee information.
	fn export_timesheet(&self, output: &mut String, timesheet: &TimesheetView)
	{
		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{

				writeln!(output, "{}", markdown::Element::Heading
				{
					depth: 3,
					text: format!("{} – {}",
						DateTime::<Local>::from(timesheet.time_begin),
						timesheet.time_end.map(|time| DateTime::<Local>::from(time).to_string()).unwrap_or_else(|| "Current".into())
					),
				}).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Employee Information"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&timesheet.employee)).unwrap();

				if !timesheet.expenses.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Expenses"}).unwrap();

					timesheet.expenses.iter().try_for_each(|e| writeln!(output, "{}{}",
						markdown::Element::Heading {depth: 5, text: format!("{} – {}", e.category, e.cost)},
						markdown::Element::BlockText(&e.description),
					)).unwrap();
				}

				writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Work Notes"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&timesheet.work_notes)).unwrap();
			},
		};
	}

	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	pub fn export_job(&self, job: JobView) -> String
	{
		let mut output = String::new();

		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{
				writeln!(output, "{}", markdown::Element::Heading {depth: 1, text: format!("Job #{} for {}", job.id, job.client)}).unwrap();

				writeln!(output, "{} {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Opened:")},
					DateTime::<Local>::from(job.date_open),
				).unwrap();

				if let Some(date) = job.date_close
				{
					writeln!(output, "{} {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Closed:")},
						DateTime::<Local>::from(date),
					).unwrap();
				}

				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Invoice"}).unwrap();
				writeln!(output, "{} {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Hourly Rate:")},
					job.invoice.hourly_rate,
				).unwrap();

				if let Some(date) = &job.invoice.date
				{
					writeln!(output, "{} {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Status:")},
						date,
					).unwrap();
				}

				writeln!(output, "{} {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Total Amount Owed:")},
					Job::from(&job).total(),
				).unwrap();
				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Objectives"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.objectives)).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Notes"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.notes)).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Timesheets"}).unwrap();
				job.timesheets.iter().for_each(|t| self.export_timesheet(&mut output, t));
			},
		};

		output
	}
}

#[cfg(all(feature="markdown", test))]
mod tests
{
	use
	{
		std::collections::HashMap,

		super::{JobView, Target, TimesheetView},

		clinvoice_data::
		{
			chrono::Utc,
			Decimal, EmployeeStatus, Expense, ExpenseCategory, Id, Invoice, Money,
			views::{EmployeeView, LocationView, OrganizationView, PersonView},
		},
	};

	#[test]
	fn export_job()
	{
		let organization = OrganizationView
		{
			id: Id::new_v4(),
			location: LocationView
			{
				id: Id::new_v4(),
				outer: Some(LocationView
				{
					id: Id::new_v4(),
					outer: Some(LocationView
					{
						id: Id::new_v4(),
						outer: Some(LocationView
						{
							id: Id::new_v4(),
							outer: Some(LocationView
							{
								id: Id::new_v4(),
								outer: None,
								name: "Earth".into(),
							}.into()),
							name: "USA".into(),
						}.into()),
						name: "Arizona".into(),
					}.into()),
					name: "Phoenix".into(),
				}.into()),
				name: "1337 Some Street".into(),
			},
			name: "Big Old Test".into(),
		};

		let testy_mctesterson = EmployeeView
		{
			contact_info: HashMap::new(),
			id: Id::new_v4(),
			organization: organization.clone(),
			person: PersonView
			{
				id: Id::new_v4(),
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		let job = JobView
		{
			client: organization,
			date_close: None,
			date_open: Utc::today().and_hms(0, 0, 0),
			id: Id::new_v4(),
			invoice: Invoice
			{
				date: None,
				hourly_rate: Money::new(Decimal::new(2000, 2), "USD"),
			},
			notes: "* I tested the function.".into(),
			objectives: "* I want to test this function.".into(),
			timesheets: vec![
				TimesheetView
				{
					employee: testy_mctesterson,
					expenses: Vec::new(),
					time_begin: Utc::today().and_hms(2, 0, 0),
					time_end: Some(Utc::today().and_hms(2, 3, 0)),
					work_notes: "* Wrote the test.".into(),
				},
			],
		};

		assert_eq!(
			Target::Markdown.export_job(job),
"",
		);
	}
}
