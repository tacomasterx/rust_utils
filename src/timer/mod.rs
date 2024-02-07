// use super::cli;
use std::/* {io::{stdout, Write}, thread::{self, JoinHandle}, */ time::Instant;//};

pub struct Timer {
    hours: usize,
    minutes: usize,
    seconds: usize,
}

impl Timer {
    pub fn get_seconds(&self) -> usize{
        self.seconds + self.minutes * 60 + self.hours * 3600
    }

    pub fn get_digits(&self) -> (usize, usize, usize) {
        (self.hours,  self.minutes, self.seconds)
    }

    pub fn from_secs(secs: usize) -> Timer {
        let hours = secs / 3600;
        let seconds = secs % 60;
        let minutes = (secs/60) % 60;
        Timer{
            seconds,
            minutes,
            hours,
        }
    }

    pub fn new(hours: usize, minutes: usize, seconds: usize) -> Timer{
        Timer{
            seconds,
            minutes,
            hours,
        }
    }

    pub fn tic(&mut self, start: &Instant) {
        loop {
            let elapsed_time = start.elapsed().as_micros();
            if elapsed_time >= 1_000_000 {
                let value = self.get_seconds() + 1;
                if value >= 60 {
                    self.seconds = value % 60;
                    if value / 60 >= 60 {
                        self.hours = value / 60 / 60;
                        self.minutes = (value / 60) % 60;
                        break
                    }
                    self.minutes = value / 60;
                    break
                }
                self.seconds = value;
                break
            }
        }
    }

    pub fn countup(&mut self, start: &Instant) {
        loop {
            let elapsed_time = start.elapsed().as_micros();
            if  elapsed_time >= 1_000_000 {
                if self.seconds < 60 {
                    self.seconds += 1;
                    break
                }
                if self.minutes < 60 {
                    self.minutes += 1;
                    self.seconds = 00;
                    break
                }
                self.hours += 1;
                self.minutes = 00;
                self.seconds = 00;
                break
            }
        }
    }

    pub fn countdown(&mut self, start: &Instant) {
        loop {
            let elapsed_time = start.elapsed().as_micros();
            if  elapsed_time >= 1_000_000 {
                if self.seconds > 0 {
                    self.seconds -= 1;
                    break
                }
                if self.minutes > 0 {
                    self.minutes -= 1;
                    self.seconds = 59;
                    break
                }
                if self.hours > 0 {
                    self.hours -= 1;
                    self.minutes = 59;
                    self.seconds = 59;
                    break
                }
                break
            }
        }
    }

    pub fn text_render(&self) -> String {
        let hours = match self.hours > 9 as usize {
            true => format!("{}", &self.hours ),
            false => format!("0{}", &self.hours),
        };
        let minutes = match self.minutes > 9 as usize {
            true => format!("{}", &self.minutes ),
            false => format!("0{}", &self.minutes),
        };
        let seconds = match self.seconds > 9 as usize {
            true => format!("{}", &self.seconds ),
            false => format!("0{}", &self.seconds),
        };

        format!("[{}:{}:{}]", hours, minutes, seconds)
    }

    pub fn binary_conversion(value: usize) -> [u8;10] {
        const TWO: usize = 2;
        let mut bits = [0;10];
        let mut buffer: usize = value;
        for i in ( 0..10 ).rev() {
            let bit_value = TWO.pow(i as u32);
            bits[i] = ( buffer / bit_value ) as u8;
            buffer = buffer % bit_value;
        }
        bits
    }

