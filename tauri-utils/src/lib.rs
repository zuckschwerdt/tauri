pub mod platform;
pub mod process;
mod webview_trait;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
    errors {}
}
