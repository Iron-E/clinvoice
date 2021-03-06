use
{
	super::LocationView,
	crate::views::RestorableSerde,
};

impl RestorableSerde for LocationView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
	}
}
