extern crate nom;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
#[derive(Debug,PartialEq)]
pub struct Iostat<'a>{
    pub kernel_version: &'a str,
    pub hostname: &'a str,


}
#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn str_kernel_version(input: &str)->Result<&str, std::num::ParseIntError> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}
fn parse_iostat(input: &str)
     -> IResult<&str, Iostat> {

    let (input,err) = tag("")(input)?;
    let (input,(kernel_version,hostname)) = tuple((str_kernel_version,str_hostname))(input)?
    Ok((input,Iostat{kernel_version,hostname}))
}
fn main() {}
#[test]
fn parse_header(){
    assert_eq!(
        parse_iostat("Linux 5.4.0-99-generic (mm) 	02/12/22 	_x86_64_	(2 CPU)

        02/12/22 03:06:45
        avg-cpu:  %user   %nice %system %iowait  %steal   %idle"),
        Iostat{
            "Linux 5.4.0-99-generic",
            "mm"

        }
        
    )
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
