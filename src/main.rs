use clap::Parser;
use textwrap::fill;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message you want the cow to say
    message: String,
}

fn main() {
    let args = Args::parse();

    let cow = r#"     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||"#;

    let wrapped_message = fill(&args.message, 40);
    let max_line_length = wrapped_message.lines().map(|line| line.len()).max().unwrap_or(0);

    if wrapped_message.lines().count() == 1 {
        // Single line message
        println!(" {}", "-".repeat(max_line_length + 2));
        println!("| {} |", wrapped_message);
        println!(" {}", "-".repeat(max_line_length + 2));
    } else {
        // Multiple lines message
        println!(" {}", "_".repeat(max_line_length + 2));

        for (i, line) in wrapped_message.lines().enumerate() {
            if i == 0 {
                println!("/ {:<1$} \\", line, max_line_length);
            } else if i == wrapped_message.lines().count() - 1 {
                println!("\\ {:<1$} /", line, max_line_length);
            } else {
                println!("| {:<1$} |", line, max_line_length);
            }
        }

        println!(" {}", "-".repeat(max_line_length + 2));
    }
    print!("{}", cow);
}
