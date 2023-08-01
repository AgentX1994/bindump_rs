use std::fmt::Write;
use std::io;

use nom;
use thiserror::Error;

use super::parse::Input;

pub type BinDumpResult<T> = Result<T, BinDumpError>;

#[derive(Error, Debug)]
pub enum BinDumpError {
    #[error("IO Error: {error}")]
    IoError {
        #[from]
        error: io::Error,
    },
    #[error("Unknown Magic Error: {magic:?}")]
    UnknownMagic { magic: Vec<u8> },
    #[error("Parse Error: {error}")]
    ParseError { error: String },
}

impl<'a> From<nom::error::VerboseError<Input<'a>>> for BinDumpError {
    fn from(error: nom::error::VerboseError<Input<'a>>) -> Self {
        // let error = nom::error::VerboseError {
        //     errors: error
        //         .errors
        //         .iter()
        //         .map(|(i, kind)| (i.to_vec(), kind.clone()))
        //         .collect(),
        // };
        let mut error_string = String::new();
        writeln!(&mut error_string, "Parsing failed:").expect("Couldn't write to string");
        for (input, err) in error.errors {
            writeln!(&mut error_string, "{:?} at:", err).expect("Couldn't write to string");
            writeln!(&mut error_string, "{:02x?}", &input[..20]).expect("Couldn't write to string");
        }
        Self::ParseError {
            error: error_string,
        }
    }
}
/// Useful functions to calculate the offset between slices and show a hexdump of a slice
trait Offset {
    /// Offset between the first byte of self and the first byte of the argument
    fn offset(&self, second: &Self) -> usize;
}

impl<'a> Offset for &'a [u8] {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

// pub fn convert_error(input: parse::Input, e: nom::error::VerboseError<parse::Input>) -> String {
//     use std::fmt::Write;

//     let mut result = String::new();

//     for (i, (substring, kind)) in e.errors.iter().enumerate() {
//         let offset = input.offset(&*substring);

//         if input.is_empty() {
//             match kind {
//                 nom::error::VerboseErrorKind::Char(c) => {
//                     write!(&mut result, "{}: expected '{}', got empty input\n\n", i, c)
//                 }
//                 nom::error::VerboseErrorKind::Context(s) => {
//                     write!(&mut result, "{}: in {}, got empty input\n\n", i, s)
//                 }
//                 nom::error::VerboseErrorKind::Nom(e) => {
//                     write!(&mut result, "{}: in {:?}, got empty input\n\n", i, e)
//                 }
//             }
//         } else {
//             let prefix = &input.as_bytes()[..offset];

//             // Count the number of newlines in the first `offset` bytes of input
//             let line_number = prefix.iter().filter(|&&b| b == b'\n').count() + 1;

//             // Find the line that includes the subslice:
//             // Find the *last* newline before the substring starts
//             let line_begin = prefix
//                 .iter()
//                 .rev()
//                 .position(|&b| b == b'\n')
//                 .map(|pos| offset - pos)
//                 .unwrap_or(0);

//             // Find the full line after that newline
//             let line = input[line_begin..]
//                 .lines()
//                 .next()
//                 .unwrap_or(&input[line_begin..])
//                 .trim_end();

//             // The (1-indexed) column number is the offset of our substring into that line
//             let column_number = line.offset(substring) + 1;

//             match kind {
//                 VerboseErrorKind::Char(c) => {
//                     if let Some(actual) = substring.chars().next() {
//                         write!(
//                             &mut result,
//                             "{i}: at line {line_number}:\n\
//                  {line}\n\
//                  {caret:>column$}\n\
//                  expected '{expected}', found {actual}\n\n",
//                             i = i,
//                             line_number = line_number,
//                             line = line,
//                             caret = '^',
//                             column = column_number,
//                             expected = c,
//                             actual = actual,
//                         )
//                     } else {
//                         write!(
//                             &mut result,
//                             "{i}: at line {line_number}:\n\
//                  {line}\n\
//                  {caret:>column$}\n\
//                  expected '{expected}', got end of input\n\n",
//                             i = i,
//                             line_number = line_number,
//                             line = line,
//                             caret = '^',
//                             column = column_number,
//                             expected = c,
//                         )
//                     }
//                 }
//                 VerboseErrorKind::Context(s) => write!(
//                     &mut result,
//                     "{i}: at line {line_number}, in {context}:\n\
//                {line}\n\
//                {caret:>column$}\n\n",
//                     i = i,
//                     line_number = line_number,
//                     context = s,
//                     line = line,
//                     caret = '^',
//                     column = column_number,
//                 ),
//                 VerboseErrorKind::Nom(e) => write!(
//                     &mut result,
//                     "{i}: at line {line_number}, in {nom_err:?}:\n\
//                {line}\n\
//                {caret:>column$}\n\n",
//                     i = i,
//                     line_number = line_number,
//                     nom_err = e,
//                     line = line,
//                     caret = '^',
//                     column = column_number,
//                 ),
//             }
//         }
//         // Because `write!` to a `String` is infallible, this `unwrap` is fine.
//         .unwrap();
//     }

//     result
// }
