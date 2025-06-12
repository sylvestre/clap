use std::collections::HashMap;

#[cfg(feature = "i18n")]
use fluent::{FluentArgs, FluentBundle, FluentResource};
#[cfg(feature = "i18n")]
use std::fs;
#[cfg(feature = "i18n")]
use std::path::{Path, PathBuf};
#[cfg(feature = "i18n")]
use std::str::FromStr;
#[cfg(feature = "i18n")]
use std::sync::OnceLock;
#[cfg(feature = "i18n")]
use unic_langid::LanguageIdentifier;

use std::error::Error;

 #[cfg(feature = "i18n")]
use std::fmt;

#[cfg(feature = "i18n")]
#[derive(Debug)]
pub enum LocalizationError {
    Io {
        source: std::io::Error,
        path: PathBuf,
    },
    Parse(String),
    Bundle(String),
    LocalesDirNotFound(String),
    PathResolution(String),
}

#[cfg(feature = "i18n")]
impl fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalizationError::Io { source, path } => {
                write!(f, "I/O error loading '{}': {}", path.display(), source)
            }
            LocalizationError::Parse(msg) => write!(f, "Parse error: {}", msg),
            LocalizationError::Bundle(msg) => write!(f, "Bundle error: {}", msg),
            LocalizationError::LocalesDirNotFound(path) => {
                write!(f, "Locales directory not found: {}", path)
            }
            LocalizationError::PathResolution(msg) => {
                write!(f, "Path resolution error: {}", msg)
            }
        }
    }
}

#[cfg(feature = "i18n")]
impl std::error::Error for LocalizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LocalizationError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[cfg(feature = "i18n")]
impl From<std::io::Error> for LocalizationError {
    fn from(error: std::io::Error) -> Self {
        LocalizationError::Io {
            source: error,
            path: PathBuf::from("<unknown>"),
        }
    }
}

pub const DEFAULT_LOCALE: &str = "en-US";

/// Retrieves a localized message by its identifier with English fallback.
///
/// When the `i18n` feature is disabled (default), always returns the provided
/// English string for zero-cost localization. When `i18n` is enabled, looks up
/// the message ID in the user's locale and falls back to the English string.
///
/// # Arguments
///
/// * `id` - The message identifier for localization lookup
/// * `english` - The English text to use as fallback and when i18n is disabled
///
/// # Returns
///
/// A `String` containing the localized message or English fallback
///
/// # Examples
///
/// ```
/// use uucore::locale::get_message;
///
/// // Fast path - returns English directly when i18n disabled
/// let msg = get_message("error-unknown-cause", "unknown cause");
///
/// // With i18n enabled - looks up in user locale, falls back to English
/// let msg = get_message("error-file-not-found", "file not found");
/// ```
#[cfg(not(feature = "i18n"))]
#[inline]
pub fn get_message(_id: &str, english: &str) -> String {
    // Zero-cost path: just return the English string
    english.to_string()
}

#[cfg(feature = "i18n")]
pub fn get_message(id: &str, english: &str) -> String {
    get_message_internal(id, english, None)
}

/// Retrieves a localized message with variable substitution.
///
/// When the `i18n` feature is disabled, uses simple string replacement on the
/// English template. When enabled, uses the full Fluent localization system
/// with proper pluralization and grammar rules.
///
/// # Arguments
///
/// * `id` - The message identifier for localization lookup
/// * `english` - The English template with `{key}` placeholders
/// * `args` - Key-value pairs for variable substitution
///
/// # Returns
///
/// A `String` containing the localized message with substitution
///
/// # Examples
///
/// ```
/// use uucore::locale::get_message_with_args;
/// use std::collections::HashMap;
///
/// let mut args = HashMap::new();
/// args.insert("file".to_string(), "test.txt".to_string());
/// args.insert("line".to_string(), "42".to_string());
///
/// // English template with placeholders
/// let msg = get_message_with_args(
///     "error-parse-failed",
///     "failed to parse {file} at line {line}",
///     args
/// );
/// ```
#[cfg(not(feature = "i18n"))]
pub fn get_message_with_args(_id: &str, english: &str, args: HashMap<String, String>) -> String {
    // Fast path: simple string replacement on English template
    let mut result = english.to_string();
    for (key, value) in args {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, &value);
    }
    result
}

