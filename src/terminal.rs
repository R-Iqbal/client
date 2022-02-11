use std::error::Error;

use console::Term;
use dialoguer::theme::{ColorfulTheme, Theme};
use indicatif::ProgressBar;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct Keypair(RsaPublicKey, RsaPrivateKey);

pub struct Terminal {
    term: Term,
    theme: Box<dyn Theme>,
}
impl Terminal {
    pub fn new() -> Result<Terminal, Box<dyn Error>> {
        // Creates a new terminal context and clears the scren
        let term = Term::stdout();
        term.clear_screen()?;

        let theme = ColorfulTheme::default();

        Ok(Terminal {
            term,
            theme: Box::new(theme),
        })
    }
    /// Generates an asymmetric keypair using OS randomness
    /// and displays a progress bar in the terminal to indicate
    /// the main thread is being block by the keypair generation.  
    pub fn generate_keypair(&self) -> Result<Keypair, Box<dyn Error>> {
        let progress_bar = ProgressBar::new_spinner();

        // Every 100 ms the progress bar will *tick*
        progress_bar.enable_steady_tick(100);

        // Generate randomness using the operating system to generate the key
        let mut rng = OsRng;
        let bits = 2048;
        progress_bar.set_message("Generating key pair");

        let private_key = RsaPrivateKey::new(&mut rng, bits)?;

        let public_key = RsaPublicKey::from(&private_key);

        // Remove the placeholder message which is associated with the loading bar
        progress_bar.abandon_with_message("");

        return Ok(Keypair(public_key, private_key));
    }
}
