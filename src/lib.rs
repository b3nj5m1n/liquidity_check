use std::collections::HashSet;

use lazy_static::lazy_static;
use rusty_money::{iso::*, LocalFormat};

/// All currencies currently supported by [rusty_money]
const CURRENCIES: &'static [&Currency] = &[
    AED, AFN, ALL, AMD, ANG, AOA, ARS, AUD, AWG, AZN, BAM, BBD, BDT, BGN, BHD, BIF, BMD, BND, BOB,
    BRL, BSD, BTN, BWP, BYN, BYR, BZD, CAD, CDF, CHF, CLF, CLP, CNY, COP, CRC, CUC, CUP, CVE, CZK,
    DJF, DKK, DOP, DZD, EGP, ERN, ETB, EUR, FJD, FKP, GBP, GEL, GHS, GIP, GMD, GNF, GTQ, GYD, HKD,
    HNL, HRK, HTG, HUF, IDR, ILS, INR, IQD, IRR, ISK, JMD, JOD, JPY, KES, KGS, KHR, KMF, KPW, KRW,
    KWD, KYD, KZT, LAK, LBP, LKR, LRD, LSL, LYD, MAD, MDL, MGA, MKD, MMK, MNT, MOP, MRU, MUR, MVR,
    MWK, MXN, MYR, MZN, NAD, NGN, NIO, NOK, NPR, NZD, OMR, PAB, PEN, PGK, PHP, PKR, PLN, PYG, QAR,
    RON, RSD, RUB, RWF, SAR, SBD, SCR, SDG, SEK, SGD, SHP, SKK, SLL, SOS, SRD, SSP, STD, STN, SVC,
    SYP, SZL, THB, TJS, TMT, TND, TOP, TRY, TTD, TWD, TZS, UAH, UGX, USD, UYU, UYW, UZS, VES, VND,
    VUV, WST, XAF, XAG, XAU, XBA, XBB, XBC, XBD, XCD, XDR, XOF, XPD, XPF, XPT, XTS, YER, ZAR, ZMK,
    ZMW, ZWL,
];

lazy_static! {
    static ref SEPARATORS_DIGIT: HashSet<char> = {
        let mut m = HashSet::new();
        for currency in CURRENCIES {
            m.insert(LocalFormat::from_locale(currency.locale).digit_separator);
        }
        m
    };
    static ref SEPARATORS_EXPONENT: HashSet<char> = {
        let mut m = HashSet::new();
        for currency in CURRENCIES {
            m.insert(LocalFormat::from_locale(currency.locale).exponent_separator);
        }
        m
    };
    static ref SYMBOLS_CODES: HashSet<&'static str> = {
        let mut m = HashSet::new();
        for currency in CURRENCIES {
            m.insert(currency.symbol);
            m.insert(currency.iso_alpha_code);
        }
        m
    };
}

fn is_seperator(input: &char) -> bool {
    SEPARATORS_DIGIT.contains(input)
}

/// Takes a string as input and splits it into symbol/code and value parts
/// ```
/// # use liquidity_check::split;
/// assert_eq!(split("$50").unwrap(), ("$".into(), "50".into()));
/// # assert_eq!(split("$ 50").unwrap(), ("$".into(), "50".into()));
/// assert_eq!(split("50$").unwrap(), ("50".into(), "$".into()));
/// # assert_eq!(split("50 $").unwrap(), ("50".into(), "$".into()));
/// # assert_eq!(split("USD50").unwrap(), ("USD".into(), "50".into()));
/// # assert_eq!(split("USD 50").unwrap(), ("USD".into(), "50".into()));
/// # assert_eq!(split("50USD").unwrap(), ("50".into(), "USD".into()));
/// # assert_eq!(split("50.0 $").unwrap(), ("50.0".into(), "$".into()));
/// assert_eq!(split("50 USD").unwrap(), ("50".into(), "USD".into()));
/// assert_eq!(split("50"), None);
/// assert_eq!(split("50,000 PAB").unwrap(), ("50,000".into(), "PAB".into()));
/// ```
pub fn split(input: &str) -> Option<(String, String)> {
    // Remove whitespace
    let s = input.split_whitespace().collect::<String>();
    // Check if the first character is a digit
    let is_value_first: bool = match s.chars().next() {
        Some(c) => c.is_digit(10),
        None => return None,
    };
    let first = s
        .chars()
        .take_while(|c| is_value_first == c.is_digit(10) || is_seperator(c))
        .collect::<String>();
    let second = s
        .chars()
        .skip(first.chars().count())
        .take_while(|c| is_value_first != c.is_digit(10) || is_seperator(c))
        .collect::<String>();
    if first.is_empty() || second.is_empty() {
        return None;
    }
    Some((first, second))
}

/// Takes a string as input and returns true if it represents a monetary value
/// ```
/// # use liquidity_check::validate;
/// assert_eq!(validate("$50"), true);
/// assert_eq!(validate("€ 50"), true);
/// assert_eq!(validate("50 EUR"), true);
/// assert_eq!(validate("50.0 ¥"), true);
/// assert_eq!(validate("50,000 PAB"), true);
/// assert_eq!(validate("50"), false);
/// assert_eq!(validate("50 ER"), false);
/// assert_eq!(validate("50_$"), false);
/// ```
pub fn validate(input: &str) -> bool {
    let split = match split(input) {
        Some(s) => s,
        None => return false,
    };
    if !SYMBOLS_CODES.contains(split.0.as_str()) && !SYMBOLS_CODES.contains(split.1.as_str()) {
        return false;
    }
    if split
        .0
        .chars()
        .filter(|c| !SEPARATORS_DIGIT.contains(c))
        .collect::<String>()
        .parse::<f64>()
        .is_err()
        && split
            .1
            .chars()
            .filter(|c| !SEPARATORS_DIGIT.contains(c))
            .collect::<String>()
            .parse::<f64>()
            .is_err()
    {
        return false;
    }
    true
}
