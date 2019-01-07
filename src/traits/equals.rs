pub trait DbEquals<OTHER> {}

impl<'a> DbEquals<&'a str> for String {}
impl<'a> DbEquals<String> for &'a str {}

impl<A> DbEquals<A> for A {}
