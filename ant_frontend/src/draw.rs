use std::io::Write;
use crate::buffer::DefaultBuffer;
use termion::color;


const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

pub fn draw_statusline(screen: &mut impl Write, buffer: &DefaultBuffer, height: u16, width: u16) {

    write!(screen, "{}", termion::cursor::Goto(1, height-1)).unwrap();

    let width = width as usize;

    let mut file_name = "[No Name]".to_string();
    let mut file_path = "".to_string();

    if let Some(name) = &buffer.name {
        file_name = name.clone();
    }
    
    if let Some(path) = &buffer.file_path {
        file_path = path.clone();
    }

    let mut status = format!(
        "{} - {} lines",
        file_name,
        buffer.line_count
    );

    let buffer_indicator = format!(
        "{} | {}/{}",
        file_path,
        buffer.current_x,
        buffer.current_y
    );

    let len = status.len() + buffer_indicator.len();
    status.push_str(&" ".repeat(width.saturating_sub(len)));
    status = format!("{}{}", status, buffer_indicator);
    status.truncate(width);
    write!(screen, "{}", color::Bg(STATUS_BG_COLOR)).unwrap();
    write!(screen, "{}", color::Fg(STATUS_FG_COLOR)).unwrap();
    write!(screen, "{}\r", status).unwrap();
    write!(screen, "{}", color::Bg(color::Reset)).unwrap();
    write!(screen, "{}", color::Fg(color::Reset)).unwrap();
}


pub fn draw_lines(screen: &mut impl Write, buffer: &mut DefaultBuffer, last: u16) {
    let line_iterator = &buffer.lines;

    for (line_number, line) in line_iterator.iter().enumerate() { 
        let line_number = (line_number+1) as usize;
        if line_number>last as usize {
            continue;
        } else {
            if (line_number>=10)&(line_number<100) {
                write!(screen, "{}{}  {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            } else if (line_number>=100)&(line_number<1000) {
                write!(screen, "{}{} {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            } else {
                write!(screen, "{}{}   {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            }
        }
    };
    buffer.set_position(screen, 5, 1);
}