#[cfg(feature = "i18n")]
pub fn get_message_with_args(id: &str, english: &str, args: HashMap<String, String>) -> String {
    let mut fluent_args = FluentArgs::new();

    for (key, value) in args {
        // Try to parse as number first for proper pluralization support
        if let Ok(num_val) = value.parse::<i64>() {
            fluent_args.set(key, num_val);
        } else if let Ok(float_val) = value.parse::<f64>() {
            fluent_args.set(key, float_val);
        } else {
            fluent_args.set(key, value);
        }
    }

    get_message_internal(id, english, Some(fluent_args))
}

// Helper macros for convenient usage
#[macro_export]
macro_rules! msg {
    ($id:expr, $english:expr) => {
        $crate::util::locale::get_message($id, $english)
    };
}

#[cfg(not(feature = "i18n"))]
#[macro_export]
macro_rules! msg_args {
    ($id:expr, $english:expr, $($key:expr => $value:expr),*) => {{
        // Fast path: direct string replacement
        let mut result = $english.to_string();
        $(
            let placeholder = format!("{{{}}}", $key);
            result = result.replace(&placeholder, &$value.to_string());
        )*
        result
    }};
}

#[cfg(feature = "i18n")]
#[macro_export]
macro_rules! msg_args {
    ($id:expr, $english:expr, $($key:expr => $value:expr),*) => {{
        let mut args = std::collections::HashMap::new();
        $(
            args.insert($key.to_string(), $value.to_string());
        )*
        $crate::util::locale::get_message_with_args($id, $english, args)
    }};
}

// The rest of the i18n implementation (only compiled with feature enabled)
#[cfg(feature = "i18n")]
struct Localizer {
    primary_bundle: FluentBundle<FluentResource>,
    fallback_bundle: Option<FluentBundle<FluentResource>>,
}

#[cfg(feature = "i18n")]
impl Localizer {
    fn new(primary_bundle: FluentBundle<FluentResource>) -> Self {
        Self {
            primary_bundle,
            fallback_bundle: None,
        }
    }

    fn with_fallback(mut self, fallback_bundle: FluentBundle<FluentResource>) -> Self {
        self.fallback_bundle = Some(fallback_bundle);
        self
    }

    fn format(&self, id: &str, english: &str, args: Option<&FluentArgs>) -> String {
        // Try primary bundle first
        if let Some(message) = self.primary_bundle.get_message(id).and_then(|m| m.value()) {
            let mut errs = Vec::new();
            return self
                .primary_bundle
                .format_pattern(message, args, &mut errs)
                .to_string();
        }

        // Fall back to English bundle if available
        if let Some(ref fallback) = self.fallback_bundle {
            if let Some(message) = fallback.get_message(id).and_then(|m| m.value()) {
                let mut errs = Vec::new();
                return fallback
                    .format_pattern(message, args, &mut errs)
                    .to_string();
            }
        }

        // Final fallback to provided English string
        english.to_string()
    }
}

#[cfg(feature = "i18n")]
thread_local! {
    static LOCALIZER: OnceLock<Localizer> = const { OnceLock::new() };
}

#[cfg(feature = "i18n")]
fn get_message_internal(id: &str, english: &str, args: Option<FluentArgs>) -> String {
    LOCALIZER.with(|lock| {
        lock.get()
            .map(|loc| loc.format(id, english, args.as_ref()))
            .unwrap_or_else(|| {
                // If localizer not initialized, fall back to English
                english.to_string()
            })
    })
}

#[cfg(feature = "i18n")]
fn init_localization(
    locale: &LanguageIdentifier,
    locales_dir: &Path,
) -> Result<(), LocalizationError> {
    let en_locale = LanguageIdentifier::from_str(DEFAULT_LOCALE)
        .expect("Default locale should always be valid");

    let english_bundle = create_bundle(&en_locale, locales_dir)?;
    let loc = if locale == &en_locale {
        Localizer::new(english_bundle)
    } else {
        if let Ok(primary_bundle) = create_bundle(locale, locales_dir) {
            Localizer::new(primary_bundle).with_fallback(english_bundle)
        } else {
            Localizer::new(english_bundle)
        }
    };

    LOCALIZER.with(|lock| {
        lock.set(loc)
            .map_err(|_| LocalizationError::Bundle("Localizer already initialized".into()))
    })?;
    Ok(())
}

