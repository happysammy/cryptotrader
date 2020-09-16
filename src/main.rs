use cryptotrader::models::*;
use huobi;
fn main() {
    // let a: Pair = Pair::new("btc", "usdt");
    // let b: OrderType = OrderType::Limit;
    // println!("{}", Pair::from_string("btc_usdt"));
    // println!("{}", b);
    // let a: String = okex::client::get_timestamp();
    // let b = okex::client::Client::new("key".into(), "secret".into(), "okex321".into());
    // println!("{}", a);
    // let client = huobi::Client::new(
    //     "api-key",
    //     "secret-key",
    // );

    // match client.accounts() {
    //     Ok(accounts) => println!(
    //         "accounts:\n{}",
    //         accounts
    //             .into_iter()
    //             .map(|account| format!(
    //                 "{}: {} - {}",
    //                 account.account_id, account.state, account.account_type
    //             ))
    //             .collect::<Vec<String>>()
    //             .join("\n")
    //     ),
    //     Err(why) => println!("error information: {}", why),
    // }

    
    
    // let client = huobi::Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

    // match client.common_symbols() {
    //     Ok(pairs) => println!(
    //         "symbols: {}",
    //         pairs
    //             .into_iter()
    //             .map(|pair| pair.symbol)
    //             .collect::<Vec<String>>()
    //             .join(", ")
    //     ),
    //     Err(why) => println!("error: {}", why),
    // }

    // println!("common_timestamp: {:?}", client.common_timestamp());
    // let path = "/tmp/dat";
    // match read_file(path) {
    //     Ok(file) => { println!("{}", file)}
    //     Err(e) => { println!("{} {}", path, e)}
    // }
    // let bar = foo(60)?;
    // assert_eq!("bar", bar);
    // Ok(())
    
      
}

// #[derive(Debug)]
// enum Error {
//     OptionError(String),
// }

// impl std::error::Error for Error {}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Error::OptionError(ref e) => e.fmt(f),
//         }
//     }
// }


// pub type Result<I> = std::result::Result<I, Error>;

// fn foo(index: i32) -> Option<String> {
//     if index > 60 {
//         return Some("bar".to_string());
//     }
//     None
// }

// fn read_file(path: &str) -> Result<String, std::io::Error> {
//     std::fs::read_to_string(path)
// }

// /// 转换为utf8内容
// fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
//     std::str::from_utf8(v)
// }

// /// 转化为u32数字
// fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
//     v.parse::<u32>()
// }