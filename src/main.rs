use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "crd-yaml-to-markdown", about = "A CRD Yaml to Markdown table generator")]
struct Cli {
    #[structopt(parse(from_os_str), short="i", required=true)]
    yaml: std::path::PathBuf,
//    #[structopt(parse(from_os_str), short="o",required=false)]
//    output: std::path::PathBuf,
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.yaml)
    .expect("could not read file");
    let yaml = crd_yaml_to_markdown::yaml_to_markdown(content.as_str())?;
    println!("{}",yaml);
    Ok(())
}
