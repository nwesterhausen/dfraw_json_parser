/// The `Searchable` trait is used to define a method that returns a vector of strings that can be
/// used to search for an object. This is used to search for raws in the raws database.
pub trait Searchable {
    /// The `get_search_vec` function returns a vector of strings that can be used to search for an
    /// object.
    fn get_search_vec(&self) -> Vec<String>;
}

/// The `get_search_string` function takes an object that implements the `Searchable` trait and
/// returns a string that can be used to search for the object.
pub fn get_search_string(object: &dyn Searchable) -> String {
    object.get_search_vec().join(" ")
}

/// The `clean_search_vec` function takes a vector of strings, cleans and filters the strings, and
/// returns a new vector. This is used to clean up search terms before they are used to search for
/// raws.
///
/// This function is used by the `Searchable` trait.
///
/// Arguments:
///
/// * `vec`: A vector of strings representing search terms.
///
/// Returns:
///
/// The function `clean_search_vec` returns a `Vec<String>`.
pub fn clean_search_vec(vec: &[String]) -> Vec<String> {
    let mut vec: Vec<String> = vec.join(" ").split_whitespace().map(String::from).collect();

    // Lowercase everything
    vec = vec.iter().map(|x| x.to_lowercase()).collect();

    // Remove any periods, commas, etc.
    vec = vec
        .iter()
        .map(|x| x.replace('.', ""))
        .map(|x| x.replace(',', ""))
        .map(|x| x.replace('(', ""))
        .map(|x| x.replace(')', ""))
        .map(|x| x.replace(';', ""))
        // ! This is dangerous, because it can obscure bad tag parsing.
        .map(|x| x.replace(':', " "))
        .collect();

    // Uniq the vec
    vec.sort();
    vec.dedup();

    // Remove some generic words
    vec.retain(|x| !x.eq_ignore_ascii_case("creature"));
    vec.retain(|x| !x.eq_ignore_ascii_case("all"));
    vec.retain(|x| !x.eq_ignore_ascii_case("the"));
    vec.retain(|x| !x.eq_ignore_ascii_case("of"));
    vec.retain(|x| !x.eq_ignore_ascii_case("in"));
    vec.retain(|x| !x.eq_ignore_ascii_case("and"));
    vec.retain(|x| !x.eq_ignore_ascii_case("a"));
    vec.retain(|x| !x.eq_ignore_ascii_case("an"));
    vec.retain(|x| !x.eq_ignore_ascii_case("with"));

    vec
}
