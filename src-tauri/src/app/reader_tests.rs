use super::reader::Reader;
use anyhow::Result;
use tempfile::tempdir;

fn commit(repository: &git2::Repository) -> Result<git2::Oid> {
    let mut index = repository.index()?;
    index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let oid = index.write_tree()?;
    let signature = git2::Signature::now("test", "test@email.com").unwrap();
    let commit_oid = repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "some commit",
        &repository.find_tree(oid)?,
        &[&repository.find_commit(repository.refname_to_id("HEAD")?)?],
    )?;
    Ok(commit_oid)
}

fn test_repository() -> Result<git2::Repository> {
    let path = tempdir()?.path().to_str().unwrap().to_string();
    let repository = git2::Repository::init(&path)?;
    let mut index = repository.index()?;
    let oid = index.write_tree()?;
    let signature = git2::Signature::now("test", "test@email.com").unwrap();
    repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &repository.find_tree(oid)?,
        &[],
    )?;
    Ok(repository)
}

#[test]
fn test_working_directory_reader_read_file() -> Result<()> {
    let repository = test_repository()?;

    let file_path = "test.txt";
    std::fs::write(&repository.path().parent().unwrap().join(file_path), "test")?;

    let reader = super::reader::get_working_directory_reader(&repository);
    assert_eq!(reader.read_to_string(&file_path)?, "test");

    Ok(())
}

#[test]
fn test_commit_reader_read_file() -> Result<()> {
    let repository = test_repository()?;

    let file_path = "test.txt";
    std::fs::write(&repository.path().parent().unwrap().join(file_path), "test")?;

    let oid = commit(&repository)?;

    std::fs::write(
        &repository.path().parent().unwrap().join(file_path),
        "test2",
    )?;

    let reader = super::reader::get_commit_reader(&repository, oid)?;
    assert_eq!(reader.read_to_string(&file_path)?, "test");

    Ok(())
}

#[test]
fn test_working_directory_reader_list_files() -> Result<()> {
    let repository = test_repository()?;

    std::fs::write(
        &repository.path().parent().unwrap().join("test.txt"),
        "test",
    )?;
    std::fs::create_dir(&repository.path().parent().unwrap().join("dir"))?;
    std::fs::write(
        &repository
            .path()
            .parent()
            .unwrap()
            .join("dir")
            .join("test.txt"),
        "test",
    )?;

    let reader = super::reader::get_working_directory_reader(&repository);
    let files = reader.list_files(".")?;
    assert_eq!(files.len(), 2);
    assert!(files.contains(&"test.txt".to_string()));
    assert!(files.contains(&"dir/test.txt".to_string()));

    Ok(())
}

#[test]
fn test_commit_reader_list_files() -> Result<()> {
    let repository = test_repository()?;

    std::fs::write(
        &repository.path().parent().unwrap().join("test.txt"),
        "test",
    )?;
    std::fs::create_dir(&repository.path().parent().unwrap().join("dir"))?;
    std::fs::write(
        &repository
            .path()
            .parent()
            .unwrap()
            .join("dir")
            .join("test.txt"),
        "test",
    )?;

    let oid = commit(&repository)?;

    std::fs::remove_dir_all(&repository.path().parent().unwrap().join("dir"))?;

    let reader = super::reader::get_commit_reader(&repository, oid)?;
    let files = reader.list_files(".")?;
    assert_eq!(files.len(), 2);
    assert!(files.contains(&"test.txt".to_string()));
    assert!(files.contains(&"dir/test.txt".to_string()));

    Ok(())
}
