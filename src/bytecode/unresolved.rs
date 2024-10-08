pub trait Unresolved {
    type Resolved;
    type NeededToResolve;

    fn resolve(self, _: &Self::NeededToResolve) -> Result<Self::Resolved, String>;
    fn unresolve(resolved: Self::Resolved, _: &mut Self::NeededToResolve) -> Self;
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

    fn unresolve(resolved: Self::Resolved, needed_to_resolve: &mut Self::NeededToResolve) -> Self {
        resolved
            .into_iter()
            .map(|item| Unresolved::unresolve(item, needed_to_resolve))
            .collect()
    }
}
