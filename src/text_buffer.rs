use crate::doubly_linked_list::List;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(PartialEq)]
struct Span {
    //length: u32,
    newlines: u32,
    is_append: bool,
    start_index: usize,
    end_index: usize,
}

pub struct Buffer {
    file: PathBuf,
    original_text: Vec<u8>,
    append: Vec<u8>,
    spans: List<Span>,
}

impl Buffer {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let original_text = fs::read(path.as_ref()).map_err(|err| err.to_string())?;
        let newlines = Self::num_newlines(&original_text);
        let mut spans: List<Span> = List::new();

        spans.push_front(Span {
            newlines,
            is_append: false,
            start_index: 0,
            end_index: original_text.len() - 1,
        });

        Ok(Buffer {
            file: PathBuf::from(path.as_ref()),
            original_text,
            append: Vec::new(),
            spans,
        })
    }

    pub fn get_lines(&self, line_start: u32, line_end: u32) -> impl Iterator<Item = String> {
        let mut newline_count: u32 = 0;
        let mut line_buf: Vec<String> = Vec::new();

        let skip_spans = self
            .spans
            .iter()
            .take_while(|span| {
                if newline_count + span.newlines < line_start - 1 {
                    newline_count += span.newlines;
                    return true;
                }
                false
            })
            .count();

        let mut add_to_prev = false;
        for span in self.spans.iter().skip(skip_spans) {
            let char_span = self.get_character_span(span);
            let string = String::from_utf8(char_span.to_vec())
                .map_err(|e| e.to_string())
                .unwrap();
            let mut lines: Vec<String>;
            if newline_count < line_start - 1 {
                lines = string
                    .split('\n')
                    .skip((line_start - 1 - newline_count).try_into().unwrap())
                    .map(|s| s.to_string())
                    .collect();
            } else {
                lines = string.split('\n').map(|s| s.to_string()).collect();
            }
            // remove extra lines
            lines.truncate((line_end - newline_count) as usize);
            if add_to_prev {
                let final_index = line_buf.len() - 1;
                line_buf[final_index] = line_buf[line_buf.len() - 1].to_string() + &lines[0];
                lines.remove(0);
            }
            add_to_prev = lines.len() > 0 && lines[lines.len() - 1] != "";
            line_buf.append(&mut lines);
            newline_count += lines.len() as u32;

            if newline_count >= line_end {
                break;
            }
        }
        return line_buf.into_iter();
    }

    fn num_newlines(buf: &[u8]) -> u32 {
        buf.iter()
            .filter(|character| character.is_ascii() && char::from(**character) == '\n')
            .count()
            .try_into()
            .unwrap()
    }

    fn get_character_span(&self, span: &Span) -> &[u8] {
        if span.is_append {
            &self.append[span.start_index..=span.end_index]
        } else {
            &self.original_text[span.start_index..=span.end_index]
        }
    }
}
