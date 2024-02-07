pub mod cli;
pub mod timer;

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
    fn from_secs_works() {
        let mut timer = timer::Timer::from_secs(33365);
        let origin = Instant::now();
        timer.countup(&origin);
        assert!(timer.get_seconds() == 33366)
    }

    #[test]
    fn tic_works() {
        let mut timer = timer::Timer::new(0,0,0);
        let origin = Instant::now();
        timer.tic(&origin);
        assert!(timer.get_seconds() == 1)
    }

    #[test]
    fn countup_works() {
        let mut timer = timer::Timer::new(1,1,1);
        let origin = Instant::now();
        timer.countup(&origin);
        assert!(timer.get_seconds() == 3662)
    }

    #[test]
    fn countdown_works() {
        let mut timer = timer::Timer::new(0,0,8);
        let origin = Instant::now();
        timer.countdown(&origin);
        assert!(timer.get_seconds() == 7)
    }
}
