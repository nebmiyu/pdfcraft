//! What this build is called. voss-solutions PDF Pro is a renamed build of PdfCraft (MIT OR
//! Apache-2.0, by the ArtCraft team). The code keeps its internal `pdfcraft` names (crates,
//! executables, settings folders); everything a person reads comes from here.

use std::collections::HashMap;
use std::sync::Mutex;

/// The product name shown in the window title, menus, About and installers.
pub const NAME: &str = "voss-solutions PDF Pro";
/// The short name used where the full one is too long (executable, folder, volume names).
pub const SHORT_NAME: &str = "PDF Pro";
/// Who publishes this build.
pub const VENDOR: &str = "voss-solutions";
/// The vendor's web site.
pub const WEBSITE: &str = "https://www.voss-solutions.ch";
/// The project this build is based on, named in About as the licences ask.
pub const UPSTREAM_NAME: &str = "PdfCraft";
/// Where this build's source lives.
pub const SOURCE: &str = "https://github.com/nebmiyu/pdfcraft";

/// English UI strings and their translations name the upstream product; this swaps the name for
/// [`NAME`] at display time, so every language shows the new name without touching the catalogs.
/// A string without the upstream name comes back as it is. Rewritten strings are kept for the
/// life of the process (the set of distinct UI strings is finite), so the result can be handed
/// out with any lifetime.
pub fn rebrand(s: &str) -> &str {
    // The attribution the licences ask for names the upstream project on purpose.
    if !s.contains(UPSTREAM_NAME) || s.to_ascii_lowercase().contains("based on pdfcraft") {
        return s;
    }
    static CACHE: Mutex<Option<HashMap<String, &'static str>>> = Mutex::new(None);
    let mut guard = match CACHE.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let cache = guard.get_or_insert_with(HashMap::new);
    if let Some(done) = cache.get(s) {
        return done;
    }
    let done: &'static str = Box::leak(s.replace(UPSTREAM_NAME, NAME).into_boxed_str());
    cache.insert(s.to_owned(), done);
    done
}

#[cfg(test)]
mod tests {
    use super::{NAME, rebrand};

    #[test]
    fn the_upstream_name_is_replaced_and_other_text_is_untouched() {
        assert_eq!(rebrand("Welcome to PdfCraft"), format!("Welcome to {NAME}"));
        assert_eq!(rebrand("Based on PdfCraft"), "Based on PdfCraft");
        assert_eq!(rebrand("Open a file"), "Open a file");
    }
}