    pub fn display_digit_render<'b>(value: u8) -> [[&'b str;3];15] {
        const ZERO:  &str = "0000";
        const ONE:   &str = "1111";
        const TWO:   &str = "2222";
        const THREE: &str = "3333";
        const FOUR:  &str = "4444";
        const FIVE:  &str = "5555";
        const SIX:   &str = "6666";
        const SEVEN: &str = "7777";
        const EIGHT: &str = "8888";
        const NINE:  &str = "9999";
        const BLANK: &str = "    ";
        const ERROR: &str = "EEEE";
        const COLON: &str = "::::";
        let mut number_array = [[BLANK;3];15];

        match value {
            0 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = ZERO;
                        }
                    } else {
                        number_array[row][0] = ZERO;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = ZERO;
                    }
                }
                number_array
            },

            1 => {
                for row in 0..15 {
                    number_array[row][2] = ONE;
                }
                number_array
            },

            2 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = TWO;
                        }
                        continue
                    } if ( row > 2 ) && ( row < 6 ) {
                        number_array[row][0] = BLANK;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = TWO;
                        continue
                    }
                    number_array[row][0] = TWO;
                    number_array[row][1] = BLANK;
                    number_array[row][2] = BLANK;

                }
                number_array
            },

            3 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = THREE;
                        }
                    } else {
                        number_array[row][0] = BLANK;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = THREE;
                    }
                }
                number_array
            },

            4 => {
                for row in 0..15 {
                    if row > 5 && row < 9 {
                        for cell in 0..3 {
                            number_array[row][cell] = FOUR;
                        }
                        continue
                    } 
                    if row < 6 {
                        number_array[row][0] = FOUR;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = FOUR;
                        continue
                    }
                    number_array[row][0] = BLANK;
                    number_array[row][1] = BLANK;
                    number_array[row][2] = FOUR;
                }
                number_array
            },

            5 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = FIVE;
                        }
                        continue
                    } if ( row > 2 ) && ( row < 6 ) {
                        number_array[row][0] = FIVE;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = BLANK;
                        continue
                    }
                    number_array[row][0] = BLANK;
                    number_array[row][1] = BLANK;
                    number_array[row][2] = FIVE;

                }
                number_array
            },

            6 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = SIX;
                        }
                        continue
                    } 
                    if (row > 2) && (row < 6) {
                        number_array[row][0] = SIX;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = BLANK;
                        continue
                    }
                    number_array[row][0] = SIX;
                    number_array[row][1] = BLANK;
                    number_array[row][2] = SIX;
                }
                number_array
            },

            7 => {
                for row in 0..15 {
                    if row < 3 {
                        for cell in 0..3 {
                            number_array[row][cell] = SEVEN;
                        }
                        continue
                    }
                    number_array[row][2] = SEVEN;
                }
                number_array
            },


            8 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = EIGHT;
                        }
                    } else {
                        number_array[row][0] = EIGHT;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = EIGHT;
                    }
                }
                number_array
            },

            9 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = NINE;
                        }
                        continue
                    } 
                    if (row > 2) && (row < 6) {
                        number_array[row][0] = NINE;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = NINE;
                        continue
                    }
                    number_array[row][0] = BLANK;
                    number_array[row][1] = BLANK;
                    number_array[row][2] = NINE;
                }
                number_array
            },

            13 => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = BLANK;
                        }
                    } else {
                        number_array[row][0] = BLANK;
                        number_array[row][1] = COLON;
                        number_array[row][2] = BLANK;
                    }
                }
                number_array
            },

            _ => {
                for row in 0..15 {
                    if ( row < 3 ) || (row > 5 && row < 9) || (row > 11) {
                        for cell in 0..3 {
                            number_array[row][cell] = ERROR;
                        }
                    } else {
                        number_array[row][0] = ERROR;
                        number_array[row][1] = BLANK;
                        number_array[row][2] = BLANK;
                    }
                }
                number_array
            },
        }
    }

    pub fn display_format_line(& self, line_number: u8, display_type: &Type) {
        if line_number > 14 {
            panic!("Line out of bounds!")
        }
        let ( hours, minutes, seconds ) = self.get_digits();
        let digit_array = [
            Timer::separate_digits(hours),
            Timer::separate_digits(minutes),
            Timer::separate_digits(seconds),
        ];
        let mut row_string = String::new();     
        let limit = match display_type {
            Type::Short => {2},
            Type::Long => {3},
        };
        for hms in 0..limit {
            for digit in 0..2 {
                let binding = Timer::display_digit_render(digit_array[hms][digit]);
                let something = binding[line_number as usize].join("");
                let binding = something.as_str();
                row_string += binding;
                if digit < 1 { 
                    row_string += "   ";
                }
            }
            if hms < limit - 1 { 
                let binding = Timer::display_digit_render(13);
                let something = binding[line_number as usize].join("");
                let binding = something.as_str();
                row_string += binding;
            }
        }
        println!("{row_string}");
    }

    pub fn separate_digits(value: usize) -> [u8;2] {
        let value = value as u8;
        let mut digit_array: [u8;2] = [0;2];
        digit_array[0] = value/10;
        digit_array[1] = value%10;
        digit_array
    }


}

pub enum Type {
    Short, Long
}


