/// Converts a telugu unicode character to devanagari.
///
/// # Examples
///
/// ```
/// let arg = 'శ';
/// let answer = teltools::tel2hin(arg);
///
/// assert_eq!('श', answer);
/// ```
pub fn tel2hin(telchar: char) -> char {
    let telmin = '\u{0c00}' as u32;
    let telmax = '\u{0c7f}' as u32;
    let hinmin = '\u{0900}' as u32;
    let mut c = telchar as u32;
    let delta = telmin - hinmin;
    if (c >= telmin) && (c <= telmax) {
        c = c - delta;
    }
    match std::char::from_u32(c) {
        Some(c) => c,
        None => telchar,
    }
}

/// Converts a devanagari unicode character to telugu.
///
/// # Examples
///
/// ```
/// let arg = 'श';
/// let answer = teltools::hin2tel(arg);
///
/// assert_eq!('శ', answer);
/// ```
pub fn hin2tel(hinchar: char) -> char {
    let telmin = '\u{0c00}' as u32;
    let hinmin = '\u{0900}' as u32;
    let hinmax = '\u{097f}' as u32;
    let mut c = hinchar as u32;
    let delta = telmin - hinmin;
    if (c >= hinmin)
        && (c <= hinmax)
        && (hinchar != '\u{0950}')
        && (hinchar != '\u{0951}')
        && (hinchar != '\u{0952}')
        && (hinchar != '\u{0953}')
        && (hinchar != '\u{0954}')
        && (hinchar != '\u{0964}')
        && (hinchar != '\u{0965}')
    {
        c = c + delta;
    }
    match std::char::from_u32(c) {
        Some(c) => c,
        None => hinchar,
    }
}

enum Context {
    Vowel,
    Consonant,
    Mark,
    Other,
}

struct ResultStr {
    pub m_result: String,
    pub m_alt_result: String,
    pub m_context: Context,
}

use std::collections::BTreeMap;

const HALANT: char = '\u{0c4d}';

/// Transliterates an English string into Telugu.
///
/// # Examples
///
/// ```
/// let arg = "om";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("ॐ", answer);
/// ```
///
/// ```
/// let arg = "1";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("౧", answer);
/// ```
///
/// ```
/// let arg = "om gaM gaNapatayE namaH ||";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("ॐ గం గణపతయే నమః ॥", answer);
/// ```
///
/// ```
/// let arg = "om shrI mahA saraswatyai namaH ||\nom shrI gurubhyOH namaH ||";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("ॐ శ్రీ మహా సరస్వత్యై నమః ॥\nॐ శ్రీ గురుభ్యోః నమః ॥", answer);
/// ```
///
/// ```
/// let arg = "prathamO daivyO bhiShak";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("ప్రథమో దైవ్యో భిషక్", answer);
/// ```
///
/// ```
/// let arg = "mAm rakShatu";
/// let answer = teltools::trans(arg);
///
/// assert_eq!("మామ్ రక్షతు", answer);
/// ```
pub fn trans(line: &str) -> String {
    let map = get_telugu_trans_map();
    let mut index = 0;
    let mut out = String::new();

    let mut context = &Context::Other;

    while index < line.len() {
        let mut found_match = false;

        for k in map.keys().rev() {
            if line[index..].starts_with(k) {
                found_match = true;
                index = index + k.len();
                if let Some(v) = map.get(k) {
                    let (result, ctx) = append_output(&v, &context);
                    out.push_str(&result);
                    context = ctx;
                } else {
                    out.push_str(k);
                }
                continue;
            }
        }
        if !found_match {
            match context {
                Context::Consonant => out.push(HALANT),
                _ => {}
            }
            out.push_str(&line[index..index + 1]);
            index = index + 1;
            context = &Context::Other;
        }
    }
    match context {
        Context::Consonant => out.push(HALANT),
        _ => {}
    }

    return out;
}

fn append_output<'a>(
    result_str: &'a ResultStr,
    current_context: &'a Context,
) -> (String, &'a Context) {
    let ctx = &result_str.m_context;
    let res = &result_str.m_result;
    let altres = &result_str.m_alt_result;

    let mut result = String::new();

    match current_context {
        Context::Consonant => match ctx {
            Context::Consonant => result.push_str(&altres),
            Context::Vowel => result.push_str(&altres),
            _ctx1 => result.push_str(&res),
        },
        _ctx2 => {
            result.push_str(&res);
        }
    };

    return (result, ctx);
}

