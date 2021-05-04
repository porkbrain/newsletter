use google_vision1::api::{TextAnnotation as GAnnotation, Word as GWord};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Annotation {
    pub text: String,
    pub words: Vec<Word>,
}

/// Since there are many words in each text, during serialization we rename each
/// attribute so that when we inpect the generated JSON, it's less cluttered.
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Default, Clone)]
pub struct Word {
    #[serde(rename = "w")]
    pub word: String,
    #[serde(rename = "tl")]
    pub top_left: Point,
    #[serde(rename = "br")]
    pub bottom_right: Point,
}

#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Default, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Annotation {
    pub fn from(annotation: GAnnotation) -> Option<Self> {
        let text = annotation.text?;
        let words = annotation
            .pages?
            .into_iter()
            .filter_map(|p| p.blocks)
            .flatten()
            .filter_map(|b| b.paragraphs)
            .flatten()
            .filter_map(|p| p.words)
            .flatten()
            .filter_map(Word::from)
            .collect();

        Some(Self { text, words })
    }
}

impl Word {
    pub fn from(word: GWord) -> Option<Self> {
        // finds the rectangle around the word
        let vertices = word.bounding_box?.vertices?;
        let top = vertices.iter().min_by(|a, b| a.y.cmp(&b.y))?.y?;
        let bottom = vertices.iter().max_by(|a, b| a.y.cmp(&b.y))?.y?;
        let left = vertices.iter().min_by(|a, b| a.x.cmp(&b.x))?.x?;
        let right = vertices.iter().max_by(|a, b| a.x.cmp(&b.x))?.x?;

        // and collects all the symbols of the word
        let text: String =
            word.symbols?.into_iter().filter_map(|s| s.text).collect();

        (!text.is_empty()).then(|| Self {
            word: text,
            top_left: Point { y: top, x: left },
            bottom_right: Point {
                y: bottom,
                x: right,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_filters_out_words_without_vertices_or_symbols() {
        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [{"x": 1}]
            },
            "symbols": [{"text": "h"}]
        }))
        .unwrap();
        assert_eq!(None, Word::from(word));

        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [{"x": 1, "y": 2}, {"x": 5, "y": 4}]
            },
            "symbols": []
        }))
        .unwrap();
        assert_eq!(None, Word::from(word));
    }

    #[test]
    fn it_constructs_word() {
        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [
                    {"x": 1, "y": 5},
                    {"x": 3, "y": 0},
                    {"x": 5, "y": 4}
                ]
            },
            "symbols": [{"text": "h"}, {"text": "w"}]
        }))
        .unwrap();

        assert_eq!(
            Some(Word {
                word: "hw".to_string(),
                top_left: Point { x: 1, y: 0 },
                bottom_right: Point { x: 5, y: 5 },
            }),
            Word::from(word)
        );
    }

    #[test]
    fn it_ignores_no_pages_or_text() {
        let annotation: GAnnotation = serde_json::from_value(json!({
            "text": "hw"
        }))
        .unwrap();
        assert_eq!(None, Annotation::from(annotation));

        let annotation: GAnnotation = serde_json::from_value(json!({
            "pages": []
        }))
        .unwrap();
        assert_eq!(None, Annotation::from(annotation));
    }

    #[test]
    fn it_constructs_text() {
        let gen_word = |t| {
            json!({
                "boundingBox": {
                    "vertices": [{"x": 0, "y": 0}]
                },
                "symbols": [{"text": t}]
            })
        };
        let annotation: GAnnotation = serde_json::from_value(json!({
            "text": "1 2 3 4 5 6 7 8",
            "pages": [
                {"blocks": [
                    {"paragraphs": [{"words": [gen_word("1"), gen_word("2")]}]},
                    {"paragraphs": [{"words": [gen_word("3"), gen_word("4")]}]}
                ]},
                {"blocks": [
                    {"paragraphs": [{"words": [gen_word("5"), gen_word("6")]}]},
                    {"paragraphs": [{"words": [gen_word("7"), gen_word("8")]}]}
                ]}
            ]
        }))
        .unwrap();

        let gen_word = |t: &str| Word {
            word: t.to_string(),
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 0, y: 0 },
        };
        assert_eq!(
            Some(Annotation {
                text: "1 2 3 4 5 6 7 8".to_string(),
                words: vec![
                    gen_word("1"),
                    gen_word("2"),
                    gen_word("3"),
                    gen_word("4"),
                    gen_word("5"),
                    gen_word("6"),
                    gen_word("7"),
                    gen_word("8"),
                ]
            }),
            Annotation::from(annotation)
        );
    }
}
