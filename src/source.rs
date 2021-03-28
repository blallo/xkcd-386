use rand::distributions::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::internal::{InternalError, InternalResult};
use crate::mangle::mangle_word;

const RANDOM_WORD_LENGTH: usize = 16;

pub struct Source {
    kind: SourceKind,
    limit: Option<u32>,
}

pub enum SourceKind {
    Plain(FileSource),
    Mangled(FileSource),
    Random(RandomSource),
}

pub struct FileSource {
    reader: BufReader<File>,
    mangled: Vec<String>,
}

pub struct RandomSource {
    rng: ThreadRng,
}

impl Source {
    pub fn with_mangling(self) -> Self {
        match self.kind {
            SourceKind::Plain(from_file) => Self {
                kind: SourceKind::Mangled(from_file),
                limit: self.limit,
            },
            kind => Self {
                kind,
                limit: self.limit,
            },
        }
    }

    pub fn with_limit(self, limit: Option<u32>) -> Self {
        Self { limit, ..self }
    }

    fn tick(&mut self) -> InternalResult<()> {
        if let Some(limit) = self.limit {
            self.limit = Some(limit - 1);
            if limit < 1 {
                return Err(InternalError::Generic("Iteration stoped".to_string()));
            }
        }
        Ok(())
    }

    fn get(&mut self) -> Option<String> {
        match &mut self.kind {
            SourceKind::Plain(from_file) => from_file.get(),
            SourceKind::Mangled(from_file) => from_file.mangle_get(),
            SourceKind::Random(random) => Some(random.get()),
        }
    }
}

impl RandomSource {
    fn get(&mut self) -> String {
        self.rng
            .clone()
            .sample_iter(&Alphanumeric)
            .take(RANDOM_WORD_LENGTH)
            .map(char::from)
            .collect()
    }
}

impl FileSource {
    fn get(&mut self) -> Option<String> {
        let mut result = String::new();
        self.reader.read_line(&mut result).ok().map(|_| result)
    }

    fn mangle_get(&mut self) -> Option<String> {
        if let Some(result) = self.mangled.pop() {
            Some(result)
        } else {
            self.get().and_then(|result| {
                self.mangled = mangle_word(result);
                self.mangle_get()
            })
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = InternalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "random" {
            Ok(Self {
                kind: SourceKind::Random(RandomSource { rng: thread_rng() }),
                limit: None,
            })
        } else {
            File::open(value)
                .map(BufReader::new)
                .map(|reader| Self {
                    kind: SourceKind::Plain(FileSource {
                        reader,
                        mangled: vec![],
                    }),
                    limit: None,
                })
                .map_err(|e| e.into())
        }
    }
}

impl Iterator for Source {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let result = self.get();
        self.tick().ok().and_then(|()| result)
    }
}
