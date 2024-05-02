mod models;
mod requests;
mod tasks;

macro_rules! configure_insta {
    ($suffix:expr) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix($suffix);
        let _guard = settings.bind_to_scope();
    };
}
pub(crate) use configure_insta;
