use clap::clap_app;

fn main() {
    let app = clap_app!(enigma =>
        (version: "1.0")
        (author: "Colin F. <ctfristoe@gmail.com>")
        (about: "A digital Wermacht/Lufwaffe enigma machine")
        // (@arg verbose: --rotors "Sets the rotors to use")
        (@subcommand encode =>
            (about: "controls testing features")
            (@arg INPUT: +required "Sets the input file to use")
        )
    );
    let matches = app.get_matches();

    println!(input);
}