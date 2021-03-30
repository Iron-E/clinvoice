use
{
	super::BincodeLocation,
	crate::data::{BincodeOrganization, Error, Result},
	clinvoice_adapter::data::{Deletable, LocationAdapter, Match, OrganizationAdapter},
	std::{borrow::Cow, fs, io::ErrorKind},
};

impl Deletable for BincodeLocation<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		if let Err(e) = fs::remove_file(self.filepath())
		{
			// We don't care if a file is missing; we want it deleted anyway.
			if e.kind() != ErrorKind::NotFound
			{
				return Err(e.into());
			}
		}

		if cascade
		{
			BincodeLocation::retrieve(
				Match::Any, // id
				Match::Any, // name
				Match::EqualTo(Cow::Borrowed(&Some(self.location.id))), // outer id
				self.store,
			)?.into_iter().try_for_each(|l|
				BincodeLocation
				{
					location: &l,
					store: self.store,
				}.delete(true)
			)?;

			BincodeOrganization::retrieve(
				Match::Any, // id
				Match::EqualTo(Cow::Borrowed(&self.location.id)), // location
				Match::Any, // name
				self.store,
			)?.into_iter().try_for_each(|o|
				BincodeOrganization
				{
					organization: &o,
					store: self.store,
				}.delete(true)
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, Deletable, LocationAdapter},
		crate::{data::BincodeOrganization, util},
		clinvoice_adapter::data::OrganizationAdapter,
		std::time::Instant,
	};

	#[test]
	fn test_delete()
	{
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation
			{
				location: &BincodeLocation::create("Earth", store).unwrap(),
				store,
			};

			let usa = BincodeLocation
			{
				location: &earth.create_inner("USA").unwrap(),
				store,
			};

			let arizona = BincodeLocation
			{
				location: &usa.create_inner("Arizona").unwrap(),
				store,
			};

			let phoenix = BincodeLocation
			{
				location: &arizona.create_inner("Phoenix").unwrap(),
				store,
			};

			let dogood = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					arizona.location.clone(),
					"DoGood Inc",
					&store
				).unwrap(),
				store,
			};

			let start = Instant::now();

			// delete just phoenix.
			phoenix.delete(false).unwrap();

			// assert that phoenix is gone.
			assert!(!&phoenix.filepath().is_file());

			// delete the usa and everything in it.
			usa.delete(true).unwrap();

			println!("\n>>>>> BincodeLocation::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert that every location inside the USA is gone
			assert!(&earth.filepath().is_file());
			assert!(!&usa.filepath().is_file());
			assert!(!&arizona.filepath().is_file());

			// assert that `dogood`, located in arizona, is gone.
			assert!(!&dogood.filepath().is_file());
		});
	}
}
