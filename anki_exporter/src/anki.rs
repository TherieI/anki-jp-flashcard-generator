use mecab::Tagger;
use std::rc::Rc;

const KA_HI_OFFSET: u32 = 0x60;
fn katakana_to_hiragana(katakana: &str) -> String {
    let mut hiragana = String::with_capacity(katakana.len());
    for c in katakana.chars() {
        if 0x30A1 <= c as u32 && c as u32 <= 0x30F6 {
            hiragana.push(char::from_u32(c as u32 - KA_HI_OFFSET).unwrap());
        } else {
            hiragana.push(c);
        }
    }
    hiragana
}

#[derive(Debug)]
pub struct Card {
    front: Rc<str>,
    back: Rc<str>,
}

#[derive(Debug, Default)]
pub struct CardBuilder {
    tango: String,
    example_sentence: String,
    english: String,
}

impl CardBuilder {
    /// Must have vocab
    pub fn vocab(mut self, tango: &str) -> CardBuilder {
        self.tango.push_str(tango);
        self
    }

    /// Example sentence optional
    pub fn example(mut self, desc: &str) -> CardBuilder {
        self.example_sentence.push_str(desc);
        self
    }

    /// Must have translation
    pub fn translation(mut self, english: &str) -> CardBuilder {
        self.english.push_str(english);
        self
    }

    pub fn construct(self) -> Option<Card> {
        if self.tango.is_empty() || self.english.is_empty() {
            None
        } else {
            let tagger = Tagger::new("");

            let mecab_res = tagger.parse_str(self.tango.as_str());
            let katakana_reading = mecab_res.rsplit(",").nth(1).unwrap();

            let front = String::from(format!("「{}」<br>{}", self.tango, self.example_sentence));
            let back = String::from(format!("<ruby>{}<rt>{}</rt></ruby><br>{}", self.tango, katakana_to_hiragana(katakana_reading), self.english));
            Some(Card {
                front: front.into(),
                back: back.into(),
            })
        }
    }
}

impl Card {
    pub fn new() -> CardBuilder {
        CardBuilder::default()
    }

    pub fn format_anki(&self) -> String {
        String::from(format!("{};{}", self.front, self.back))
    }
}

#[cfg(test)]
mod tests {
    use crate::anki::{katakana_to_hiragana, Card};

    #[test]
    fn kata_hira_conversion() {
        println!("{}", katakana_to_hiragana("ダイッスキダヨ~"));
    }

    #[test]
    fn create_tango() {
        let card = Card::new()
            .vocab("単語")
            .example("彼は、新しい単語をたくさん覚えました。")
            .translation("Vocabulary")
            .construct()
            .unwrap();
        println!("{:?}", card)
    }

    #[test]
    fn create_verb() {
        let card = Card::new()
            .vocab("押し流す")
            .example("何々")
            .translation("To wash over")
            .construct()
            .unwrap();
        println!("{:?}", card)
    }
}
