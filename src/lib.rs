use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;
use std::fmt::Write;

pub struct ColorCodes {
    codes: HashMap<String, String>,
}

lazy_static! {
    static ref COLOR_CODES: ColorCodes = ColorCodes::new();
}

impl ColorCodes {
    pub fn new() -> Self {
        let color_pairs = vec![
            ("@d", "\x1b[30m"),
            ("@r", "\x1b[31m"),
            ("@g", "\x1b[32m"),
            ("@y", "\x1b[33m"),
            ("@b", "\x1b[34m"),
            ("@m", "\x1b[35m"),
            ("@c", "\x1b[36m"),
            ("@w", "\x1b[37m"),
            ("@D", "\x1b[90m"),
            ("@R", "\x1b[91m"),
            ("@G", "\x1b[92m"),
            ("@Y", "\x1b[93m"),
            ("@B", "\x1b[94m"),
            ("@M", "\x1b[95m"),
            ("@C", "\x1b[96m"),
            ("@W", "\x1b[97m"),
            ("@u", "\x1b[0m"),
        ];

        let mut codes = HashMap::with_capacity(color_pairs.len() + 256);
        for &(code, ansi_code) in &color_pairs {
            codes.insert(code.to_string(), ansi_code.to_string());
        }

        for i in 0..256 {
            codes.insert(format!("@x{:03}", i), format!("\x1b[38;5;{}m", i));
        }

        ColorCodes { codes }
    }

    pub fn utilprint(&self, text: &str) -> Result<(), String> {
        let mut output = String::with_capacity(text.len() * 2);
        let mut current_index = 0;

        while let Some(start) = text[current_index..].find(|c| c == '@' || c == '#') {
            let (end, code) = self.get_code(&text[current_index + start..]);
            write!(
                output,
                "{}{}",
                &text[current_index..current_index + start],
                code
            )
            .map_err(|e| e.to_string())?;
            current_index += start + end;
        }

        write!(output, "{}\x1b[0m", &text[current_index..]).map_err(|e| e.to_string())?;
        println!("{}", output);
        Ok(())
    }

    fn get_code(&self, text: &str) -> (usize, String) {
        if let Some(color) = self.codes.get(&text[..2]) {
            return (2, color.clone());
        }
        if text.starts_with("@@") || text.starts_with("##") {
            return (2, text[1..2].to_string());
        }

        if text.starts_with("@x") && text.len() >= 5 {
            return (5, self.codes.get(&text[..5]).cloned().unwrap_or_default());
        }
        if text.starts_with('#') && text.len() >= 5 {
            return (
                5,
                parse_unicode_code(&text[1..5]).map_or_else(String::new, |c| c.to_string()),
            );
        }
        (2, String::new())
    }
}

fn parse_unicode_code(code: &str) -> Option<char> {
    u32::from_str_radix(code, 16).ok().and_then(char::from_u32)
}

pub fn utilprint(text: impl AsRef<str>) {
    if let Err(e) = COLOR_CODES.utilprint(text.as_ref()) {
        eprintln!("Error printing text: {}", e);
    }
}

fn preprocess_text(s: &str) -> String {
    let mut result = String::new();
    let mut current_index = 0;

    while let Some(start) = s[current_index..].find(|c| c == '@' || c == '#') {
        result.push_str(&s[current_index..current_index + start]);

        let (end, code_type) = COLOR_CODES.get_code(&s[current_index + start..]);

        match code_type {
            _ if code_type.is_empty() => (),
            _ if code_type.starts_with("\x1b") => (),
            _ => result.push(' '),
        }

        current_index += start + end;
    }

    result.push_str(&s[current_index..]);
    result
}

fn visible_length(s: &str) -> usize {
    preprocess_text(s).chars().count()
}

