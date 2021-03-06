use
{
	std::borrow::Cow::Borrowed,

	super::{EmployeeAdapter, Error},
	crate::Store,

	clinvoice_data::{Employee, Timesheet},
	clinvoice_query as query,
};

/// # Summary
///
/// Convert some `timesheet` into its referenced [`Employee`].
pub fn to_employee<E>(timesheet: &Timesheet, store: &Store)
	-> Result<Employee, <E as EmployeeAdapter>::Error>
where
	E : EmployeeAdapter,
{
	E::retrieve(
		&query::Employee
		{
			id: query::Match::EqualTo(Borrowed(&timesheet.employee_id)),
			..Default::default()
		},
		store,
	)?.into_iter().next().ok_or_else(|| Error::DataIntegrity(timesheet.employee_id).into())
}