#[cfg(feature = "i18n")]
fn create_bundle(
    locale: &LanguageIdentifier,
    locales_dir: &Path,
) -> Result<FluentBundle<FluentResource>, LocalizationError> {
    let locale_path = locales_dir.join(format!("{locale}.ftl"));

    let ftl_file = fs::read_to_string(&locale_path).map_err(|e| LocalizationError::Io {
        source: e,
        path: locale_path.clone(),
    })?;

    let resource = FluentResource::try_new(ftl_file).map_err(|_| {
        LocalizationError::Parse(format!(
            "Failed to parse localization resource for {}: {}",
            locale,
            locale_path.display()
        ))
    })?;

    let mut bundle = FluentBundle::new(vec![locale.clone()]);

    bundle.add_resource(resource).map_err(|errs| {
        LocalizationError::Bundle(format!(
            "Failed to add resource to bundle for {}: {:?}",
            locale, errs
        ))
    })?;

    Ok(bundle)
}

#[cfg(feature = "i18n")]
fn detect_system_locale() -> Result<LanguageIdentifier, LocalizationError> {
    let locale_str = std::env::var("LANG")
        .unwrap_or_else(|_| DEFAULT_LOCALE.to_string())
        .split('.')
        .next()
        .unwrap_or(DEFAULT_LOCALE)
        .to_string();

    LanguageIdentifier::from_str(&locale_str)
        .map_err(|_| LocalizationError::Parse(format!("Failed to parse locale: {}", locale_str)))
}

/// Sets up localization using the system locale.
///
/// This function only has an effect when the `i18n` feature is enabled.
/// When disabled, all `get_message` calls use the provided English strings
/// for zero-cost localization.
///
/// # Arguments
///
/// * `app_name` - Name of the application (used to find locale directory)
///
/// # Returns
///
/// * `Ok(())` if initialization succeeds or if `i18n` feature is disabled
/// * `Err(LocalizationError)` if initialization fails (only with `i18n` feature)
///
/// # Examples
///
/// ```
/// use uucore::locale::setup_localization;
///
/// // With i18n feature: initializes full localization system
/// // Without i18n feature: no-op, returns Ok(())
/// setup_localization("mkdir")?;
///
/// // Now all get_message calls will use localization (if enabled)
/// // or fall back to the provided English strings
/// ```
#[cfg(not(feature = "i18n"))]
#[inline]
pub fn setup_localization(_app_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "ICI0");

    // No-op when i18n is disabled - always use English strings
    Ok(())
}

#[cfg(feature = "i18n")]
pub fn setup_localization(app_name: &str) -> Result<(), LocalizationError> {

    let locale = detect_system_locale().unwrap_or_else(|_| {
        LanguageIdentifier::from_str(DEFAULT_LOCALE).expect("Default locale should always be valid")
    });

    let locales_dir = get_locales_dir(app_name)?;
    init_localization(&locale, &locales_dir)
}

#[cfg(feature = "i18n")]
fn get_locales_dir(app_name: &str) -> Result<PathBuf, LocalizationError> {
    #[cfg(debug_assertions)]
    {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let dev_path = PathBuf::from(manifest_dir)
            .join("../uu")
            .join(app_name)
            .join("locales");

        if dev_path.exists() {
            return Ok(dev_path);
        }

        let fallback_dev_path = PathBuf::from(manifest_dir).join("locales");
        if fallback_dev_path.exists() {
            return Ok(fallback_dev_path);
        }

        Err(LocalizationError::LocalesDirNotFound(format!(
            "Development locales directory not found at {} or {}",
            dev_path.display(),
            fallback_dev_path.display()
        )))
    }

    #[cfg(not(debug_assertions))]
    {
        use std::env;
        let exe_path = env::current_exe().map_err(|e| {
            LocalizationError::PathResolution(format!("Failed to get executable path: {}", e))
        })?;

        let exe_dir = exe_path.parent().ok_or_else(|| {
            LocalizationError::PathResolution("Failed to get executable directory".to_string())
        })?;

        let coreutils_path = exe_dir.join("locales").join(app_name);
        if coreutils_path.exists() {
            return Ok(coreutils_path);
        }

        let fallback_path = exe_dir.join("locales");
        if fallback_path.exists() {
            return Ok(fallback_path);
        }

        Err(LocalizationError::LocalesDirNotFound(format!(
            "Release locales directory not found at {} or {}",
            coreutils_path.display(),
            fallback_path.display()
        )))
    }
}
