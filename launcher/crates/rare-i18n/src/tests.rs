use super::*;
use std::collections::BTreeSet;

fn placeholders(s: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = s;
    while let Some(i) = rest.find('{') {
        let after = &rest[i + 1..];
        match after.find('}') {
            Some(j) => {
                out.insert(after[..j].to_string());
                rest = &after[j + 1..];
            }
            None => break,
        }
    }
    out
}

/// Every key is in every language.
#[test]
fn every_language_has_every_key() {
    let mut missing = Vec::new();
    for l in Lang::ALL.into_iter().skip(1) {
        for a in areas(l) {
            for k in a.keys {
                if !a.given.contains(k) {
                    missing.push(format!("{}: {}.{}", l.code(), a.name, k));
                }
            }
        }
    }
    assert!(missing.is_empty(), "{} keys missing:\n{}", missing.len(), missing.join("\n"));
}

/// Translations keep English's placeholders, plural forms and are never empty.
#[test]
fn placeholders_and_plurals_match_english() {
    let en = areas(Lang::En);
    let mut bad = Vec::new();
    for l in Lang::ALL.into_iter().skip(1) {
        for (a, e) in areas(l).iter().zip(&en) {
            for (i, k) in a.keys.iter().enumerate() {
                if !a.given.contains(k) {
                    continue;
                }
                let (v, ev) = (a.values[i], e.values[i]);
                let at = format!("{}: {}.{}", l.code(), a.name, k);
                if v.trim().is_empty() && !ev.trim().is_empty() {
                    bad.push(format!("{at}: empty"));
                }
                if placeholders(v) != placeholders(ev) {
                    bad.push(format!("{at}: placeholders {:?}, English has {:?}", placeholders(v), placeholders(ev)));
                }
                let forms = v.split('|').count();
                if ev.contains('|') {
                    if forms != l.plural_forms() {
                        bad.push(format!("{at}: {forms} plural forms, {} takes {}", l.code(), l.plural_forms()));
                    }
                } else if forms != 1 {
                    bad.push(format!("{at}: has '|' but English is not a plural"));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// English plurals have exactly one|other.
#[test]
fn english_plurals_have_two_forms() {
    for a in areas(Lang::En) {
        for (k, v) in a.keys.iter().zip(&a.values) {
            if v.contains('|') {
                assert_eq!(v.split('|').count(), 2, "{}.{k}", a.name);
            }
        }
    }
}

/// No key is given twice in English (the struct would not compile) and every area has a name.
#[test]
fn areas_list() {
    let en = areas(Lang::En);
    let names: BTreeSet<_> = en.iter().map(|a| a.name).collect();
    assert_eq!(names.len(), en.len());
}

#[test]
fn codes() {
    assert_eq!(Lang::from_code("pt_BR.UTF-8"), Some(Lang::PtBr));
    assert_eq!(Lang::from_code("pt-PT"), Some(Lang::PtBr));
    assert_eq!(Lang::from_code("zh_CN.utf8"), Some(Lang::Zh));
    assert_eq!(Lang::from_code("zh-Hans"), Some(Lang::Zh));
    assert_eq!(Lang::from_code("ja_JP.UTF-8"), Some(Lang::Ja));
    assert_eq!(Lang::from_code("ru"), Some(Lang::Ru));
    assert_eq!(Lang::from_code("es_MX"), Some(Lang::Es));
    assert_eq!(Lang::from_code("ko_KR"), Some(Lang::Ko));
    assert_eq!(Lang::from_code("en_US.UTF-8"), Some(Lang::En));
    assert_eq!(Lang::from_code("C.UTF-8"), None);
    assert_eq!(Lang::from_code("POSIX"), None);
    assert_eq!(Lang::from_code("de_DE"), None);
    assert_eq!(Lang::from_code(""), None);
    for l in Lang::ALL {
        assert_eq!(Lang::from_code(l.code()), Some(l));
        assert_eq!(Lang::from_code(l.posix()), Some(l));
    }
}

#[test]
fn plural_rules() {
    let ru = |n: f64| Lang::Ru.plural_index(n);
    assert_eq!([1.0, 21.0, 101.0].map(ru), [0, 0, 0]);
    assert_eq!([2.0, 3.0, 4.0, 22.0, 104.0].map(ru), [1, 1, 1, 1, 1]);
    assert_eq!([0.0, 5.0, 11.0, 12.0, 14.0, 25.0, 111.0].map(ru), [2, 2, 2, 2, 2, 2, 2]);
    assert_eq!(Lang::En.plural_index(1.0), 0);
    assert_eq!(Lang::En.plural_index(0.0), 1);
    assert_eq!(Lang::En.plural_index(2.0), 1);
    assert_eq!(Lang::PtBr.plural_index(0.0), 1);
    assert_eq!(Lang::PtBr.plural_index(1.0), 0);
    assert_eq!(Lang::PtBr.plural_index(2.0), 1);
    assert_eq!(Lang::Ja.plural_index(1.0), 0);
    assert_eq!(plural_in(Lang::En, "{n} game|{n} games", 1.0), "{n} game");
    assert_eq!(plural_in(Lang::En, "{n} game|{n} games", 3.0), "{n} games");
    assert_eq!(plural_in(Lang::Ru, "{n} игра|{n} игры|{n} игр", 5.0), "{n} игр");
    assert_eq!(plural_in(Lang::Ru, "{n} игра|{n} игры|{n} игр", 22.0), "{n} игры");
    // A Russian key that fell back to English two forms.
    assert_eq!(plural_in(Lang::Ru, "{n} game|{n} games", 5.0), "{n} games");
    assert_eq!(plural_in(Lang::Ja, "{n} 本のゲーム", 5.0), "{n} 本のゲーム");
}

#[test]
fn fill_and_macros() {
    assert_eq!(fill("{a} and {b}", &[("a", &1), ("b", &"x")]), "1 and x");
    assert_eq!(fill("{unknown} {", &[]), "{unknown} {");
    assert_eq!(trf!("Hi {name}, {n} new", name = "Ana", n = 3), "Hi Ana, 3 new");
    assert_eq!(trn!("{n} game|{n} games", 1usize), "1 game");
    assert_eq!(trn!("{n} game|{n} games in {store}", 2u32, store = "Steam"), "2 games in Steam");
    assert_eq!(trn!("{n} game|{n} games", 1200, n = "1,200"), "1,200 games");
}

#[test]
fn language_switch() {
    assert_eq!(lang(), Lang::En);
    let g = generation();
    set(Lang::Ja);
    assert_eq!(lang(), Lang::Ja);
    assert!(generation() > g);
    assert_eq!(tr().common.ok as *const str, tr_in(Lang::Ja).common.ok as *const str);
    set(Lang::En);
    assert_eq!(tr().common.ok, "OK");
}
