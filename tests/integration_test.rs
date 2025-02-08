use anyhow::Result;

use tidyql_rs::app::App;

#[test]
fn test_app_creation() -> Result<()> {
    let app = App::new()?;
    assert!(app.files.len() > 0);
    assert!(app.selected_file.is_none());
    assert!(app.file_content.is_empty());
    assert!(app.formatted_content.is_none());
    assert!(app.diff_content.is_none());
    assert!(app.notification.is_none());
    Ok(())
}

#[test]
fn test_select_file() -> Result<()> {
    let mut app = App::new()?;
    app.select_file(0)?;
    assert!(app.selected_file.is_some());
    Ok(())
}

#[test]
fn test_read_selected_file() -> Result<()> {
    let mut app = App::new()?;
    app.select_file(0)?;
    assert!(!app.file_content.is_empty());
    Ok(())
}

/*
Not good for ci/cd :(
#[test]
fn test_format_sql() -> Result<()> {
    let temp_dir = tempdir()?;
    let test_sql = "SELECT * FROM users;";
    let test_file_path = temp_dir.path().join("test.sql");
    let mut test_file = File::create(&test_file_path)?;
    test_file.write_all(test_sql.as_bytes())?;

    let mut app = App::new()?;
    app.files.push(test_file_path.clone());
    app.select_file(0)?;
    app.format_sql()?;
    assert!(app.formatted_content.is_some());
    assert!(app.notification.is_some());
    assert_eq!(app.notification.unwrap(), "SQL formatted successfully");

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_save_formatted_file() -> Result<()> {
    let temp_dir = tempdir()?;
    let test_sql = "SELECT * FROM users;";
    let test_file_path = temp_dir.path().join("test.sql");
    let mut test_file = File::create(&test_file_path)?;
    test_file.write_all(test_sql.as_bytes())?;

    let mut app = App::new()?;
    app.files.push(test_file_path.clone());
    app.select_file(0)?;
    app.format_sql()?;
    app.save_formatted_file()?;
    assert!(app.notification.is_some());
    assert_eq!(app.notification.unwrap(), "File saved successfully");

    temp_dir.close()?;
    Ok(())
}
*/
