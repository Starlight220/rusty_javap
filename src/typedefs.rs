#[allow(non_camel_case_types)]
pub type w1 = u8;
#[allow(non_camel_case_types)]
pub type w2 = u16;
#[allow(non_camel_case_types)]
pub type w4 = u32;
#[allow(non_camel_case_types)]
pub type w8 = u64;

pub trait Unresolved {
    type Resolved;
    type NeededToResolve;

    fn resolve(self, _: &Self::NeededToResolve) -> Result<Self::Resolved, String>;
}

impl<T: Unresolved> Unresolved for Vec<T> {
    type Resolved = Vec<T::Resolved>;
    type NeededToResolve = T::NeededToResolve;

    fn resolve(self, needed_to_resolve: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        let mut resolved = vec![];
        for unresolved in self {
            resolved.push(unresolved.resolve(needed_to_resolve)?)
        }
        Ok(resolved)
    }
}

#[macro_export]
macro_rules! container {
    ($c:ident, $t:ty) => {
        #[derive(Debug)]
        pub struct $c(Vec<$t>);

        impl From<Vec<$t>> for $c {
            fn from(vec: Vec<$t>) -> Self {
                Self(vec)
            }
        }
        impl Display for $c {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{} [{}]:", stringify!($c), self.0.len())?;
                for field in &self.0 {
                    write!(f, "\t")?;
                    field.fmt(f)?;
                }
                write!(f, "")
            }
        }
    };
}
