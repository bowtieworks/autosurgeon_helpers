use autosurgeon::{Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
use email_address::EmailAddress;

pub fn hydrate<D: ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: Prop<'_>,
) -> Result<EmailAddress, HydrateError> {
    let inner = String::hydrate(doc, obj, prop)?;
    inner.parse().map_err(|e| {
        HydrateError::unexpected(
            "a valid email address",
            format!("an email address which failed to parse due to: {e}"),
        )
    })
}

pub fn reconcile<R: Reconciler>(value: &EmailAddress, mut reconciler: R) -> Result<(), R::Error> {
    reconciler.str(value)
}

pub mod option {
    use autosurgeon::{Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
    use email_address::EmailAddress;

    pub fn hydrate<D: ReadDoc>(
        doc: &D,
        obj: &automerge::ObjId,
        prop: Prop<'_>,
    ) -> Result<Option<EmailAddress>, HydrateError> {
        let inner = Option::<String>::hydrate(doc, obj, prop)?;
        inner
            .map(|inner| {
                inner.parse::<EmailAddress>().map_err(|e| {
                    HydrateError::unexpected(
                        "a valid email address",
                        format!("an email address which failed to parse due to: {e}"),
                    )
                })
            })
            .transpose()
    }

    pub fn reconcile<R: Reconciler>(
        value: &Option<EmailAddress>,
        mut reconciler: R,
    ) -> Result<(), R::Error> {
        match value {
            Some(v) => reconciler.str(v),
            None => reconciler.none(),
        }
    }
}
