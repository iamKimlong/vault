//! UI Components
//!
//! Reusable TUI widgets for the credential manager.

pub mod detail;
pub mod form;
pub mod list;
pub mod statusline;
pub mod dialogs;
pub mod help;
pub mod input_field;
pub mod layout;
pub mod logs;
pub mod scroll;
pub mod tags;
pub mod export_dialog;

// Re-exports
pub use detail::{CredentialDetail, DetailView};
pub use form::{CredentialForm, CredentialFormWidget};
pub use list::{CredentialItem, CredentialList, EmptyState, ListViewState};
pub use statusline::{HelpBar, MessageType, StatusLine};
pub use dialogs::{ConfirmDialog, PasswordDialog};
pub use help::{HelpScreen};
pub use export_dialog::ExportDialog;
