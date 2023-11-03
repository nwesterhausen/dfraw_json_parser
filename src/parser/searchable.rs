/// The `Searchable` trait is used to allow objects to be searched for using a search string.
///
/// These let you search for things like "egg" or "oil" and find creatures, plants, etc that have
/// those words in their raws (e.g. the creature has a caste with `LaysEggs` or a plant can be pressed
/// to oil.)
///
/// The `Searchable` trait is implemented for all structs that are parsed from raws.
pub trait Searchable {
    /// The `get_search_vec` function returns a vector of strings that can be used to search for
    /// this object. This is primarily a convenience piece because it's one step from `get_search_vec`
    /// to creating a string combining all the search terms.
    fn get_search_vec(&self) -> Vec<String>;
}

/// The `get_search_string` function takes an object that implements the `Searchable` trait and
/// returns a string that can be used to search for that object. This simply calls a `join` on the
/// vector returned by `get_search_vec`.
///
/// This calls `clean_search_vec` on the vector returned by `get_search_vec` to clean up the search
/// terms. (i.e. remove duplicate words, remove generic words like "the", lowercase the entire string, etc.)
///
/// Arguments:
///
/// * `object`: A reference to an object that implements the `Searchable` trait.
///
/// Returns:
///
/// A string with all the search terms for the object.
pub fn get_search_string(object: &dyn Searchable) -> String {
    clean_search_vec(object.get_search_vec().as_slice()).join(" ")
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
