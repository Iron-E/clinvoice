mod deletable;
mod employee_adapter;
mod error;
mod initializable;
mod job_adapter;
mod location_adapter;
mod r#match;
mod organization_adapter;
mod person_adapter;
mod updatable;
pub mod retrieve;

pub use
{
	deletable::Deletable,
	employee_adapter::EmployeeAdapter,
	error::Error,
	initializable::Initializable,
	job_adapter::JobAdapter,
	location_adapter::LocationAdapter,
	r#match::Match,
	organization_adapter::OrganizationAdapter,
	person_adapter::PersonAdapter,
	updatable::Updatable,
};