pub fn print_help() {
    let help_text = vec![
        "                  UTILPRINT HELP              ",
        "",
        " Standard Colors:                             ",
        " @d@@d - @dBlack@u         @r@@r - @rRed@u",
        " @y@@y - @yYellow@u        @b@@b - @bBlue@u ",
        " @c@@c - @cCyan@u          @w@@w - @wWhite@u  ",
        " @g@@g - @gGreen@u         @m@@m - @mMagenta@u",
        "",
        " Bright Colors:                               ",
        " @D@@D - @DGray@u          @R@@R - @RBright Red@u",
        " @G@@G - @GBright Green@u  @Y@@Y - @YBright Yellow@u ",
        " @B@@B - @BBright Blue@u   @M@@M - @MBright Magenta@u ",
        " @C@@C - @CBright Cyan@u   @W@@W - @WBright White@u ",
        "",
        " Xterm 256 Colors:                            ",
        " Use @@xNNN where NNN is the color number.    ",
        " Example: @@x124 for a deep red.              ",
        "",
        " Unicode Characters:                          ",
        " Use ##NNNN where NNNN is the Unicode code.    ",
        " Example: ##263A for a smiley face. #263A  ",
    ];

    let max_visible_length = help_text
        .iter()
        .map(|line| visible_length(line))
        .max()
        .unwrap_or(0);

    let horizontal_border: String = "═".repeat(max_visible_length + 2);
    println!("╔{}╗", horizontal_border);
    for line in help_text.iter() {
        let visible_line_length = visible_length(&preprocess_text(line));
        let padding = max_visible_length - visible_line_length;
        let formatted_line = format!("║ {}{} ║", line, " ".repeat(padding));
        utilprint(&formatted_line);
    }
    println!("╚{}╝", horizontal_border);
}

pub trait TextEffects {
    fn pastelbow(&self) -> String;
    fn rainbow(&self) -> String;
    fn lover(&self) -> String;
    fn red(&self) -> String;
}

impl TextEffects for &str {
    fn pastelbow(&self) -> String {
        let colors = vec![
            "@x226", "@x190", "@x155", "@x119", "@x120", "@x084", "@x085", "@x050", "@x051",
            "@x045", "@x075", "@x069", "@x105", "@x099", "@x135", "@x165", "@x201", "@x200",
            "@x205", "@x204", "@x210", "@x209", "@x215", "@x220", "@x226",
        ];
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            let color = &colors[i % colors.len()];
            result.push_str(&format!("{}{}", color, c));
        }
        result.push_str("@u");
        result
    }

    fn rainbow(&self) -> String {
        let colors = vec![
            "@x196", "@x202", "@x208", "@x214", "@x220", "@x226", "@x154", "@x082", "@x046",
            "@x035", "@x025", "@x021", "@x020", "@x019", "@x054", "@x055", "@x091", "@x092",
            "@x126", "@x161",
        ];
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            let color = &colors[i % colors.len()];
            result.push_str(&format!("{}{}", color, c));
        }
        result.push_str("@u");
        result
    }

    fn lover(&self) -> String {
        let colors = vec!["@x218", "@x176", "@x229", "@x219", "@x153", "@x219"];
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            let color = &colors[i % colors.len()];
            result.push_str(&format!("{}{}", color, c));
        }
        result.push_str("@u");
        result
    }

    fn red(&self) -> String {
        format!("@r{}@u", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilprint() {
        let text = "@rHello, @bworld!";
        assert_eq!(COLOR_CODES.utilprint(text), Ok(()));
    }

    #[test]
    fn test_parse_unicode_code() {
        assert_eq!(parse_unicode_code("263A"), Some('☺'));
        assert_eq!(parse_unicode_code("1F602"), Some('😂'));
    }

    #[test]
    fn test_pastelbow() {
        let text = "Hello".pastelbow();
        assert!(text.starts_with("@x226H"));
        assert!(text.contains("@x190e"));
        assert!(text.contains("@x155l"));
        assert!(text.contains("@x119o"));
    }

    #[test]
    fn test_rainbow() {
        let text = "Hello World".rainbow();
        assert!(text.contains("@x196H"));
        assert!(text.contains("@x202e"));
        assert!(text.contains("@x208l"));
        assert!(text.contains("@x214l"));
        assert!(text.contains("@x220o"));
        assert!(text.contains("@x226 "));
        assert!(text.contains("@x154W"));
        assert!(text.contains("@x082o"));
        assert!(text.contains("@x046r"));
        assert!(text.contains("@x035l"));
        assert!(text.contains("@x025d"));
    }

    #[test]
    fn test_lover() {
        let text = "Hello".lover();
        assert!(text.contains("@x218H"));
        assert!(text.contains("@x176e"));
        assert!(text.contains("@x229l"));
        assert!(text.contains("@x219l"));
        assert!(text.contains("@x153o"));
    }
}
