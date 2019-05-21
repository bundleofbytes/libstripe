use std::fmt;

/// Currency is the list of supported currencies.
///
/// For more details see https://support.stripe.com/questions/which-currencies-does-stripe-support.
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    AED, // United Arab Emirates Dirham
    AFN, // Afghan Afghani
    ALL, // Albanian Lek
    AMD, // Armenian Dram
    ANG, // Netherlands Antillean Gulden
    AOA, // Angolan Kwanza
    ARS, // Argentine Peso
    AUD, // Australian Dollar
    AWG, // Aruban Florin
    AZN, // Azerbaijani Manat
    BAM, // Bosnia & Herzegovina Convertible Mark
    BBD, // Barbadian Dollar
    BDT, // Bangladeshi Taka
    BGN, // Bulgarian Lev
    BIF, // Burundian Franc
    BMD, // Bermudian Dollar
    BND, // Brunei Dollar
    BOB, // Bolivian Boliviano
    BRL, // Brazilian Real
    BSD, // Bahamian Dollar
    BWP, // Botswana Pula
    BZD, // Belize Dollar
    CAD, // Canadian Dollar
    CDF, // Congolese Franc
    CHF, // Swiss Franc
    CLP, // Chilean Peso
    CNY, // Chinese Renminbi Yuan
    COP, // Colombian Peso
    CRC, // Costa Rican Colón
    CVE, // Cape Verdean Escudo
    CZK, // Czech Koruna
    DJF, // Djiboutian Franc
    DKK, // Danish Krone
    DOP, // Dominican Peso
    DZD, // Algerian Dinar
    EEK, // Estonian Kroon
    EGP, // Egyptian Pound
    ETB, // Ethiopian Birr
    EUR, // Euro
    FJD, // Fijian Dollar
    FKP, // Falkland Islands Pound
    GBP, // British Pound
    GEL, // Georgian Lari
    GIP, // Gibraltar Pound
    GMD, // Gambian Dalasi
    GNF, // Guinean Franc
    GTQ, // Guatemalan Quetzal
    GYD, // Guyanese Dollar
    HKD, // Hong Kong Dollar
    HNL, // Honduran Lempira
    HRK, // Croatian Kuna
    HTG, // Haitian Gourde
    HUF, // Hungarian Forint
    IDR, // Indonesian Rupiah
    ILS, // Israeli New Sheqel
    INR, // Indian Rupee
    ISK, // Icelandic Króna
    JMD, // Jamaican Dollar
    JPY, // Japanese Yen
    KES, // Kenyan Shilling
    KGS, // Kyrgyzstani Som
    KHR, // Cambodian Riel
    KMF, // Comorian Franc
    KRW, // South Korean Won
    KYD, // Cayman Islands Dollar
    KZT, // Kazakhstani Tenge
    LAK, // Lao Kip
    LBP, // Lebanese Pound
    LKR, // Sri Lankan Rupee
    LRD, // Liberian Dollar
    LSL, // Lesotho Loti
    LTL, // Lithuanian Litas
    LVL, // Latvian Lats
    MAD, // Moroccan Dirham
    MDL, // Moldovan Leu
    MGA, // Malagasy Ariary
    MKD, // Macedonian Denar
    MNT, // Mongolian Tögrög
    MOP, // Macanese Pataca
    MRO, // Mauritanian Ouguiya
    MUR, // Mauritian Rupee
    MMK, // Myanmar Kyat
    MVR, // Maldivian Rufiyaa
    MWK, // Malawian Kwacha
    MXN, // Mexican Peso
    MYR, // Malaysian Ringgit
    MZN, // Mozambican Metical
    NAD, // Namibian Dollar
    NGN, // Nigerian Naira
    NIO, // Nicaraguan Córdoba
    NOK, // Norwegian Krone
    NPR, // Nepalese Rupee
    NZD, // New Zealand Dollar
    PAB, // Panamanian Balboa
    PEN, // Peruvian Nuevo Sol
    PGK, // Papua New Guinean Kina
    PHP, // Philippine Peso
    PKR, // Pakistani Rupee
    PLN, // Polish Złoty
    PYG, // Paraguayan Guaraní
    QAR, // Qatari Riyal
    RON, // Romanian Leu
    RSD, // Serbian Dinar
    RUB, // Russian Ruble
    RWF, // Rwandan Franc
    SAR, // Saudi Riyal
    SBD, // Solomon Islands Dollar
    SCR, // Seychellois Rupee
    SEK, // Swedish Krona
    SGD, // Singapore Dollar
    SHP, // Saint Helenian Pound
    SLL, // Sierra Leonean Leone
    SOS, // Somali Shilling
    SRD, // Surinamese Dollar
    STD, // São Tomé and Príncipe Dobra
    SVC, // Salvadoran Colón
    SZL, // Swazi Lilangeni
    THB, // Thai Baht
    TJS, // Tajikistani Somoni
    TOP, // Tongan Paʻanga
    TRY, // Turkish Lira
    TTD, // Trinidad and Tobago Dollar
    TWD, // New Taiwan Dollar
    TZS, // Tanzanian Shilling
    UAH, // Ukrainian Hryvnia
    UGX, // Ugandan Shilling
    USD, // United States Dollar
    UYU, // Uruguayan Peso
    UZS, // Uzbekistani Som
    VEF, // Venezuelan Bolívar
    VND, // Vietnamese Đồng
    VUV, // Vanuatu Vatu
    WST, // Samoan Tala
    XAF, // Central African Cfa Franc
    XCD, // East Caribbean Dollar
    XOF, // West African Cfa Franc
    XPF, // Cfp Franc
    YER, // Yemeni Rial
    ZAR, // South African Rand
    ZMW, // Zambian Kwacha
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &format!("{:?}", self))
    }
}
