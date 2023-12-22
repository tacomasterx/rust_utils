pub mod cli {
    use std::io::{self, Write};
    use getch::Getch;

    pub fn pause() {
        let key_input = Getch::new();
        print!("Press any key to continue...");
        if let Err(error) = io::stdout().flush() {
            panic!("{}", error)
        }
        if let Err(error) = key_input.getch() {
            panic!("{}", error)
        }

    }

    pub fn printf(str: String) {
        print!("{}", str);
        if let Err(error) = io::stdout().flush() {
            panic!("{error}")
        }
    }
}

pub mod timer {
    use std::time::Instant;

    pub struct Timer {
        hours: u32,
        minutes: u32,
        seconds: u32,
    }

    impl Timer {
        pub fn get_seconds(&mut self) -> u32{
            self.seconds + self.minutes * 60 + self.hours * 3600
        }

        pub fn new(hours: u32, minutes: u32, seconds: u32) -> Timer{
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

        pub fn countdown(&mut self, start: &Instant) {
            loop {
                let elapsed_time = start.elapsed().as_micros();
                if elapsed_time >= 1_000_000 {
                    let value = self.get_seconds() - 1;
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

        pub fn render(&mut self) -> String {
            let hours = match self.hours > 9 as u32 {
                true => format!("{}", &self.hours ),
                false => format!("0{}", &self.hours),
            };
            let minutes = match self.minutes > 9 as u32 {
                true => format!("{}", &self.minutes ),
                false => format!("0{}", &self.minutes),
            };
            let seconds = match self.seconds > 9 as u32 {
                true => format!("{}", &self.seconds ),
                false => format!("0{}", &self.seconds),
            };

            format!("[{}:{}:{}]", hours, minutes, seconds)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::{ cli, timer };
    use std::time::Instant;

    #[test]
    fn pause_works() {
        cli::pause()
    }

    #[test]
    fn printf_works() {
        cli::printf(format!( "Just a test! {}", 1234 ))
    }

    #[test]
    fn tic_works() {
        let mut timer = timer::Timer::new(0,0,0);
        let origin = Instant::now();
        timer.tic(&origin);
        assert!(timer.get_seconds() == 1)
    }

    #[test]
    fn countdown_works() {
        let mut timer = timer::Timer::new(0,0,8);
        let origin = Instant::now();
        timer.countdown(&origin);
        assert!(timer.get_seconds() == 7)
    }
}
