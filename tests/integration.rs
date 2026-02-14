use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

use tempfile::TempDir;

use dot::manifest::{MANIFEST_FILE, Manifest};

/// Integration test helper: initialize a manifest
fn init_manifest(repo: &Path) {
    let manifest_path = repo.join(MANIFEST_FILE);
    fs::write(&manifest_path, "").unwrap();
}

/// Integration test helper: add a file to the repo
fn add_file(source: &Path, repo: &Path) {
    let file_name = source.file_name().unwrap();
    let local_path = repo.join(file_name);

    fs::rename(source, &local_path).unwrap();
    let canonical = local_path.canonicalize().unwrap();
    symlink(&canonical, source).unwrap();

    let manifest_path = repo.join(MANIFEST_FILE);
    let mut manifest = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path).unwrap();
        Manifest::parse(&content).unwrap()
    } else {
        Manifest::empty()
    };
    manifest.insert(file_name.into(), source).unwrap();
    manifest.save_to(&manifest_path).unwrap();
}

/// Integration test helper: remove a tracked file
fn remove_file(file_name: &str, repo: &Path) {
    let manifest_path = repo.join(MANIFEST_FILE);
    let content = fs::read_to_string(&manifest_path).unwrap();
    let mut manifest = Manifest::parse(&content).unwrap();

    let symlink_path = manifest.get(Path::new(file_name)).unwrap();
    let local_path = repo.join(file_name);

    fs::remove_file(&symlink_path).unwrap();
    fs::rename(&local_path, &symlink_path).unwrap();

    manifest.remove(Path::new(file_name));
    manifest.save_to(&manifest_path).unwrap();
}

/// Integration test helper: sync all tracked files
fn sync_files(repo: &Path) {
    let manifest_path = repo.join(MANIFEST_FILE);
    let content = fs::read_to_string(&manifest_path).unwrap();
    let manifest = Manifest::parse(&content).unwrap();

    for (local_name, symlink_result) in manifest.iter() {
        let symlink_path = symlink_result.unwrap();
        let local_path = repo.join(local_name);

        if !symlink_path.exists() {
            if let Some(parent) = symlink_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let canonical = local_path.canonicalize().unwrap();
            symlink(&canonical, &symlink_path).unwrap();
        }
    }
}

#[test]
fn full_workflow() {
    let repo = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();

    // Create config in fake home
    let config = home.path().join(".myconfig");
    fs::write(&config, "my config").unwrap();

    // Init
    init_manifest(repo.path());
    assert!(repo.path().join(MANIFEST_FILE).exists());

    // Add
    add_file(&config, repo.path());
    assert!(repo.path().join(".myconfig").exists());
    assert!(config.symlink_metadata().unwrap().file_type().is_symlink());

    // Sync (up to date - no-op)
    sync_files(repo.path());

    // Remove
    remove_file(".myconfig", repo.path());
    assert!(!repo.path().join(".myconfig").exists());
    assert!(!config.symlink_metadata().unwrap().file_type().is_symlink());
    assert_eq!(fs::read_to_string(&config).unwrap(), "my config");
}

#[test]
fn sync_recreates_deleted_symlinks() {
    let repo = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();

    let config = home.path().join(".config");
    fs::write(&config, "content").unwrap();

    init_manifest(repo.path());
    add_file(&config, repo.path());

    // Delete symlink manually
    fs::remove_file(&config).unwrap();

    // Sync recreates it
    sync_files(repo.path());
    assert!(config.symlink_metadata().unwrap().file_type().is_symlink());
}
