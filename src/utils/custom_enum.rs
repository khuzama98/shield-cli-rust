#[derive(Debug)]
pub enum Language {
    Javascript,
    Typescript,
}

#[derive(Debug)]
pub enum ProofSystem {
    Groth16,
    Plonk,
}

#[derive(Debug)]
pub enum PackageManager {
    Yarn,
    NPM,
}
