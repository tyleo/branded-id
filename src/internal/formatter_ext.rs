use std::{
    fmt,
    fmt::{Alignment, Formatter},
};

pub trait FormatterExt {
    fn pad_str(&mut self, s: &str) -> fmt::Result;
}

/// Padding after the end of something. Returned by `Formatter::padding`.
#[must_use = "don't forget to write the post padding"]
struct PostPadding {
    fill: char,
    padding: usize,
}

impl PostPadding {
    fn new(fill: char, padding: usize) -> PostPadding {
        PostPadding { fill, padding }
    }

    /// Write this post padding.
    fn write(self, f: &mut Formatter<'_>) -> fmt::Result {
        for _ in 0..self.padding {
            write!(f, "{}", self.fill)?;
        }
        Ok(())
    }
}

/// Write the pre-padding and return the unwritten post-padding. Callers are
/// responsible for ensuring post-padding is written after the thing that is
/// being padded.
fn padding(this: &mut Formatter, padding: usize) -> Result<PostPadding, fmt::Error> {
    let (pre_pad, post_pad) = match this.align() {
        Some(Alignment::Right) => (padding, 0),
        Some(Alignment::Center) => (padding / 2, (padding + 1) / 2),
        _ => (0, padding),
    };

    let fill = this.fill();
    for _ in 0..pre_pad {
        write!(this, "{}", fill)?;
    }

    Ok(PostPadding::new(fill, post_pad))
}

impl FormatterExt for Formatter<'_> {
    fn pad_str(&mut self, s: &str) -> fmt::Result {
        if let Some(width) = self.width() {
            let chars_count = s.chars().count();
            // If we're under the maximum width, check if we're over the minimum
            // width, if so it's as easy as just emitting the string.
            if chars_count >= width {
                write!(self, "{}", s)
            }
            // If we're under both the maximum and the minimum width, then fill
            // up the minimum width with the specified string + some alignment.
            else {
                let post_padding = padding(self, width - chars_count)?;
                write!(self, "{}", s)?;
                post_padding.write(self)
            }
        } else {
            write!(self, "{}", s)
        }
    }
}
