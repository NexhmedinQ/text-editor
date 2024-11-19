use core::str;
use std::{
    fs::{self}, path::{Path, PathBuf}
};

use crate::doubly_linked_list::List;

#[derive(PartialEq, Clone, Copy)]
struct Span {
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

    pub fn get_lines(&self, line_start: u32, line_end: u32) -> Vec<String> {
        let mut newline_count: u32 = 0;
        let mut line_buf: Vec<String> = Vec::new();

        let skip_spans = self
            .spans
            .iter()
            .take_while(|span| {
                if newline_count + span.newlines < line_start {
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
            if newline_count < line_start {
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
        return line_buf;
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

    pub fn insert(&mut self, line: u32, start_index: u32, text: &str) {
        let new_span_newlines = text.as_bytes().iter()
        .filter(|ch| **ch as char == '\n')
        .count();
        let new_span = Span { newlines: new_span_newlines as u32, is_append: true, start_index: self.append.len(), end_index: self.append.len() + text.len() };
        self.add_to_append(text);
        let mut curr_span = self.spans.head();
        let mut cur_line_num = 0;

        // find the span we're inserting at
        while curr_span.is_some() && (unsafe { *curr_span.unwrap() }).elem.newlines + cur_line_num < line {
            cur_line_num += (unsafe { *curr_span.unwrap() }).elem.newlines;
            curr_span = self.spans.next(curr_span.unwrap());
        }
        // get to the specific line since span might have multiple newlines
        let text = self.get_character_span(&unsafe { *curr_span.unwrap() }.elem);
 
        let mut line_start_index: u32 = 0;
        if line - cur_line_num > 0 {
            line_start_index = text.iter().enumerate()
            .filter(|(_, c)| **c as char == '\n')
            .map(|(index, _)| index)
            .nth((line - cur_line_num).try_into().unwrap())
            .unwrap()
            .try_into()
            .unwrap();
        }
        

        // get to the specific index of the line


        // while let Some(span) = curr_span.next() {
        //     if newlines + span.newlines >= line {
        //         let find_line = self
        //             .get_character_span(span)
        //             .iter()
        //             .enumerate()
        //             .filter(|(_, c)| **c as char == '\n')
        //             .map(|(index, _)| index)
        //             .nth((line - newlines).try_into().unwrap())
        //             .unwrap()
        //             .try_into()
        //             .unwrap();
        //         let span_length = span.end_index - span.start_index + 1;
        //         break;
        //     }
        //     newlines += span.newlines;
        // }
        // while let Some(span) = curr_span.next() {

        // }
        // edge cases: adding to beginning or the end of the document -> add two dummy nodes (one front one back)
        // what to do if the new span contains a newline.......
        //......................................................................................
        //....................................................................................
        //.......................................................................................
        //........................................................................................

        // while let Some(span) = curr_span {
        //     curr_span.next_back()
        //     // check if we're at the span of the line we want
        //     //  then check we're at the index??
        //     // need to check for boundary and of in the middle of the span
        //     // what about the case where the boundary is the beginning of the span??
        //     if newlines == line {

        //     }
        //     let mut line_index: u32;
        //     if newlines + span.newlines >= line {
        //         line_index = self
        //             .get_character_span(span)
        //             .iter()
        //             .enumerate()
        //             .filter(|(_, c)| **c as char == '\n')
        //             .map(|(index, _)| index)
        //             .nth((line - newlines).try_into().unwrap())
        //             .unwrap()
        //             .try_into()
        //             .unwrap();
        //         let insertion_point = line_index + start_index;
                
        //     }
        //     newlines += span.newlines;
        // }
        // thisisainsertion
        // insertion scenarios:
        // - inserting in the middle of a span -> will result in the span being split into 3 pieces essentially
        // insertion at span boundary -> add it between 2 spans and add the spans either side of the boundary one to the undo stack (maintaining pointer to old pos)

        // multiple spans can span a single line so we need to determine if we have a boudary or middle case
        // iterate through the spans and keep a previous pointer to the previous span
        // if the insertion is at index 6 and span 1 is 1-5 and span 2 is 6-10 we have a boudary case
    }

    fn add_to_append(&mut self, text: &str) {
        self.append.append(&mut text.to_string().into_bytes());
    }

    // fn find_span_index_with_line(&self, line: u32) -> u32 {
    //     let mut newline_count: u32 = 0;

    //     self
    //         .spans
    //         .iter()
    //         .take_while(|span| {
    //             if newline_count + span.newlines < line {
    //                 newline_count += span.newlines;
    //                 return true;
    //             }
    //             false
    //         })
    //         .count().try_into().unwrap()
    // }

    fn find_insertion_point(&self) {}

    pub fn redo(&mut self) {}

    pub fn undo(&mut self) {}
}
