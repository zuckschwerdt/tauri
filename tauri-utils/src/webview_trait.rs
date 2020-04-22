use raw_window_handle::RawWindowHandle;

/// Trait for abstracting web view implementations.
///
/// # Implementation Contract
/// 1) [`WebViewBinding::init`] must use [`compiler_error`] if it does not support the current
/// target
/// 2) The binding must not assume that it is the only instance of itself.
pub trait WebViewBinding {
    /// The constructor for the binding.
    fn init(window_handle: RawWindowHandle) -> Self where Self: Sized;

    /// Sends JavaScript to the backing engine for evaluation.
    fn eval(&mut self, js: &str) -> crate::Result<()>;
    /// Injects CSS, overwriting, into the backing engine.
    fn inject_css(&mut self, css: &str) -> crate::Result<()>;
    /// Loads the given HTML, overwriting the currently loaded page.
    fn load_html(&mut self, html: &str) -> crate::Result<()>;
}

/// Contains the bindings used for a given instance of a web view.
pub struct WebView(Box<dyn WebViewBinding>);

impl WebView {
    /// Builds the generic web view layer, calling [`WebViewBinding::init`] internally.
    pub fn new<T: WebViewBinding + 'static>(window_handle: RawWindowHandle) -> crate::Result<Self> {
        Ok(Self(Box::new(T::init(window_handle))))
    }

    /// Sends JavaScript to the backing engine for evaluation.
    pub fn eval(&mut self, js: &str) -> crate::Result<()> {
        self.0.eval(js)
    }

    /// Injects CSS, overwriting, into the backing engine.
    pub fn inject_css(&mut self, css: &str) -> crate::Result<()> {
        self.0.inject_css(css)
    }

    /// Loads the given HTML, overwriting the currently loaded page.
    pub fn load_html(&mut self, html: &str) -> crate::Result<()> {
        self.0.load_html(html)
    }
}
