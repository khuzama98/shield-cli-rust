use super::custom_enum::{Language, PackageManager, ProofSystem};

#[derive(Debug)]
pub struct SelectedArguments {
    project_path: String,
    language: Language,
    proof_system: ProofSystem,
    package_manager: PackageManager,
}

impl SelectedArguments {
    pub fn new() -> Self {
        SelectedArguments {
            project_path: String::new(),
            language: Language::Javascript,
            proof_system: ProofSystem::Groth16,
            package_manager: PackageManager::NPM,
        }
    }

    pub fn set_project_path(&mut self, path: String) {
        self.project_path = path;
    }

    pub fn set_language(&mut self, lang: Language) {
        self.language = lang;
    }

    pub fn set_proof_system(&mut self, proof: ProofSystem) {
        self.proof_system = proof;
    }

    pub fn set_package_manager(&mut self, pkg: PackageManager) {
        self.package_manager = pkg;
    }
}
