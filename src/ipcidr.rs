// STuck again on this only working while I'm in the related module :(
use autosurgeon::{hydrate::Unexpected, Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
use cidr::IpCidr;
use std::str::FromStr;

pub fn hydrate<D: ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: Prop<'_>,
) -> Result<IpCidr, HydrateError> {
    let inner = String::hydrate(doc, obj, prop)?;
    let inner = IpCidr::from_str(&inner).map_err(|e| {
        HydrateError::Unexpected(Unexpected::Other {
            expected: "an IP CIDR range".into(),
            found: e.to_string(),
        })
    })?;
    Ok(inner)
}

pub fn reconcile<R: Reconciler>(a_cidr: &IpCidr, mut reconciler: R) -> Result<(), R::Error> {
    reconciler.str(a_cidr.to_string())
}
