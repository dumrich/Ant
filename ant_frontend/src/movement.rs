use std::io::Write;
use crate::buffer::DefaultBuffer;
use crate::draw::draw_lines;


pub fn move_down(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_y+buffer.shown_first-1!=buffer.line_count as u16 {

        let height = buffer.current_y as usize;
        let width = buffer.current_x as usize;
        
        let (_terminal_width,terminal_height) = termion::terminal_size().unwrap();
        let possible_width = buffer.lines[buffer.shown_first as usize+height-1].1.chars().count();

        if height==(terminal_height as usize)-2 {
            if (buffer.shown_line as usize)!=buffer.line_count {
                draw_lines(screen, buffer, (buffer.shown_line+1) as usize);
                buffer.shown_line = buffer.shown_line+1;
                buffer.shown_first = buffer.shown_first+1;
                buffer.set_position(screen, width as u16, (height) as u16);
            }
        } else if !(width>=possible_width+4) {
            buffer.set_position(screen, buffer.current_x, buffer.current_y+1);

        } else if possible_width==0 {
            buffer.set_position(screen, 5, buffer.current_y+1);
        } else if possible_width+4<width {
            buffer.set_position(screen, (possible_width+4) as u16, buffer.current_y+1);

        }

    }
    
}

pub fn move_up(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_y != 1 {
        let height = buffer.current_y as usize;
        let width = buffer.current_x as usize;
        
        let possible_width = buffer.lines[buffer.shown_first as usize+height-3].1.chars().count();
        if possible_width+4>=width {
            buffer.set_position(screen, buffer.current_x, buffer.current_y-1);
            
            screen.flush().unwrap();
        } else if possible_width==0 {
            buffer.set_position(screen, 5, buffer.current_y-1);

            screen.flush().unwrap();
        } else if possible_width+5<width {
            buffer.set_position(screen, (possible_width+5) as u16, buffer.current_y-1);

            screen.flush().unwrap();
        }

    } else if buffer.shown_first > 1 {

        draw_lines(screen, buffer, (buffer.shown_line-1) as usize);
        buffer.set_position(screen, buffer.current_x, 1);

        buffer.shown_line = buffer.shown_line-1;
        buffer.shown_first = buffer.shown_first-1;
        screen.flush().unwrap();
    }
}

pub fn move_left(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_x!=5 {
        buffer.set_position(screen, buffer.current_x-1, buffer.current_y);

        screen.flush().unwrap();

    }
}

pub fn move_right(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let height = buffer.current_y;
    let width = buffer.current_x as usize;

    let possible_width = buffer.lines[(buffer.shown_first+height-2) as usize].1.chars().count();

    if possible_width==0 {
       () 
    } else if width!=possible_width+5 {
        buffer.set_position(screen, buffer.current_x+1, buffer.current_y);

        screen.flush().unwrap();
    } else if width == possible_width+5 {
        buffer.on_last = true;
    }
}
