use crate::config::Config;
use crate::opt::ListArgs;
use shell_escape::escape;
use std::borrow::Cow;
use std::io;

pub fn run(args: &ListArgs) {
    list(args, &Config::load_or_exit(), &mut io::stdout()).unwrap();
}

fn list<W: io::Write>(_args: &ListArgs, config: &Config, out: &mut W) -> Result<(), io::Error> {
    for abbrev in &config.abbrevs {
        let abbr = abbrev.trigger.get_abbr();
        let snippet = escape(Cow::from(abbrev.function.get_snippet_string()));

        writeln!(out, "{}={}", abbr, snippet)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::load_from_str(
            r"
            abbrevs:
              - name: git
                abbr: g
                replace-self: git

              - name: git commit
                abbr: c
                replace-self: commit
                global: true
                context: 'git'

              - name: '>/dev/null'
                abbr: 'null'
                replace-self: '>/dev/null'
                global: true

              - name: $HOME
                abbr: home
                replace-self: $HOME
                evaluate: true
            ",
        )
        .unwrap()
    }

    #[test]
    fn test_list() {
        let args = ListArgs {};
        let config = test_config();

        let mut buf = Vec::new();
        list(&args, &config, &mut std::io::BufWriter::new(&mut buf)).unwrap();

        let output = std::str::from_utf8(&buf).unwrap();

        let expected = r"g=git
c=commit
null='>/dev/null'
home='$HOME'
";

        assert_eq!(output, expected);
    }
}
