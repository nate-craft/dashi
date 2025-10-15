use notify_rust::Notification;

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
            .show()
            .map(|_| ())
    }
}
