use
{
	crate::{app::QUERY_PROMPT, DynResult, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		Store,
	},
	clinvoice_data::views::EmployeeView,
	clinvoice_query as query,
};

/// # Summary
///
/// Retrieve all [`Employee`][location]s from the specified `store`. If no
/// [`Employee`][location]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Employee`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::data::EmployeeAdapter::retrieve
/// [location]: clinvoice_data::Employee
pub fn retrieve_views<'err, E, L, O, P>(query: Option<query::Employee>, store: &Store) -> DynResult<'err, Vec<EmployeeView>> where
	E : EmployeeAdapter,
	L : LocationAdapter,
	O : OrganizationAdapter,
	P : PersonAdapter,

	<E as EmployeeAdapter>::Error : 'err +
		From<<L as LocationAdapter>::Error> +
		From<<O as OrganizationAdapter>::Error> +
		From<<P as PersonAdapter>::Error>,
	<L as LocationAdapter>::Error : 'err,
	<O as OrganizationAdapter>::Error : 'err,
	<P as PersonAdapter>::Error : 'err,
{
	let query = match query
	{
		Some(q) => q,
		_ => input::edit_default(format!("{}employees", QUERY_PROMPT))?,
	};

	let results = E::retrieve(&query, &store)?;
	results.into_iter().map(|e|
		E::into_view::<L, O, P>(e, &store)
	).filter_map(|result| match result
	{
		Ok(t) => match query.matches_view(&t)
		{
			Ok(b) if b => Some(Ok(t)),
			Err(e) => Some(Err(DataError::from(e).into())),
			_ => None,
		},
		Err(e) => Some(Err(e)),
	}).collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
}
