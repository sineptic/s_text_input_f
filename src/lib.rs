#![warn(clippy::doc_markdown)]

use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, num::ParseIntError};

pub type Blocks = Vec<Block>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Block {
    /// # Response
    /// Vec of starting positions(stating from 0; stringified) of each elem in the same order as it's displayed for user on
    /// screan
    Order(Vec<String>),
    /// # Response
    /// Vec of {`answer_positions`(starting from 0)}.`to_string()`
    AnyOf(Vec<String>),
    /// # Response
    /// vec!\[{`answer_position`(starting from 0)}.`to_string()`]
    OneOf(Vec<String>),
    /// # Response
    /// Vec of all placeholder answers in same order.
    Paragraph(Paragraph),
}

pub type Paragraph = Vec<ParagraphItem>;
#[derive(Debug, Serialize, Deserialize)]
pub enum ParagraphItem {
    Text(String),
    Placeholder,
}

pub type Response = Vec<ResponseItem>;
pub type ResponseItem = Vec<String>;
pub fn eq_response(a: &Response, b: &Response, trim: bool, case_insensetive: bool) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b)
        .all(|(a, b)| eq_response_item(a, b, trim, case_insensetive))
}
fn eq_response_item(
    a: &ResponseItem,
    b: &ResponseItem,
    trim: bool,
    case_insensetive: bool,
) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(a, b)| {
        if trim {
            let (a, b) = (a.trim(), b.trim());
            if case_insensetive {
                let (a, b) = (a.to_lowercase(), b.to_lowercase());
                a == b
            } else {
                a == b
            }
        } else {
            #[allow(clippy::collapsible_if)]
            if case_insensetive {
                let (a, b) = (a.to_lowercase(), b.to_lowercase());
                a == b
            } else {
                a == b
            }
        }
    })
}

/// # Returns
/// Vec of (`initial_pos`, `current_pos`)
pub fn response_as_order(response: ResponseItem) -> Result<Vec<(usize, usize)>, ParseIntError> {
    response
        .into_iter()
        .enumerate()
        .map(|(i, a)| -> Result<_, ParseIntError> { Ok((a.parse()?, i)) })
        .collect::<Result<Vec<_>, _>>()
        .map(|mut vec| {
            vec.sort_unstable();
            vec
        })
}
/// # Returns
/// `BTreeSet` of selected items
pub fn response_as_any_of(response: ResponseItem) -> Result<BTreeSet<usize>, ParseIntError> {
    response.into_iter().map(|s| s.parse()).collect()
}
/// # Errors
/// - `None` if there not 1 element
/// - `Some(ParseIntError)` if there not a number
pub fn response_as_one_of(response: ResponseItem) -> Option<Result<usize, ParseIntError>> {
    let [val]: [String; 1] = response.try_into().ok()?;
    Some(val.parse())
}
pub fn response_as_placeholders(response: ResponseItem) -> Vec<String> {
    response
}