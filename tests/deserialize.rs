use ldtk_deser_json::{deserialize_project, project::Project};

#[test]
fn test_deserialize() {
    let json = String::from_utf8(include_bytes!("../assets/harvest_hero.ldtk").to_vec()).unwrap();
    deserialize_project(json).unwrap();
}

#[test]
fn test_correct_properties() {
    let json = String::from_utf8(include_bytes!("../assets/harvest_hero.ldtk").to_vec()).unwrap();
    let project = deserialize_project(json).unwrap();

    assert_eq!(project.bg_color, String::from("#40465B"));
}
