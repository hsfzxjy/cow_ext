use std::borrow::{Borrow, Cow};

pub trait CowExt<'a, T: ?Sized + ToOwned>: Sized {
    /// Clones as a `Cow`` with shorter lifetime.
    ///
    /// `clone_s` performs "shallow clone", by casting the borrowed variant to a shorter lifetime,
    /// or referencing the owned variant. It never clones the owned data, and thus is cheaper than
    /// `Clone::clone`.
    fn clone_s<'b: 'a>(&'b self) -> Cow<'b, T>;
    /// Clones as a `Cow`` with longer lifetime.
    ///
    /// `clone_l` always clones the data, whether it is borrowed or owned.
    fn clone_l<'b>(&self) -> Cow<'b, T>
    where
        'a: 'b;
    /// Converts into a `Cow` with static lifetime.
    ///
    /// `into_cow_static` clones the data if it is borrowed, and returns the owned data otherwise.
    fn into_cow_static(self) -> Cow<'static, T>;
}

impl<'a, T: ?Sized + ToOwned> CowExt<'a, T> for Cow<'a, T> {
    fn clone_s<'b: 'a>(&'b self) -> Cow<'b, T> {
        Cow::Borrowed(&**self)
    }
    fn clone_l<'b>(&self) -> Cow<'b, T>
    where
        'a: 'b,
    {
        match self {
            Cow::Borrowed(b) => Cow::Owned((*b).to_owned()),
            Cow::Owned(o) => Cow::Owned(o.borrow().to_owned()),
        }
    }
    fn into_cow_static(self) -> Cow<'static, T> {
        match self {
            Cow::Borrowed(b) => Cow::Owned(b.to_owned()),
            Cow::Owned(o) => Cow::Owned(o),
        }
    }
}
