/// The `Complexity` enum is used to determine how a token is parsed.
///
/// A token can have no arguments, a single argument, or multiple arguments. This corresponds to the
/// `None`, `Simple`, and `Complex` variants, respectively.
pub enum TagComplexity {
    /// The token affects raws by itself with no arguments
    None,
    /// The token affects raws and requires a single argument
    Simple,
    /// The token affects raws and requires multiple arguments
    Complex,
}
