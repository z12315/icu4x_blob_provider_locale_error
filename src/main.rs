#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use icu::collator::{CollatorOptions, Collator, Strength};
    use icu::locid::{Locale, locale};
    use icu_provider::hello_world::HelloWorldFormatter;
    use icu_provider_blob::BlobDataProvider;

    const LOCALE: Locale = locale!("de_de"); // let's try some other language
    const DATA: &[u8] = std::include_bytes!("data");

    #[test]
    fn icu_compare_german() {
        fn compare(left: &str, right: &str) -> std::cmp::Ordering {
            let provider = BlobDataProvider::try_new_from_static_blob(DATA).unwrap();
        
            let options = CollatorOptions::new();
            let collator: Collator = Collator::try_new_with_buffer_provider(
                &provider,
                &LOCALE.into(),
                options,
            )
            .unwrap();
        
            collator.compare(left, right)
        }

        assert_eq!(compare("ä", "a"), Ordering::Equal);
    }

    #[test]
    fn icu_collator_example() {
        let provider = BlobDataProvider::try_new_from_static_blob(DATA).unwrap();

        let locale_es: Locale = locale!("es-u-co-trad");
        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Primary);
        let collator_es: Collator = Collator::try_new_with_buffer_provider(
            &provider,
            &locale_es.into(),
            options,
        )
        .unwrap();
        
        assert_eq!(collator_es.compare("pollo", "polvo"), Ordering::Greater);
    }

    #[test]
    fn icu_provider_blob_example() {
        let provider = BlobDataProvider::try_new_from_static_blob(DATA).unwrap();

        let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
            &provider,
            &locale!("la").into(),
        )
        .expect("locale exists");

        assert_eq!(formatter.format().to_string(), "Ave, munde");
    }
}

fn main() {

}