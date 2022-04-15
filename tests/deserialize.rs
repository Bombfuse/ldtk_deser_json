use ldtk_nano::{deserialize_project, project::Project};

#[test]
fn test_deserialize() {
    let json = String::from_utf8(include_bytes!("../assets/harvest_hero.ldtk").to_vec()).unwrap();
    let project = deserialize_project(json);

    println!("{:?}", (project.bg_color));
}
