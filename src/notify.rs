use notify_rust::{Hint, Notification};

pub fn notify(
    silent: bool,
    title: impl AsRef<str>,
    body: impl AsRef<str>,
) -> Result<(), notify_rust::error::Error> {
    if silent {
        Ok(())
    } else {
        Notification::new()
            .summary(title.as_ref())
            .body(body.as_ref())
            .id(9999)
            .hint(Hint::Custom(
                "x-canonical-private-synchronous".to_string(),
                "anything".to_string(),
            ))
            .show()
            .map(|_| ())
    }
}
