use crate::faker::filesystem::raw::*;
use crate::faker::boolean::raw::Boolean;
use crate::impls::std::path::PathFaker;
use crate::locales::{Data, EN};
use crate::{Dummy, Fake};
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::PathBuf;

impl<L: Data> Dummy<FilePath<L>> for PathBuf {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FilePath<L>, rng: &mut R) -> Self {
        let faker = PathFaker::new(L::PATH_ROOT_DIRS, L::PATH_SEGMENTS, L::PATH_EXTENSIONS, 4);
        faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<FilePath<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FilePath<L>, rng: &mut R) -> Self {
        let faker = PathFaker::new(L::PATH_ROOT_DIRS, L::PATH_SEGMENTS, L::PATH_EXTENSIONS, 4);
        let p: PathBuf = faker.fake_with_rng(rng);
        p.to_string_lossy().into()
    }
}

impl<L: Data> Dummy<FileName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FileName<L>, rng: &mut R) -> Self {
        let name = L::PATH_SEGMENTS.choose(rng).unwrap();
        let ext = L::PATH_EXTENSIONS.choose(rng).unwrap();
        format!("{}.{}", name, ext)
    }
}

impl<L: Data> Dummy<FileExtension<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FileExtension<L>, rng: &mut R) -> Self {
        L::PATH_EXTENSIONS.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<FileExtension<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FileExtension<L>, rng: &mut R) -> Self {
        let ext = L::PATH_EXTENSIONS.choose(rng).unwrap();
        (*ext).to_string()
    }
}

impl<L: Data> Dummy<DirPath<L>> for PathBuf {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DirPath<L>, rng: &mut R) -> Self {
        let faker = PathFaker::new(L::PATH_ROOT_DIRS, L::PATH_SEGMENTS, &[], 4);
        faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DirPath<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DirPath<L>, rng: &mut R) -> Self {
        let faker = PathFaker::new(L::PATH_ROOT_DIRS, L::PATH_SEGMENTS, &[], 4);
        let p: PathBuf = faker.fake_with_rng(rng);
        p.to_string_lossy().into()
    }
}

const UNSTABLE_SEMVER: &'static [&'static str] = &[
    "alpha", "beta", "rc"
];

impl<L: Data> Dummy<Semver<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Semver<L>, rng: &mut R) -> Self {
        let patch = &mut(0..20).fake_with_rng::<u8, _>(rng).to_string();
        if Boolean(EN, 10).fake_with_rng(rng) {
            patch.push_str(&format!(
                "-{}.{}",
                *UNSTABLE_SEMVER.choose(rng).unwrap(),
                &(0..9).fake_with_rng::<u8, _>(rng).to_string()
            ));
        }
        format!(
            "{}.{}.{}",
            &(0..9).fake_with_rng::<u8, _>(rng).to_string(),
            &(0..20).fake_with_rng::<u8, _>(rng).to_string(),
            patch
        )
    }
}
