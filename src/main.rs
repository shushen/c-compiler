use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="A C Compiler written in Rust", long_about = None)]
struct Args {
    /// The source file to compile
    #[arg(required = true)]
    file: String,

    /// Run the lexer
    #[arg(short, long, default_value = "true")]
    lex: bool,

    /// Run the lexer and parser
    #[arg(short, long, default_value = "true")]
    parse: bool,

    /// Run the lexer, parser, and codegen
    #[arg(short, long, default_value = "true")]
    codegen: bool,
}

fn main() {
    let args = Args::parse();
    println!("Compiling {:?}", args.file);

    if args.lex || args.parse || args.codegen {
        println!("Running lexer");
    }

    if args.parse || args.codegen {
        println!("Running parser");
    }

    if args.codegen {
        println!("Running codegen");
    }
}
