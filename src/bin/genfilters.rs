use dialetto::HP;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use xorf::Filter;

struct LangConfig {
    language_name: &'static str,
    filename: &'static str,
    threshold: i32,
}

impl LangConfig {
    const fn new(language_name: &'static str, filename: &'static str, threshold: i32) -> Self {
        LangConfig {
            language_name,
            filename,
            threshold,
        }
    }
}

const LANGS: [LangConfig; 61] = [
    LangConfig::new("English", "2018/en/en_full.txt", 200),
    LangConfig::new("Afrikaans", "2018/af/af_full.txt", 200),
    LangConfig::new("Arabic", "2018/ar/ar_full.txt", 200),
    LangConfig::new("Bulgarian", "2018/bg/bg_full.txt", 200),
    LangConfig::new("Bengali", "2018/bn/bn_full.txt", 200),
    LangConfig::new("Breton", "2018/br/br_full.txt", 200),
    LangConfig::new("Bosnian", "2018/bs/bs_full.txt", 200),
    LangConfig::new("Catalan", "2018/ca/ca_full.txt", 200),
    LangConfig::new("Czech", "2018/cs/cs_full.txt", 200),
    LangConfig::new("Danish", "2018/da/da_full.txt", 200),
    LangConfig::new("German", "2018/de/de_full.txt", 200),
    LangConfig::new("Greek", "2018/el/el_full.txt", 200),
    // en here
    LangConfig::new("Esperanto", "2018/eo/eo_full.txt", 200),
    LangConfig::new("Spanish", "2018/es/es_full.txt", 200),
    LangConfig::new("Estonian", "2018/et/et_full.txt", 200),
    LangConfig::new("Basque", "2018/eu/eu_full.txt", 200),
    LangConfig::new("Persian", "2018/fa/fa_full.txt", 200),
    LangConfig::new("Finnish", "2018/fi/fi_full.txt", 200),
    LangConfig::new("French", "2018/fr/fr_full.txt", 200),
    LangConfig::new("Galacian", "2018/gl/gl_full.txt", 200),
    LangConfig::new("Hebrew", "2018/he/he_full.txt", 200),
    LangConfig::new("Hindi", "2018/hi/hi_full.txt", 200),
    LangConfig::new("Croatian", "2018/hr/hr_full.txt", 200),
    LangConfig::new("Hungarian", "2018/hu/hu_full.txt", 200),
    LangConfig::new("Armenian", "2018/hy/hy_full.txt", 200),
    LangConfig::new("Indonesian", "2018/id/id_full.txt", 200),
    LangConfig::new("Icelandic", "2018/is/is_full.txt", 200),
    LangConfig::new("Italian", "2018/it/it_full.txt", 200),
    LangConfig::new("Japanese", "2018/ja/ja_full.txt", 200),
    LangConfig::new("Georgian", "2018/ka/ka_full.txt", 200),
    LangConfig::new("Kazakh", "2018/kk/kk_full.txt", 200),
    LangConfig::new("Korean", "2018/ko/ko_full.txt", 200),
    LangConfig::new("Lithuanian", "2018/lt/lt_full.txt", 200),
    LangConfig::new("Latvian", "2018/lv/lv_full.txt", 200),
    LangConfig::new("Macedonian", "2018/mk/mk_full.txt", 200),
    LangConfig::new("Malayalam", "2018/ml/ml_full.txt", 200),
    LangConfig::new("Malay", "2018/ms/ms_full.txt", 200),
    LangConfig::new("Dutch", "2018/nl/nl_full.txt", 200),
    LangConfig::new("Norwegian", "2018/no/no_full.txt", 200),
    LangConfig::new("Polish", "2018/pl/pl_full.txt", 200),
    LangConfig::new("Portuguese", "2018/pt/pt_full.txt", 200),
    LangConfig::new("Portuguese (Brazil)", "2018/pt_br/pt_br_full.txt", 200),
    LangConfig::new("Romanian", "2018/ro/ro_full.txt", 200),
    LangConfig::new("Russian", "2018/ru/ru_full.txt", 200),
    LangConfig::new("Sinhala", "2018/si/si_full.txt", 200),
    LangConfig::new("Slovak", "2018/sk/sk_full.txt", 200),
    LangConfig::new("Slovenian", "2018/sl/sl_full.txt", 200),
    LangConfig::new("Albanian", "2018/sq/sq_full.txt", 200),
    LangConfig::new("Serbian", "2018/sr/sr_full.txt", 200),
    LangConfig::new("Swedish", "2018/sv/sv_full.txt", 200),
    LangConfig::new("Tamil", "2018/ta/ta_full.txt", 200),
    LangConfig::new("Telugu", "2018/te/te_full.txt", 200),
    LangConfig::new("Thai", "2018/th/th_50k.txt", 200),
    LangConfig::new("Tagalog", "2018/tl/tl_full.txt", 200),
    LangConfig::new("Turkish", "2018/tr/tr_full.txt", 200),
    LangConfig::new("Ukrainian", "2018/uk/uk_full.txt", 200),
    LangConfig::new("Urdu", "2018/ur/ur_full.txt", 200),
    LangConfig::new("Vietnamese", "2018/vi/vi_full.txt", 200),
    LangConfig::new("Vietnamese", "2018/vi/vi_full.txt", 200),
    LangConfig::new("Chinese", "2018/zh_cn/zh_cn_full.txt", 200),
    LangConfig::new("Chinese (Taiwan)", "2018/zh_tw/zh_tw_full.txt", 200),
];


fn main() -> Result<(), Box<dyn Error>> {

    for cf in LANGS {
        println!("Generating data structure for {}....", cf.language_name);
        generate(cf.language_name, cf.filename, cf.threshold)?;
    }

    test_en();
    Ok(())
}

fn test_en() {
    let filter = dialetto::decode_from_file("en_full.txt.bin").unwrap();
    word_check("this", &filter);
    word_check("bonjour", &filter); // Really need to clean this wordlist
    word_check("alabaster", &filter);
    word_check("catastrophe", &filter);
    word_check("aklw4relkhgh", &filter);
    word_check("a", &filter);
    word_check("ab", &filter);
    word_check("aba", &filter);
    word_check("abab", &filter);
    word_check("ababa", &filter);
    word_check("ababt", &filter);
    word_check("ababababab", &filter);
    word_check("caa", &filter);
    word_check("cac", &filter);
    word_check("xx", &filter);
}

fn generate(language_name: &str, filename: &str, threshold: i32) -> Result<HP, Box<dyn Error>> {
    let filter = dialetto::make_filter(filename, Some(threshold))?;
    let outname = format!(
        "{}.bin",
        Path::new(filename).file_name().unwrap().to_str().unwrap()
    );
    dialetto::encode_to_file(&filter, outname)?;
    Ok(filter)
}

fn word_check(word: &str, filter: &HP) {
    println!(
        "Is \"{}\" found? {:?}",
        word,
        &filter.contains(&word.to_string())
    );
}
