// STuck again on this only working while I'm in the related module :(
use autosurgeon::{hydrate::Unexpected, Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
use std::{net::IpAddr, str::FromStr};

pub fn hydrate<D: ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: Prop<'_>,
) -> Result<IpAddr, HydrateError> {
    let inner: String = String::hydrate(doc, obj, prop)?;
    // This will panic if what is stored is not a valid 16 byte array
    let inner: IpAddr = IpAddr::from_str(&inner).map_err(|_e| {
        HydrateError::Unexpected(Unexpected::Other {
            expected: "an IP address".into(),
            found: inner,
        })
    })?;
    Ok(inner)
}

pub fn reconcile<R: Reconciler>(a_cidr: &IpAddr, mut reconciler: R) -> Result<(), R::Error> {
    reconciler.str(a_cidr.to_string())
}
