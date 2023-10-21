// Stuck again on this only working while I'm in the related module :(
use autosurgeon::{hydrate::Unexpected, Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
use cidr::Ipv6Cidr;
use std::str::FromStr;

pub fn hydrate<D: ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: Prop<'_>,
) -> Result<Ipv6Cidr, HydrateError> {
    let inner = String::hydrate(doc, obj, prop)?;
    let inner = Ipv6Cidr::from_str(&inner).map_err(|e| {
        HydrateError::Unexpected(Unexpected::Other {
            expected: "an IPv6 CIDR range".into(),
            found: e.to_string(),
        })
    })?;
    Ok(inner)
}

pub fn reconcile<R: Reconciler>(a_cidr: &Ipv6Cidr, mut reconciler: R) -> Result<(), R::Error> {
    reconciler.str(a_cidr.to_string())
}

pub mod option {
    use autosurgeon::{hydrate::Unexpected, Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
    use cidr::Ipv6Cidr;
    use std::str::FromStr;

    pub fn hydrate<D: ReadDoc>(
        doc: &D,
        obj: &automerge::ObjId,
        prop: Prop<'_>,
    ) -> Result<Option<Ipv6Cidr>, HydrateError> {
        let inner = Option::<String>::hydrate(doc, obj, prop)?;
        let inner = inner
            .map(|inner| {
                Ipv6Cidr::from_str(&inner).map_err(|e| {
                    HydrateError::Unexpected(Unexpected::Other {
                        expected: "an IPv6 CIDR range or null".into(),
                        found: e.to_string(),
                    })
                })
            })
            .transpose()?;
        Ok(inner)
    }

    pub fn reconcile<R: Reconciler>(
        a_cidr: &Option<Ipv6Cidr>,
        mut reconciler: R,
    ) -> Result<(), R::Error> {
        match a_cidr {
            Some(a_cidr) => reconciler.str(a_cidr.to_string()),
            None => reconciler.none(),
        }
    }
}
