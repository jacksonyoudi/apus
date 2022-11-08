use std::fmt;
use std::fmt::Formatter;

pub struct ColoredString {
    input: String,
    fgcolor: String,
    bgcolor: String,
}


pub trait Colorize {
    // 关联常量
    const FG_RED: &'static str = "31";
    const FG_YELLOW: &'static str = "43";

    fn red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
}


impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: String::default(),
            bgcolor: String::default(),
        }
    }
}

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res: String = String::from("\x1B[");
        let mut has_wrote: bool = false;
        if !self.bgcolor.is_empty() {
            res.push_str(&self.bgcolor);
            has_wrote = true;
        }


        if !self.fgcolor.is_empty() {
            if has_wrote { res.push(';'); }
            res.push_str(&self.fgcolor);
        }

        res.push('m');
        res
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut input = &self.input.clone();
        f.write_str(&self.compute_style())?;
        f.write_str(input)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}


impl<'a> Colorize for ColoredString {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED),
            ..self
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::FG_YELLOW),
            ..self
        }
    }
}


impl<'a> Colorize for &'a str {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::FG_YELLOW),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
}
