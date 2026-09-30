use proyecto_2_raytracing::{
    app::{GameState, StateMachine},
    camera::OrbitCamera,
    characters::CharacterKind,
    geometry::Cube,
    material::{rgb, Material, Texture},
    math::{reflect, refract, schlick, Ray, Vec3},
    renderer::RayTracer,
    scene::{
        create_character_selection, create_main_world, create_panda_world, create_pardo_world,
        create_polar_world,
    },
};

#[test]
fn public_cube_intersection_reports_surface_data() {
    let cube = Cube::new(Vec3::zeros(), Vec3::repeat(2.0), 4);
    let ray =
        Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
    let hit = cube
        .intersect(&ray, 0.001, f32::INFINITY)
        .expect("cube must be hit");

    assert_eq!(hit.material_index, 4);
    assert!((hit.distance - 4.0).abs() < 1.0e-6);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn optical_helpers_cover_reflection_refraction_and_fresnel() {
    let incident = Vec3::new(0.6, -0.8, 0.0);
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let reflected = reflect(incident, normal);
    let refracted = refract(incident, normal, 1.0, 1.333).expect("water transmits this ray");

    assert!((reflected - Vec3::new(0.6, 0.8, 0.0)).norm() < 1.0e-6);
    assert!(refracted.x.abs() < incident.x.abs());
    assert!(schlick(0.0, 1.0, 1.5) > schlick(1.0, 1.0, 1.5));
}

#[test]
fn material_exposes_every_rubric_property() {
    let material = Material::new(
        "test glass",
        rgb(120, 180, 220),
        Texture::Ripples {
            secondary: rgb(200, 240, 255),
            scale: 3.0,
        },
    )
    .with_surface(0.9, 96.0, 0.25)
    .with_transmission(0.8, 1.5);

    assert!(material.sample(Vec3::new(0.3, 0.0, 0.6), (0.2, 0.7)).norm() > 0.0);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.transparency, 0.8);
    assert_eq!(material.reflectivity, 0.25);
    assert_eq!(material.refractive_index, 1.5);
}

#[test]
fn state_machine_completes_navigation_transition() {
    let mut states = StateMachine::default();
    states.begin_transition(GameState::CharacterSelection);
    assert_eq!(states.current(), GameState::Transition);
    assert_eq!(states.update(1.0), Some(GameState::CharacterSelection));
    states.begin_transition(GameState::PandaWorld);
    assert_eq!(states.update(1.0), Some(GameState::PandaWorld));
    assert_eq!(
        states.current().back_target(),
        Some(GameState::CharacterSelection)
    );
}

#[test]
fn every_world_has_geometry_materials_lights_and_bvh() {
    let main = create_main_world();
    let polar = create_polar_world();
    let pardo = create_pardo_world();
    let panda = create_panda_world();

    for scene in [&main, &polar.scene, &pardo.scene, &panda.scene] {
        assert!(scene.cubes.len() > 50);
        assert!(scene.materials.len() >= 15);
        assert!(!scene.lights.is_empty());
        assert!(scene.acceleration_node_count() > 1);
    }
    assert_eq!(polar.character, CharacterKind::Polar);
    assert_eq!(pardo.character, CharacterKind::Pardo);
    assert_eq!(panda.character, CharacterKind::Panda);
}

#[test]
fn selection_raycast_and_small_render_are_functional() {
    let selection = create_character_selection();
    let camera = OrbitCamera::new(Vec3::new(0.0, 2.5, 0.0), 15.5, std::f32::consts::PI, 0.08);
    let center_ray = camera.ray_from_screen(80.0, 45.0, 160, 90);
    assert_eq!(selection.pick(&center_ray), Some(CharacterKind::Pardo));

    let pixels = RayTracer::default().render(&selection.scene, &camera, 32, 18);
    assert_eq!(pixels.len(), 32 * 18);
    assert!(pixels.windows(2).any(|pair| pair[0] != pair[1]));
}
