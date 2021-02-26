use
{
	crate::data::{BincodeJob, BincodeOrganization},
	clinvoice_adapter::
	{
		data::{Error as DataError, MatchWhen, OrganizationAdapter},
		DynamicResult,
	},
	clinvoice_data::Organization,
};

impl Into<DynamicResult<Organization>> for BincodeJob<'_, '_, '_>
{
	fn into(self) -> DynamicResult<Organization>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.job.client_id), // id
			MatchWhen::Any, // location
			MatchWhen::Any, // name
			self.store,
		)?;

		let bincode_organization = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			_ => return Err(DataError::DataIntegrity {id: self.job.client_id}.into()),
		};

		Ok(bincode_organization.organization.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, BincodeOrganization, OrganizationAdapter},
		crate::util,
		clinvoice_adapter::{DynamicResult, data::JobAdapter},
		clinvoice_data::{chrono::Utc, Decimal, Id, Location, Money, Organization},
		std::time::Instant,
	};

	#[test]
	fn test_into_organization()
	{
		util::test_temp_store(|store|
		{
			let dogood = BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"DoGood Inc",
				*store
			).unwrap();

			let test_job = BincodeJob::create(
				dogood.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), ""),
				"Test the job creation function.",
				*store,
			).unwrap();

			let start = Instant::now();
			let test_org: DynamicResult<Organization> = test_job.into();
			println!("\n>>>>> BincodeJob::into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(dogood.organization, test_org.unwrap());
		});
	}
}
