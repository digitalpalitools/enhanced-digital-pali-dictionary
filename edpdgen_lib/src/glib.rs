use std::cmp::Ordering;

/**
 * From: https://stackoverflow.com/a/13225961/6196679
 *
 * Compares two strings, ignoring the case of ASCII characters. It treats
 * non-ASCII characters taking in account case differences. This is an
 * attempt to mimic glib's string utility function
 * <a href="http://developer.gnome.org/glib/2.28/glib-String-Utility-Functions.html#g-ascii-strcasecmp">g_ascii_strcasecmp ()</a>.
 *
 * This is a slightly modified version of java.lang.String.CASE_INSENSITIVE_ORDER.compare(String s1, String s2) method.
 *
 * @param str1  string to compare with str2
 * @param str2  string to compare with str1
 * @return      0 if the strings match, a negative value if str1 < str2, or a positive value if str1 > str2
 */
pub fn g_ascii_strcasecmp(str1: &str, str2: &str) -> Ordering {
    let str_vec1: Vec<char> = str1.chars().collect();
    let str_vec2: Vec<char> = str2.chars().collect();
    let n1 = str_vec1.len();
    let n2 = str_vec2.len();
    let c127 = 127 as char;

    let min = n1.min(n2);
    for i in 0..min {
        let c1 = str_vec1[i];
        let c2 = str_vec2[i];
        if c1 != c2 {
            if c1 > c127 || c2 > c127 {
                // If non-ASCII char...
                return c1.cmp(&c2);
            } else {
                let c1uc = c1.to_ascii_uppercase();
                let c2uc = c2.to_ascii_uppercase();
                if c1uc != c2uc {
                    let c1lc = c1.to_ascii_lowercase();
                    let c2lc = c2.to_ascii_lowercase();
                    if c1lc != c2lc {
                        return c1lc.cmp(&c2lc);
                    }
                }
            }
        }
    }

    n1.cmp(&n2)
}
