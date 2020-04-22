use raw_window_handle::RawWindowHandle;

pub trait WebViewBinding {
    /// When `Some`, adds a build restriction so the binding can only be used with the given
    /// `target_os` condition.
    const TARGET_OS: Option<&'static str>;
    /// When `Some`, adds a build restriction so the binding can only be used with the given
    /// `target_family` condition.
    const TARGET_FAMILY: Option<&'static str>;

    /// The constructor for the binding.
    fn init(window_handle: RawWindowHandle) -> crate::Result<Self>;

    /// Sends JavaScript to the backing engine for evaluation.
    fn eval(&mut self, js: &str) -> crate::Result<()>;
    /// Injects CSS, overwriting, into the backing engine.
    fn inject_css(&mut self, css: &str) -> crate::Result<()>;
    /// Loads the given HTML, overwriting the currently loaded page.
    fn load_html(&mut self, html: &str) -> crate::Result<()>;

    /// Helper method to access the associated constant.
    fn target_os() -> Option<&'static str> {
        Self::TARGET_OS
    }
    /// Helper method to access the associated constant.
    fn target_family() -> Option<&'static str> {
        Self::TARGET_FAMILY
    }
}

/// Contains the bindings used for a given instance of a web view.
pub struct WebView(Box<dyn WebViewBinding>);

impl WebView {
    /// Builds the generic web view layer, calling [`WebViewBinding::init`] internally.
    pub fn new<T: WebViewBinding>(window_handle: RawWindowHandle) -> crate::Result<Self> {
        Ok(Self(Box::new(T::init(window_handle)?)))
    }

    /// Performs a runtime check to ensure the provided bindings are valid for the current build.
    ///
    /// # Panics
    /// If the Target OS or Target Family do not match, this will panic.
    pub fn validate_target(&self) {
        if let Some(os) = self.0.target_os() {
            if !cfg!(target_os = os) {
                panic!("Invalid target. Attempted to use binding valid only for `target_os = {}`", os);
            }
        }

        if let Some(family) = self.0.target_family() {
            if !cfg!(target_family = family) {
                panic!("Invalid target. Attempted to use binding valid only for `target_family = {}`", family);
            }
        }
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
