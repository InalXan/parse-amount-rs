use actix_web::{HttpResponse, Responder};
use std::{num::ParseIntError, string::String};

fn col_money(input: &str) -> Result<(i32, String), ParseIntError> {
    let money: Vec<&str> = input.split_whitespace().collect();
    let amount = money[0].parse()?;
    let currency = money[1].to_string();
    return Ok((amount, currency));
}

pub async fn index() -> impl Responder {
    match col_money("360 euro") {
        Ok((amount, currency)) => {
            let format_html = format!("<html><head></head><body><table><caption><tr><th>Amount</th> <th>Currency</th></tr><tr><td> {} </td> <td> {} </td></tr></caption></table></body></html>", amount, currency);
            HttpResponse::Ok()
                .content_type("text/html")
                .body(format_html)
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid Input"),
    }
}
