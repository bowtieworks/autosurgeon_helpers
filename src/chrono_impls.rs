// STuck again on this only working while I'm in the related module :(
use autosurgeon::{Hydrate, HydrateError, Prop, ReadDoc, Reconciler};
use chrono::TimeZone;

pub fn hydrate<D: ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: Prop<'_>,
) -> Result<chrono::DateTime<chrono::Utc>, HydrateError> {
    let inner: i64 = i64::hydrate(doc, obj, prop)?;

    let secs = inner / 1000;
    let mills = inner % 1000;
    let nanos = u32::try_from(mills).unwrap() * 1_000_000;

    let ndt = chrono::naive::NaiveDateTime::from_timestamp_opt(secs, nanos).unwrap();
    // This will panic if what is stored is not a valid 16 byte array
    let inner: chrono::DateTime<chrono::Utc> = chrono::Utc.from_utc_datetime(&ndt);
    Ok(inner)
}

pub fn reconcile<R: Reconciler>(
    dt: &chrono::DateTime<chrono::Utc>,
    mut reconciler: R,
) -> Result<(), R::Error> {
    reconciler.i64(dt.timestamp_millis())
}
