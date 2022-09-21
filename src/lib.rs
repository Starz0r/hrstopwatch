use windows::Win32::{
    Foundation::{GetLastError, BOOL, WIN32_ERROR},
    System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
};

#[derive(PartialEq)]
pub enum StopwatchState {
    Running = 1,
    Paused = 2,
    Stopped = 3,
}

pub struct Stopwatch {
    frequency: i64,
    start_time: i64,
    time_paused: i64,
    pause_time: i64,
    stop_time: i64,
    intern_state: StopwatchState,
}

impl Stopwatch {
    pub fn start() -> Result<Self, windows::core::Error> {
        let mut sw = Self {
            frequency: 0,
            start_time: 0,
            time_paused: 0,
            pause_time: 0,
            stop_time: 0,
            intern_state: StopwatchState::Running,
        };
        let mut result = unsafe { QueryPerformanceFrequency(&mut sw.frequency) };
        if result == false {
            return unsafe { Err(GetLastError().ok().unwrap_err_unchecked()) };
        }
        result = unsafe { QueryPerformanceCounter(&mut sw.start_time) };
        if result == false {
            return unsafe { Err(GetLastError().ok().unwrap_err_unchecked()) };
        }

        Ok(sw)
    }

    pub fn stop(&mut self) -> Result<(), windows::core::Error> {
        let mut result = BOOL(true as i32);

        match self.intern_state {
            StopwatchState::Running => {
                result = unsafe { QueryPerformanceCounter(&mut self.stop_time) };
            }
            StopwatchState::Paused => {
                self.stop_time = self.pause_time;
            }
            _ => self.intern_state = StopwatchState::Stopped,
        };

        if result == false {
            get_win32_error(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }

    pub fn pause(&mut self) -> Result<(), windows::core::Error> {
        let mut result = BOOL(true as i32);

        if self.intern_state == StopwatchState::Running {
            result = unsafe { QueryPerformanceCounter(&mut self.pause_time) };
            self.intern_state = StopwatchState::Paused;
        }

        if result == false {
            get_win32_error(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }

    pub fn resume(&mut self) -> Result<(), windows::core::Error> {
        let mut time = 0;
        let result = unsafe { QueryPerformanceCounter(&mut time) };

        if self.intern_state == StopwatchState::Paused {
            let time_paused = time - self.pause_time;
            self.time_paused += time_paused;
            self.intern_state = StopwatchState::Running;
        }

        if result == false {
            get_win32_error(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn elapsed_ticks(&self) -> i64 {
        self.stop_time - self.start_time - self.time_paused
    }

    #[inline]
    pub fn elasped_nano(&self) -> i64 {
        self.elapsed_ticks() * 1000000000 / self.frequency
    }

    #[inline]
    pub fn elapsed_micro(&self) -> i64 {
        self.elasped_nano() / 1000
    }

    #[inline]
    pub fn elapsed_milli(&self) -> i64 {
        self.elapsed_micro() / 1000
    }

    #[inline]
    pub fn elapsed_seconds(&self) -> i64 {
        self.elapsed_milli() / 1000
    }

    #[inline]
    pub fn elasped_nano_f64(&self) -> f64 {
        (self.elapsed_ticks() as f64) * 1000000000.0 / (self.frequency as f64)
    }

    #[inline]
    pub fn elapsed_micro_f64(&self) -> f64 {
        self.elasped_nano_f64() / 1000.0
    }

    #[inline]
    pub fn elapsed_milli_f64(&self) -> f64 {
        self.elapsed_micro_f64() / 1000.0
    }

    #[inline]
    pub fn elapsed_seconds_f64(&self) -> f64 {
        self.elapsed_milli_f64() / 1000.0
    }
}

fn get_win32_error(err: WIN32_ERROR) -> Result<(), windows::core::Error> {
    err.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle() {
        let mut sw = Stopwatch::start().unwrap();
        sw.pause().unwrap();
        sw.resume().unwrap();
        sw.stop().unwrap();
    }

    #[test]
    fn readme() -> Result<(), Box<dyn std::error::Error>> {
        let mut num: u64 = 0;
        let mut stopwatch = Stopwatch::start()?;
        for i in 0..10000 {
            num += i;
        }
        stopwatch.stop()?;
        Ok(())
    }
}
