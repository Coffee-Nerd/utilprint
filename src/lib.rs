use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn rgb_to_xterm(r: u8, g: u8, b: u8) -> u8 {
    // Check if color is truly grayscale (all components very close)
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let is_grayscale = (max - min) < 20; // Only consider grayscale if very close
    
    if is_grayscale {
        // Use grayscale colors (232-255)
        let gray = (r as u16 + g as u16 + b as u16) / 3;
        let gray_index = if gray < 8 {
            0
        } else if gray > 238 {
            23
        } else {
            (gray - 8) / 10
        };
        232 + gray_index as u8
    } else {
        // Use the 216 color cube (16-231)
        let r6 = (r as f64 / 255.0 * 5.0).round() as u8;
        let g6 = (g as f64 / 255.0 * 5.0).round() as u8;
        let b6 = (b as f64 / 255.0 * 5.0).round() as u8;
        
        16 + 36 * r6 + 6 * g6 + b6
    }
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
    fn random_xterm_gradient(&self) -> String;
    fn random_truecolor_gradient(&self) -> String;
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

    fn random_xterm_gradient(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Generate two random xterm colors based on string hash + current time
        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let mut hasher1 = DefaultHasher::new();
        self.hash(&mut hasher1);
        time_seed.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        (self.to_string() + "salt").hash(&mut hasher2);
        (time_seed.wrapping_add(12345)).hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        // Generate random RGB colors first
        let start_r = ((hash1 >> 16) & 0xFF) as u8;
        let start_g = ((hash1 >> 8) & 0xFF) as u8;
        let start_b = (hash1 & 0xFF) as u8;
        
        let end_r = ((hash2 >> 16) & 0xFF) as u8;
        let end_g = ((hash2 >> 8) & 0xFF) as u8;
        let end_b = (hash2 & 0xFF) as u8;
        
        let chars: Vec<char> = self.chars().collect();
        let non_whitespace: Vec<(usize, char)> = chars
            .iter()
            .enumerate()
            .filter(|(_, &c)| !c.is_whitespace())
            .map(|(i, &c)| (i, c))
            .collect();
        
        if non_whitespace.is_empty() {
            return self.to_string();
        }
        
        let mut result = String::new();
        let mut last_color: Option<u8> = None;
        
        for (i, c) in chars.iter().enumerate() {
            if c.is_whitespace() {
                result.push(*c);
            } else {
                let non_ws_idx = non_whitespace.iter().position(|(idx, _)| *idx == i).unwrap();
                let total = non_whitespace.len();
                
                let t = if total > 1 {
                    non_ws_idx as f64 / (total - 1) as f64
                } else {
                    0.0
                };
                
                // Interpolate RGB values
                let r = (start_r as f64 + (end_r as i16 - start_r as i16) as f64 * t) as u8;
                let g = (start_g as f64 + (end_g as i16 - start_g as i16) as f64 * t) as u8;
                let b = (start_b as f64 + (end_b as i16 - start_b as i16) as f64 * t) as u8;
                
                // Convert RGB to closest XTERM 256 color
                let xterm_color = rgb_to_xterm(r, g, b);
                
                if last_color != Some(xterm_color) {
                    result.push_str(&format!("@x{:03}", xterm_color));
                    last_color = Some(xterm_color);
                }
                result.push(*c);
            }
        }
        result.push_str("@u");
        result
    }

    fn random_truecolor_gradient(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Generate two random RGB colors based on string hash + current time
        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        time_seed.hash(&mut hasher);
        let hash = hasher.finish();
        
        let start_r = ((hash >> 16) & 0xFF) as u8;
        let start_g = ((hash >> 8) & 0xFF) as u8;
        let start_b = (hash & 0xFF) as u8;
        
        let mut hasher2 = DefaultHasher::new();
        (self.to_string() + "gradient").hash(&mut hasher2);
        (time_seed.wrapping_add(54321)).hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        let end_r = ((hash2 >> 16) & 0xFF) as u8;
        let end_g = ((hash2 >> 8) & 0xFF) as u8;
        let end_b = (hash2 & 0xFF) as u8;
        
        let chars: Vec<char> = self.chars().collect();
        let non_whitespace: Vec<(usize, char)> = chars
            .iter()
            .enumerate()
            .filter(|(_, &c)| !c.is_whitespace())
            .map(|(i, &c)| (i, c))
            .collect();
        
        if non_whitespace.is_empty() {
            return self.to_string();
        }
        
        let mut result = String::new();
        let mut last_color_code: Option<String> = None;
        
        for (i, c) in chars.iter().enumerate() {
            if c.is_whitespace() {
                result.push(*c);
            } else {
                let non_ws_idx = non_whitespace.iter().position(|(idx, _)| *idx == i).unwrap();
                let total = non_whitespace.len();
                
                let t = if total > 1 {
                    non_ws_idx as f64 / (total - 1) as f64
                } else {
                    0.0
                };
                
                let r = (start_r as f64 + (end_r as i16 - start_r as i16) as f64 * t) as u8;
                let g = (start_g as f64 + (end_g as i16 - start_g as i16) as f64 * t) as u8;
                let b = (start_b as f64 + (end_b as i16 - start_b as i16) as f64 * t) as u8;
                
                let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
                
                if last_color_code.as_ref() != Some(&color_code) {
                    result.push_str(&color_code);
                    last_color_code = Some(color_code);
                }
                result.push(*c);
            }
        }
        result.push_str("\x1b[0m");
        result
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

    #[test]
    fn test_random_xterm_gradient() {
        let text = "Hello World".random_xterm_gradient();
        // Should contain xterm color codes
        assert!(text.contains("@x"));
        // Should end with reset
        assert!(text.ends_with("@u"));
        // Should preserve whitespace
        assert!(text.contains(" "));
        // Should be random - different calls should produce different results
        let text2 = "Hello World".random_xterm_gradient();
        // Note: There's a tiny chance they could be the same, but very unlikely
        println!("Text 1: {}", text);
        println!("Text 2: {}", text2);
    }

    #[test]
    fn test_random_truecolor_gradient() {
        let text = "Hello World".random_truecolor_gradient();
        // Should contain ANSI truecolor escape sequences
        assert!(text.contains("\x1b[38;2;"));
        // Should end with reset
        assert!(text.ends_with("\x1b[0m"));
        // Should preserve whitespace
        assert!(text.contains(" "));
        // Should be random - different calls should produce different results
        let text2 = "Hello World".random_truecolor_gradient();
        println!("Text 1: {}", text);
        println!("Text 2: {}", text2);
    }

    #[test]
    fn test_empty_string_gradients() {
        assert_eq!("".random_xterm_gradient(), "");
        assert_eq!("".random_truecolor_gradient(), "");
    }

    #[test]
    fn test_whitespace_only_gradients() {
        assert_eq!("   ".random_xterm_gradient(), "   ");
        assert_eq!("   ".random_truecolor_gradient(), "   ");
    }
}