fn get_telugu_trans_map<'a>() -> BTreeMap<&'a str, ResultStr> {
    let mut map = BTreeMap::new();

    map.insert(
        ".N",
        ResultStr {
            m_result: String::from("\u{0c01}"),
            m_alt_result: String::from("\u{0c01}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_CANDRABINDU
    map.insert(
        ".n",
        ResultStr {
            m_result: String::from("\u{0c02}"),
            m_alt_result: String::from("\u{0c02}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_ANUSVARA
    map.insert(
        "M",
        ResultStr {
            m_result: String::from("\u{0c02}"),
            m_alt_result: String::from("\u{0c02}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_ANUSVARA
    map.insert(
        "H",
        ResultStr {
            m_result: String::from("\u{0c03}"),
            m_alt_result: String::from("\u{0c03}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_VISARGA
    map.insert(
        "^a",
        ResultStr {
            m_result: String::from("\u{0c04}"),
            m_alt_result: String::from("\u{0c04}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_SHORT_A
    map.insert(
        "a",
        ResultStr {
            m_result: String::from("\u{0c05}"),
            m_alt_result: String::from(""),
            m_context: Context::Vowel,
        },
    ); // LETTER_A
    map.insert(
        "A",
        ResultStr {
            m_result: String::from("\u{0c06}"),
            m_alt_result: String::from("\u{0c3e}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_AA
    map.insert(
        "aa",
        ResultStr {
            m_result: String::from("\u{0c06}"),
            m_alt_result: String::from("\u{0c3e}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_AA
    map.insert(
        "i",
        ResultStr {
            m_result: String::from("\u{0c07}"),
            m_alt_result: String::from("\u{0c3f}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_I
    map.insert(
        "I",
        ResultStr {
            m_result: String::from("\u{0c08}"),
            m_alt_result: String::from("\u{0c40}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_II
    map.insert(
        "ii",
        ResultStr {
            m_result: String::from("\u{0c08}"),
            m_alt_result: String::from("\u{0c40}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_II
    map.insert(
        "u",
        ResultStr {
            m_result: String::from("\u{0c09}"),
            m_alt_result: String::from("\u{0c41}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_U
    map.insert(
        "U",
        ResultStr {
            m_result: String::from("\u{0c0a}"),
            m_alt_result: String::from("\u{0c42}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_UU
    map.insert(
        "uu",
        ResultStr {
            m_result: String::from("\u{0c0a}"),
            m_alt_result: String::from("\u{0c42}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_UU
    map.insert(
        "R^i",
        ResultStr {
            m_result: String::from("\u{0c0b}"),
            m_alt_result: String::from("\u{0c43}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_VOCALIC_R
    map.insert(
        "L^i",
        ResultStr {
            m_result: String::from("\u{0c0c}"),
            m_alt_result: String::from("\u{0c44}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_VOCALIC_L
    map.insert(
        "^e",
        ResultStr {
            m_result: String::from("\u{0c0d}"),
            m_alt_result: String::from("\u{0c45}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_CANDRA_E
    map.insert(
        "e",
        ResultStr {
            m_result: String::from("\u{0c0e}"),
            m_alt_result: String::from("\u{0c46}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_E
    map.insert(
        "E",
        ResultStr {
            m_result: String::from("\u{0c0f}"),
            m_alt_result: String::from("\u{0c47}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_EE
    map.insert(
        "ee",
        ResultStr {
            m_result: String::from("\u{0c0f}"),
            m_alt_result: String::from("\u{0c47}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_EE
    map.insert(
        "ai",
        ResultStr {
            m_result: String::from("\u{0c10}"),
            m_alt_result: String::from("\u{0c48}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_AI
    map.insert(
        "^o",
        ResultStr {
            m_result: String::from("\u{0c11}"),
            m_alt_result: String::from("\u{0c49}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_CANDRA_O
    map.insert(
        "o",
        ResultStr {
            m_result: String::from("\u{0c12}"),
            m_alt_result: String::from("\u{0c4a}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_O
    map.insert(
        "O",
        ResultStr {
            m_result: String::from("\u{0c13}"),
            m_alt_result: String::from("\u{0c4b}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_OO
    map.insert(
        "oo",
        ResultStr {
            m_result: String::from("\u{0c13}"),
            m_alt_result: String::from("\u{0c4b}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_OO
    map.insert(
        "au",
        ResultStr {
            m_result: String::from("\u{0c14}"),
            m_alt_result: String::from("\u{0c4c}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_AU
    map.insert(
        "k",
        ResultStr {
            m_result: String::from("\u{0c15}"),
            m_alt_result: String::from("\u{0c4d}\u{0c15}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_KA
    map.insert(
        "kSh",
        ResultStr {
            m_result: String::from("\u{0c15}\u{0c4d}\u{0c37}"),
            m_alt_result: String::from("\u{0c4d}\u{0c15}\u{0c4d}\u{0c37}"),
            m_context: Context::Consonant,
        },
    ); // CONJUNCT_KSHA
    map.insert(
        "x",
        ResultStr {
            m_result: String::from("\u{0c15}\u{0c4d}\u{0c37}"),
            m_alt_result: String::from("\u{0c4d}\u{0c15}\u{0c4d}\u{0c37}"),
            m_context: Context::Consonant,
        },
    ); // CONJUNCT_KSHA
    map.insert(
        "kh",
        ResultStr {
            m_result: String::from("\u{0c16}"),
            m_alt_result: String::from("\u{0c4d}\u{0c16}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_KHA
    map.insert(
        "g",
        ResultStr {
            m_result: String::from("\u{0c17}"),
            m_alt_result: String::from("\u{0c4d}\u{0c17}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_GA
    map.insert(
        "ga.N",
        ResultStr {
            m_result: String::from("\u{0c17}\u{0c4d}\u{0c02}"),
            m_alt_result: String::from("\u{0c4d}\u{0c17}\u{0c4d}\u{0c02}"),
            m_context: Context::Consonant,
        },
    ); // CONJUNCT_GUM
    map.insert(
        "gh",
        ResultStr {
            m_result: String::from("\u{0c18}"),
            m_alt_result: String::from("\u{0c4d}\u{0c18}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_GHA
    map.insert(
        "~N",
        ResultStr {
            m_result: String::from("\u{0c19}"),
            m_alt_result: String::from("\u{0c4d}\u{0c19}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NGA
    map.insert(
        "N^",
        ResultStr {
            m_result: String::from("\u{0c19}"),
            m_alt_result: String::from("\u{0c4d}\u{0c19}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NGA
    map.insert(
        "ch",
        ResultStr {
            m_result: String::from("\u{0c1a}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1a}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_CA
    map.insert(
        "chh",
        ResultStr {
            m_result: String::from("\u{0c1b}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1b}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_CHA
    map.insert(
        "j",
        ResultStr {
            m_result: String::from("\u{0c1c}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1c}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_JA
    map.insert(
        "jh",
        ResultStr {
            m_result: String::from("\u{0c1d}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1d}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_JHA
    map.insert(
        "~n",
        ResultStr {
            m_result: String::from("\u{0c1e}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1e}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NYA
    map.insert(
        "JN",
        ResultStr {
            m_result: String::from("\u{0c1e}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1e}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NYA
    map.insert(
        "T",
        ResultStr {
            m_result: String::from("\u{0c1f}"),
            m_alt_result: String::from("\u{0c4d}\u{0c1f}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_TTA
    map.insert(
        "Th",
        ResultStr {
            m_result: String::from("\u{0c20}"),
            m_alt_result: String::from("\u{0c4d}\u{0c20}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_TTHA
    map.insert(
        "D",
        ResultStr {
            m_result: String::from("\u{0c21}"),
            m_alt_result: String::from("\u{0c4d}\u{0c21}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_DDA
    map.insert(
        "Dh",
        ResultStr {
            m_result: String::from("\u{0c22}"),
            m_alt_result: String::from("\u{0c4d}\u{0c22}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_DDHA
    map.insert(
        "N",
        ResultStr {
            m_result: String::from("\u{0c23}"),
            m_alt_result: String::from("\u{0c4d}\u{0c23}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NNA
    map.insert(
        "t",
        ResultStr {
            m_result: String::from("\u{0c24}"),
            m_alt_result: String::from("\u{0c4d}\u{0c24}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_TA
    map.insert(
        "th",
        ResultStr {
            m_result: String::from("\u{0c25}"),
            m_alt_result: String::from("\u{0c4d}\u{0c25}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_THA
    map.insert(
        "d",
        ResultStr {
            m_result: String::from("\u{0c26}"),
            m_alt_result: String::from("\u{0c4d}\u{0c26}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_DA
    map.insert(
        "dh",
        ResultStr {
            m_result: String::from("\u{0c27}"),
            m_alt_result: String::from("\u{0c4d}\u{0c27}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_DHA
    map.insert(
        "n",
        ResultStr {
            m_result: String::from("\u{0c28}"),
            m_alt_result: String::from("\u{0c4d}\u{0c28}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NA
    map.insert(
        "N.",
        ResultStr {
            m_result: String::from("\u{0c29}"),
            m_alt_result: String::from("\u{0c4d}\u{0c29}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_NNNA
    map.insert(
        "p",
        ResultStr {
            m_result: String::from("\u{0c2a}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2a}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_PA
    map.insert(
        "ph",
        ResultStr {
            m_result: String::from("\u{0c2b}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2b}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_PHA
    map.insert(
        "b",
        ResultStr {
            m_result: String::from("\u{0c2c}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2c}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_BA
    map.insert(
        "bh",
        ResultStr {
            m_result: String::from("\u{0c2d}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2d}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_BHA
    map.insert(
        "m",
        ResultStr {
            m_result: String::from("\u{0c2e}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2e}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_MA
    map.insert(
        "y",
        ResultStr {
            m_result: String::from("\u{0c2f}"),
            m_alt_result: String::from("\u{0c4d}\u{0c2f}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_YA
    map.insert(
        "r",
        ResultStr {
            m_result: String::from("\u{0c30}"),
            m_alt_result: String::from("\u{0c4d}\u{0c30}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_RA
    map.insert(
        "R",
        ResultStr {
            m_result: String::from("\u{0c31}"),
            m_alt_result: String::from("\u{0c4d}\u{0c31}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_RRA
    map.insert(
        "l",
        ResultStr {
            m_result: String::from("\u{0c32}"),
            m_alt_result: String::from("\u{0c4d}\u{0c32}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_LA
    map.insert(
        "L",
        ResultStr {
            m_result: String::from("\u{0c33}"),
            m_alt_result: String::from("\u{0c4d}\u{0c33}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_LLA
    map.insert(
        ".L",
        ResultStr {
            m_result: String::from("\u{0c34}"),
            m_alt_result: String::from("\u{0c4d}\u{0c34}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_LLLA
    map.insert(
        "v",
        ResultStr {
            m_result: String::from("\u{0c35}"),
            m_alt_result: String::from("\u{0c4d}\u{0c35}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_VA
    map.insert(
        "w",
        ResultStr {
            m_result: String::from("\u{0c35}"),
            m_alt_result: String::from("\u{0c4d}\u{0c35}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_VA
    map.insert(
        "sh",
        ResultStr {
            m_result: String::from("\u{0c36}"),
            m_alt_result: String::from("\u{0c4d}\u{0c36}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_SHA
    map.insert(
        "Sh",
        ResultStr {
            m_result: String::from("\u{0c37}"),
            m_alt_result: String::from("\u{0c4d}\u{0c37}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_SSA
    map.insert(
        "shh",
        ResultStr {
            m_result: String::from("\u{0c37}"),
            m_alt_result: String::from("\u{0c4d}\u{0c37}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_SSA
    map.insert(
        "s",
        ResultStr {
            m_result: String::from("\u{0c38}"),
            m_alt_result: String::from("\u{0c4d}\u{0c38}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_SA
    map.insert(
        "h",
        ResultStr {
            m_result: String::from("\u{0c39}"),
            m_alt_result: String::from("\u{0c4d}\u{0c39}"),
            m_context: Context::Consonant,
        },
    ); // LETTER_HA
    map.insert(
        "^n",
        ResultStr {
            m_result: String::from("\u{0c3c}"),
            m_alt_result: String::from("\u{0c3c}"),
            m_context: Context::Mark,
        },
    ); // SIGN_NUKTA
    map.insert(
        ".a",
        ResultStr {
            m_result: String::from("\u{0c3d}"),
            m_alt_result: String::from("\u{0c3d}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_AVAGRAHA
    map.insert(
        ".h",
        ResultStr {
            m_result: String::from("\u{0c4d}"),
            m_alt_result: String::from("\u{0c4d}"),
            m_context: Context::Mark,
        },
    ); // SIGN_VIRAMA
    map.insert(
        ".r",
        ResultStr {
            m_result: String::from("\u{0c53}"),
            m_alt_result: String::from("\u{0c53}"),
            m_context: Context::Mark,
        },
    ); // SIGN_GRAVE_ACCENT
    map.insert(
        ".e",
        ResultStr {
            m_result: String::from("\u{0c54}"),
            m_alt_result: String::from("\u{0c54}"),
            m_context: Context::Mark,
        },
    ); // SIGN_ACUTE_ACCENT
    map.insert(
        "XAA",
        ResultStr {
            m_result: String::from("\u{0c55}"),
            m_alt_result: String::from("\u{0c55}"),
            m_context: Context::Vowel,
        },
    ); // LENGTH_MARK
    map.insert(
        "XAI",
        ResultStr {
            m_result: String::from("\u{0c56}"),
            m_alt_result: String::from("\u{0c56}"),
            m_context: Context::Vowel,
        },
    ); // AI_LENGTH_MARK
    map.insert(
        "XAU",
        ResultStr {
            m_result: String::from("\u{0c57}"),
            m_alt_result: String::from("\u{0c57}"),
            m_context: Context::Vowel,
        },
    ); // AU_LENGTH_MARK
    map.insert(
        "R^I",
        ResultStr {
            m_result: String::from("\u{0c60}"),
            m_alt_result: String::from("\u{0c60}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_VOCALIC_RR
    map.insert(
        "L^I",
        ResultStr {
            m_result: String::from("\u{0c61}"),
            m_alt_result: String::from("\u{0c61}"),
            m_context: Context::Vowel,
        },
    ); // LETTER_VOCALIC_LL
    map.insert(
        "XL^i",
        ResultStr {
            m_result: String::from("\u{0c62}"),
            m_alt_result: String::from("\u{0c62}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_VOCALIC_L
    map.insert(
        "XL^I",
        ResultStr {
            m_result: String::from("\u{0c63}"),
            m_alt_result: String::from("\u{0c63}"),
            m_context: Context::Vowel,
        },
    ); // SIGN_VOCALIC_LL
    map.insert(
        "0",
        ResultStr {
            m_result: String::from("\u{0c66}"),
            m_alt_result: String::from("\u{0c66}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_ZERO
    map.insert(
        "1",
        ResultStr {
            m_result: String::from("\u{0c67}"),
            m_alt_result: String::from("\u{0c67}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_ONE
    map.insert(
        "2",
        ResultStr {
            m_result: String::from("\u{0c68}"),
            m_alt_result: String::from("\u{0c68}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_TWO
    map.insert(
        "3",
        ResultStr {
            m_result: String::from("\u{0c69}"),
            m_alt_result: String::from("\u{0c69}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_THREE
    map.insert(
        "4",
        ResultStr {
            m_result: String::from("\u{0c6a}"),
            m_alt_result: String::from("\u{0c6a}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_FOUR
    map.insert(
        "5",
        ResultStr {
            m_result: String::from("\u{0c6b}"),
            m_alt_result: String::from("\u{0c6b}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_FIVE
    map.insert(
        "6",
        ResultStr {
            m_result: String::from("\u{0c6c}"),
            m_alt_result: String::from("\u{0c6c}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_SIX
    map.insert(
        "7",
        ResultStr {
            m_result: String::from("\u{0c6d}"),
            m_alt_result: String::from("\u{0c6d}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_SEVEN
    map.insert(
        "8",
        ResultStr {
            m_result: String::from("\u{0c6e}"),
            m_alt_result: String::from("\u{0c6e}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_EIGHT
    map.insert(
        "9",
        ResultStr {
            m_result: String::from("\u{0c6f}"),
            m_alt_result: String::from("\u{0c6f}"),
            m_context: Context::Mark,
        },
    ); // DIGIT_NINE
    map.insert(
        "^.",
        ResultStr {
            m_result: String::from("\u{0307}"),
            m_alt_result: String::from("\u{0307}"),
            m_context: Context::Mark,
        },
    ); // DOT_ABOVE
    map.insert(
        "_.",
        ResultStr {
            m_result: String::from("\u{0323}"),
            m_alt_result: String::from("\u{0323}"),
            m_context: Context::Mark,
        },
    ); // DOT_BELOW
    map.insert(
        "om",
        ResultStr {
            m_result: String::from("\u{0950}"),
            m_alt_result: String::from("\u{0950}"),
            m_context: Context::Mark,
        },
    ); // SIGN_OM
    map.insert(
        "AUM",
        ResultStr {
            m_result: String::from("\u{0950}"),
            m_alt_result: String::from("\u{0950}"),
            m_context: Context::Mark,
        },
    ); // SIGN_OM
    map.insert(
        "OM",
        ResultStr {
            m_result: String::from("\u{0950}"),
            m_alt_result: String::from("\u{0950}"),
            m_context: Context::Mark,
        },
    ); // SIGN_OM
    map.insert(
        "\'",
        ResultStr {
            m_result: String::from("\u{0951}"),
            m_alt_result: String::from("\u{0951}"),
            m_context: Context::Mark,
        },
    ); // SIGN_SWARITA
    map.insert(
        "\\_",
        ResultStr {
            m_result: String::from("\u{0952}"),
            m_alt_result: String::from("\u{0952}"),
            m_context: Context::Mark,
        },
    ); // SIGN_ANUDATTA
    map.insert(
        "|",
        ResultStr {
            m_result: String::from("\u{0964}"),
            m_alt_result: String::from("\u{0964}"),
            m_context: Context::Mark,
        },
    ); // SIGN_DANDA
    map.insert(
        "||",
        ResultStr {
            m_result: String::from("\u{0965}"),
            m_alt_result: String::from("\u{0965}"),
            m_context: Context::Mark,
        },
    ); // SIGN_DOUBLE_DANDA
    map.insert(
        "cs",
        ResultStr {
            m_result: String::from("\u{0C58}"),
            m_alt_result: String::from("\u{0C58}"),
            m_context: Context::Consonant,
        },
    ); // TELUGU_LETTER_TSA
    map.insert(
        "dz",
        ResultStr {
            m_result: String::from("\u{0C59}"),
            m_alt_result: String::from("\u{0C59}"),
            m_context: Context::Consonant,
        },
    ); // TELUGU_LETTER_DZA
    map.insert(
        ".H",
        ResultStr {
            m_result: String::from("\u{0C78}"),
            m_alt_result: String::from("\u{0C78}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_ZERO_FOR_ODD_POWERS_OF_FOUR
    map.insert(
        "1/4o",
        ResultStr {
            m_result: String::from("\u{0C79}"),
            m_alt_result: String::from("\u{0C79}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_ONE_FOR_ODD_POWERS_OF_FOUR
    map.insert(
        "2/4o",
        ResultStr {
            m_result: String::from("\u{0C7A}"),
            m_alt_result: String::from("\u{0C7A}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_TWO_FOR_ODD_POWERS_OF_FOUR
    map.insert(
        "3/4o",
        ResultStr {
            m_result: String::from("\u{0C7B}"),
            m_alt_result: String::from("\u{0C7B}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_THREE_FOR_ODD_POWERS_OF_FOUR
    map.insert(
        "1/4e",
        ResultStr {
            m_result: String::from("\u{0C7C}"),
            m_alt_result: String::from("\u{0C7C}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_ONE_FOR_EVEN_POWERS_OF_FOUR
    map.insert(
        "2/4e",
        ResultStr {
            m_result: String::from("\u{0C7D}"),
            m_alt_result: String::from("\u{0C7D}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_TWO_FOR_EVEN_POWERS_OF_FOUR
    map.insert(
        "3/4e",
        ResultStr {
            m_result: String::from("\u{0C7E}"),
            m_alt_result: String::from("\u{0C7E}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_FRACTION_DIGIT_THREE_FOR_EVEN_POWERS_OF_FOUR
    map.insert(
        ".t",
        ResultStr {
            m_result: String::from("\u{0C7F}"),
            m_alt_result: String::from("\u{0C7F}"),
            m_context: Context::Mark,
        },
    ); // TELUGU_SIGN_TUUMU
    map.insert(
        "\\kar",
        ResultStr {
            m_result: String::from("\u{1CD0}"),
            m_alt_result: String::from("\u{1CD0}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_KARSHANA
    map.insert(
        "\\shr",
        ResultStr {
            m_result: String::from("\u{1CD1}"),
            m_alt_result: String::from("\u{1CD1}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_SHARA
    map.insert(
        "\\prn",
        ResultStr {
            m_result: String::from("\u{1CD2}"),
            m_alt_result: String::from("\u{1CD2}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_PRENKHA
    map.insert(
        "\\nih",
        ResultStr {
            m_result: String::from("\u{1CD3}"),
            m_alt_result: String::from("\u{1CD3}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_NIHSHVASA
    map.insert(
        "\\yms",
        ResultStr {
            m_result: String::from("\u{1CD4}"),
            m_alt_result: String::from("\u{1CD4}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_YAJURVEDIC_MIDLINE_SVARITA
    map.insert(
        "\\yais",
        ResultStr {
            m_result: String::from("\u{1CD5}"),
            m_alt_result: String::from("\u{1CD5}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_YAJURVEDIC_AGGRAVATED_INDEPENDENT_SVARITA
    map.insert(
        "\\yis",
        ResultStr {
            m_result: String::from("\u{1CD6}"),
            m_alt_result: String::from("\u{1CD6}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_YAJURVEDIC_INDEPENDENT_SVARITA
    map.insert(
        "\\ykis",
        ResultStr {
            m_result: String::from("\u{1CD7}"),
            m_alt_result: String::from("\u{1CD7}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA
    map.insert(
        "\\cb",
        ResultStr {
            m_result: String::from("\u{1CD8}"),
            m_alt_result: String::from("\u{1CD8}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_CANDRA_BELOW
    map.insert(
        "\\ykiss",
        ResultStr {
            m_result: String::from("\u{1CD9}"),
            m_alt_result: String::from("\u{1CD9}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA_SCHROEDER
    map.insert(
        "\"",
        ResultStr {
            m_result: String::from("\u{1CDA}"),
            m_alt_result: String::from("\u{1CDA}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_DOUBLE_SVARITA
    map.insert(
        "\\3s",
        ResultStr {
            m_result: String::from("\u{1CDB}"),
            m_alt_result: String::from("\u{1CDB}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_TRIPLE_SVARITA
    map.insert(
        "\\ka",
        ResultStr {
            m_result: String::from("\u{1CDC}"),
            m_alt_result: String::from("\u{1CDC}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_KATHAKA_ANUDATTA
    map.insert(
        "\\.b",
        ResultStr {
            m_result: String::from("\u{1CDD}"),
            m_alt_result: String::from("\u{1CDD}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_DOT_BELOW
    map.insert(
        "\\..b",
        ResultStr {
            m_result: String::from("\u{1CDE}"),
            m_alt_result: String::from("\u{1CDE}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_TWO_DOTS_BELOW
    map.insert(
        "\\2.b",
        ResultStr {
            m_result: String::from("\u{1CDE}"),
            m_alt_result: String::from("\u{1CDE}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_TWO_DOTS_BELOW
    map.insert(
        "\\3.b",
        ResultStr {
            m_result: String::from("\u{1CDF}"),
            m_alt_result: String::from("\u{1CDF}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_THREE_DOTS_BELOW
    map.insert(
        "\rkis",
        ResultStr {
            m_result: String::from("\u{1CE0}"),
            m_alt_result: String::from("\u{1CE0}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_RIGVEDIC_KASHMIRI_INDEPENDENT_SVARITA
    map.insert(
        "\\ais",
        ResultStr {
            m_result: String::from("\u{1CE1}"),
            m_alt_result: String::from("\u{1CE1}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_TONE_ATHARVAVEDIC_INDEPENDENT_SVARITA
    map.insert(
        "\\vs",
        ResultStr {
            m_result: String::from("\u{1CE2}"),
            m_alt_result: String::from("\u{1CE2}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_VISARGA_SVARITA
    map.insert(
        "\\vu",
        ResultStr {
            m_result: String::from("\u{1CE3}"),
            m_alt_result: String::from("\u{1CE3}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_VISARGA_UDATTA
    map.insert(
        "\rvu",
        ResultStr {
            m_result: String::from("\u{1CE4}"),
            m_alt_result: String::from("\u{1CE4}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_REVERSED_VISARGA_UDATTA
    map.insert(
        "\\van",
        ResultStr {
            m_result: String::from("\u{1CE5}"),
            m_alt_result: String::from("\u{1CE5}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_VISARGA_ANUDATTA
    map.insert(
        "\rvan",
        ResultStr {
            m_result: String::from("\u{1CE6}"),
            m_alt_result: String::from("\u{1CE6}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_REVERSED_VISARGA_ANUDATTA
    map.insert(
        "\\vut",
        ResultStr {
            m_result: String::from("\u{1CE7}"),
            m_alt_result: String::from("\u{1CE7}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_VISARGA_UDATTA_WITH_TAIL
    map.insert(
        "\\vat",
        ResultStr {
            m_result: String::from("\u{1CE8}"),
            m_alt_result: String::from("\u{1CE8}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_VISARGA_ANUDATTA_WITH_TAIL
    map.insert(
        "\\aag",
        ResultStr {
            m_result: String::from("\u{1CE9}"),
            m_alt_result: String::from("\u{1CE9}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ANUSVARA_ANTARGOMUKHA
    map.insert(
        "\\abg",
        ResultStr {
            m_result: String::from("\u{1CEA}"),
            m_alt_result: String::from("\u{1CEA}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ANUSVARA_BAHIRGOMUKHA
    map.insert(
        "\\avg",
        ResultStr {
            m_result: String::from("\u{1CEB}"),
            m_alt_result: String::from("\u{1CEB}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA
    map.insert(
        "\\avgt",
        ResultStr {
            m_result: String::from("\u{1CEC}"),
            m_alt_result: String::from("\u{1CEC}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA_WITH_TAIL
    map.insert(
        "\\3k",
        ResultStr {
            m_result: String::from("\u{1CED}"),
            m_alt_result: String::from("\u{1CED}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_TIRYAK
    map.insert(
        "\\6n",
        ResultStr {
            m_result: String::from("\u{1CEE}"),
            m_alt_result: String::from("\u{1CEE}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_HEXIFORM_LONG_ANUSVARA
    map.insert(
        "\\ln",
        ResultStr {
            m_result: String::from("\u{1CEF}"),
            m_alt_result: String::from("\u{1CEF}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_LONG_ANUSVARA
    map.insert(
        "\rln",
        ResultStr {
            m_result: String::from("\u{1CF0}"),
            m_alt_result: String::from("\u{1CF0}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_RTHANG_LONG_ANUSVARA
    map.insert(
        "\num",
        ResultStr {
            m_result: String::from("\u{1CF1}"),
            m_alt_result: String::from("\u{1CF1}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ANUSVARA_UBHAYATO_MUKHA
    map.insert(
        "\\avs",
        ResultStr {
            m_result: String::from("\u{1CF2}"),
            m_alt_result: String::from("\u{1CF2}"),
            m_context: Context::Mark,
        },
    ); // VEDIC_SIGN_ARDHAVISARGA

    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() -> Result<(), String> {
        assert_eq!(tel2hin('a'), 'a');
        Ok(())
    }
    #[test]
    fn test_tel() -> Result<(), String> {
        assert_eq!(tel2hin('\u{0c02}'), '\u{0902}');
        Ok(())
    }

    #[test]
    fn test_hin() -> Result<(), String> {
        assert_eq!(tel2hin('\u{0902}'), '\u{0902}');
        Ok(())
    }
}
