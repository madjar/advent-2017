use std::io::BufRead;
use std::collections::HashMap;
use nom::{alpha, digit, GetInput};
use std::str;

named!(name<&str>, map_res!(ws!(alpha), str::from_utf8));
named!(number<i64>, flat_map!(ws!(digit), parse_to!(i64)));

named!(line<&[u8], (&str, i64, Vec<&str>)>, tuple!(
    name,
    ws!(delimited!( tag!("("), number, tag!(")") )),
    map!(opt!(complete!(
        preceded!(
            ws!(tag!("->")),
            separated_nonempty_list!(tag!(","), name))))
         ,|o| o.unwrap_or_default())
));

named!(tree<Vec<(&str, i64, Vec<&str>)>>, many1!(line));


pub fn doit<B>(mut input: B)
where
    B: BufRead,
{
    let mut all_input = Vec::new();
    input.read_to_end(&mut all_input).unwrap();
    let iresult = tree(&all_input);
    assert_eq!(iresult.remaining_input().map(str::from_utf8), Some(Ok("")));

    let result = iresult.to_result().unwrap();

    let children: HashMap<&str, &Vec<&str>> =
        result.iter().map(|&(ref n, _, ref cs)| (*n, cs)).collect();

    let weights: HashMap<&str, i64> = result.iter().map(|&(ref n, w, _)| (*n, w)).collect();

    let mut parents: HashMap<&str, Option<&str>> = children
        .iter()
        .flat_map(|(n, cs)| cs.iter().map(move |c| (*c, Some(*n))))
        .collect();

    let root = children.keys().find(|&n| !parents.contains_key(n)).unwrap();
    parents.insert(root, None);
    println!("Day 7, part 1: {}", root);



    fn recur_weight_and_check_balanced(
        children: &HashMap<&str, &Vec<&str>>,
        weights: &HashMap<&str, i64>,
        n: &str,
    ) -> i64 {
        let children_weights: Vec<(&str, i64)> = children[n]
            .iter()
            .map(|&c| (c, recur_weight_and_check_balanced(children, weights, c)))
            .collect();

        let mut iter = children_weights.iter();
        if let Some((bad_n, bad_weight, good_weight)) = {
           iter.next().and_then(|&(first_n, first_w)|
                      match iter.filter(|&&(_, w)| w != first_w).map(|&x| x).collect::<Vec<(&str, i64)>>().as_slice()  {
                          &[] => None,
                          &[(bad_n, bad_w)] => Some((bad_n, bad_w, first_w)),
                          &[(_, good_w), ..] => Some((first_n, first_w, good_w))
                      }
            )
        } {
            let better_weight = weights[bad_n] - bad_weight + good_weight;
            println!("Day 7, part 2 (keep only first line): {}", better_weight);
        }

        //println!("{:?}", all_same(children_weights.iter().map(|n| n.1)));
        weights[n] + children_weights.iter().map(|n| n.1).sum::<i64>()
    };

    recur_weight_and_check_balanced(&children, &weights, root);
}
