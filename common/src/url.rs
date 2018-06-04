use nom::IResult;
use failure::Error;
use nom;
use nom::digit;
use filter::{Filter, FilterArg, SizeUnit};
use std::str::{self, FromStr};

fn url(input: &str) -> IResult<&str, FilterArg> {
    let mut result = String::new();
    let mut remaining = input;
    let mut stack = 0;
    while let Some(n) = remaining.find(|c| ",()".contains(c)) {
        result.push_str(&remaining[..n]);
        remaining = &remaining[n..];
        match remaining.chars().next().unwrap() {
            ',' if stack == 0 => return IResult::Done(remaining, FilterArg::String(result)),
            ',' => { result.push_str(","); },
            '(' => { stack += 1; result.push_str("("); },
            ')' if stack > 0 => { stack -= 1; result.push_str(")"); },
            ')' if result.len() == 0 => return IResult::Error(error_code!(nom::ErrorKind::Complete)),
            ')' => return IResult::Done(remaining, FilterArg::String(result)),
            _ => {}
        };
        remaining = &remaining[1..];
    }
    IResult::Error(error_code!(nom::ErrorKind::Complete))
}

fn url_or_filter(input: &str) -> IResult<&str, FilterArg> {
    if input.find("(").unwrap_or(input.len()) < input.find(":").unwrap_or(input.len()) {
        filter(input).map(|f| FilterArg::Img(f))
    } else {
        url(input)
    }
}

fn parse_int_arg(input: &str) -> IResult<&str, isize> {
    let mut remaining = input;
    let mult = if input.starts_with("-") {
        remaining = &remaining[1..];
        -1
    } else { 1 };
    digit(remaining).map(|digits| digits.parse::<isize>().unwrap() * mult)
}

// Source: https://github.com/Geal/nom/blob/master/tests/float.rs#L28
// {
named!(unsigned_float(&str) -> f32, map_res!(
  recognize!(
    alt!(
      delimited!(digit, tag!("."), opt!(digit)) |
      delimited!(opt!(digit), tag!("."), digit)
    )
  ),
  FromStr::from_str
));

named!(float(&str) -> f32, map!(
  pair!(
    opt!(alt!(tag!("+") | tag!("-"))),
    unsigned_float
  ),
  |(sign, value): (Option<&str>, f32)| {
    sign.and_then(|s| if s.starts_with('-') { Some(-1f32) } else { None }).unwrap_or(1f32) * value
  }
));
// }

named!(unit(&str) -> SizeUnit, map_res!(
    alt!(tag!("px") | tag!("hcm") | tag!("vcm") | tag!("hin") | tag!("vin") | tag!("w") | tag!("h") | tag!("")),
    FromStr::from_str
));

named!(filter_arg(&str) -> FilterArg,
    alt_complete!(
		do_parse!(f: call!(float) >> unit: call!(unit) >> (FilterArg::Float(f, unit))) |
        do_parse!(i: call!(parse_int_arg) >> unit: call!(unit) >> (FilterArg::Int(i, unit))) |
        call!(url_or_filter)
    )
);

named!(one_filter(&str) -> Filter, do_parse! (
    name: take_until!("(") >>
    tag!("(") >>
    args: separated_list_complete!(tag!(","), filter_arg) >>
    tag!(")") >>
    ( Filter { name: name.to_owned(), args: args } )
));

named!(pub filter(&str) -> Filter, do_parse! (
    first: call!(one_filter) >>
    next: many0!(complete!(preceded!(tag_s!(":"), call!(one_filter)))) >>
    opt!(complete!(pair!(tag!("/"), many1!(none_of!("/"))))) >>
    ({
      let mut current = first;
      for mut f in next { 
        f.args.insert(0, FilterArg::Img(current));
        current = f;
      }
      current
    })
));

pub fn parse(input: &str) -> Result<Filter, Error> {
    match filter(input) {
        IResult::Done("", filter) => Ok(filter),
        IResult::Done(remaining, _) => bail!("Url parse error. Remaining data: {}", remaining),
        IResult::Incomplete(_) => bail!("Incomplete url."),
        IResult::Error(e) => bail!("Url parse error: {:?}.", e),
    }
}

#[test]
fn test_simple_filter() {
    assert_eq!(filter("download(s3:2666/img.jpg)"), IResult::Done("", Filter {
        name: "download".to_owned(),
        args: vec![FilterArg::Url("s3:2666/img.jpg".to_owned())]
    }))
}

#[test]
fn test_nested_filter() {
    assert_eq!(filter("resize(download(s3:2666/img.jpg),100,200)"), IResult::Done("", Filter {
        name: "resize".to_owned(),
        args: vec![
            FilterArg::Img(Filter{ name: "download".to_owned(), args: vec![FilterArg::Url("s3:2666/img.jpg".to_owned())] }),
            FilterArg::Int(100),
            FilterArg::Int(200)
        ]
    }))
}
