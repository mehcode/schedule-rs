use std::str::{self, FromStr};
use nom::*;

named!(parse_number <u32>,
    map_res!(
        map_res!(
            ws!(digit),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug)]
pub enum Field {
    All,
    Range { start: u32, end: u32 },
    Number(u32),
}

named!(parse_field <Field>,
    alt!(
        complete!(do_parse!(
            start: parse_number >>
            tag!("-")           >>
            end: parse_number   >>
            (Field::Range {
                start: start,
                end:   end,
            })
        )) |
        map!(ws!(tag!("*")), |_| { Field::All }) |
        map!(parse_number, |n| { Field::Number(n) })
    )
);

named!(pub parse <Vec<Field>>, do_parse!(
    fields: many1!(parse_field) >>
    eof!() >>
    (fields)
));
