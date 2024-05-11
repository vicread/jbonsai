use crate::vocoder::Vocoder;

type Parameter = Vec<Vec<f64>>;

pub struct SpeechGenerator {
    fperiod: usize,
    vocoder: Vocoder,
    spectrum: Parameter,
    lf0: Parameter,
    lpf: Parameter,

    next: usize,
}

impl SpeechGenerator {
    pub fn new(
        fperiod: usize,
        vocoder: Vocoder,
        spectrum: Parameter,
        lf0: Parameter,
        lpf: Parameter,
    ) -> Self {
        if !lf0.is_empty() && lf0[0].len() != 1 {
            panic!("The size of lf0 static vector must be 1.");
        }
        if !lpf.is_empty() && lpf[0].len() % 2 == 0 {
            panic!("The number of low-pass filter coefficient must be odd numbers.");
        }

        Self {
            fperiod,
            vocoder,
            spectrum,
            lf0,
            lpf,
            next: 0,
        }
    }

    pub fn fperiod(&self) -> usize {
        self.fperiod
    }

    pub fn synthesized_frames(&self) -> usize {
        self.next
    }

    /// Generate speech
    pub fn generate_step(&mut self, speech: &mut [f64]) -> usize {
        if self.lf0.len() <= self.next {
            return 0;
        }
        if speech.len() < self.fperiod {
            panic!("The length of speech buffer must be larger than fperiod.");
        }

        self.vocoder.synthesize(
            self.lf0[self.next][0],
            &self.spectrum[self.next],
            &self.lpf[self.next],
            speech,
        );
        self.next += 1;

        self.fperiod
    }

    pub fn generate_all(mut self) -> Vec<f64> {
        if self.next != 0 {
            eprintln!("The speech generator has already synthesized some frames.");
        }

        let mut buf = vec![0.0; (self.lf0.len() - self.next) * self.fperiod];
        while self.generate_step(&mut buf[self.next * self.fperiod..]) > 0 {}

        buf
    }
}
