use std::io::BufRead;
use nom::{alpha, digit, GetInput};
use std::str;

named!(name<&str>, map_res!(ws!(alpha), str::from_utf8));
named!(number<i64>, flat_map!(ws!(digit), parse_to!(i64)));

#[derive(Debug)]
struct Line<'a> {
    register: &'a str,
    inst: &'a str,
    value: i64,
    comp_register: &'a str,
    comp_operation: &'a str,
    comp_value: i64,
}

named!(line<Line>, do_parse!(
    register: name >>
        inst: name >>
        value: number >>
        ws!(tag!("if")) >>
        comp_register: name >>
        comp_operation: name >> // not right
        comp_value: number >>
        (Line {register, inst, value, comp_register, comp_operation, comp_value})
));


named!(instructions<Vec<Line>>, many1!(line));


pub fn doit<B>(mut input: B)
    where
    B: BufRead {

    let mut all_input = Vec::new();
    input.read_to_end(&mut all_input).unwrap();
    let iresult = line(&all_input);
    //assert_eq!(iresult.remaining_input().map(str::from_utf8), Some(Ok("")));

    let result = iresult.to_result().unwrap();

    println!("{:?}", result);

}
