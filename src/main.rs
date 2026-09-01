use evm_calldata::{decode, word_as_u128};
use std::io::Read;

fn render(input: &str) -> Result<String, String> {
    let d = decode(input).map_err(|e| e.to_string())?;
    let mut out = String::new();
    out.push_str(&format!("selector   {}\n", d.selector));
    out.push_str(&format!(
        "signature  {}\n",
        d.signature.unwrap_or("(not in the known table)")
    ));
    out.push_str(&format!("arg words  {}\n", d.words.len()));
    for (i, w) in d.words.iter().enumerate() {
        let hex: String = w.iter().map(|b| format!("{:02x}", b)).collect();
        let num = match word_as_u128(w) {
            Some(v) => v.to_string(),
            None => "> u128".to_string(),
        };
        out.push_str(&format!("  [{}] 0x{}  ({})\n", i, hex, num));
    }
    if !d.trailing.is_empty() {
        out.push_str(&format!("trailing   {} byte(s)\n", d.trailing.len()));
    }
    Ok(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input = if args.is_empty() {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).ok();
        s.trim().to_string()
    } else {
        args.join("")
    };
    match render(&input) {
        Ok(text) => print!("{}", text),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
