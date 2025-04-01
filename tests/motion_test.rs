use rustquake::{
    helper_functions::{load_column_file, min_max},
    models::{motion::Motion, units::AccelUnit},
};

#[test]
fn test_from_acc_real_data() {
    let acc = load_column_file("tests/data/acc.txt");
    let motion = Motion::from_acceleration(acc.clone(), 0.05, AccelUnit::G);

    assert_eq!(motion.accelerations.len(), acc.len());
    assert_eq!(motion.velocities.len(), acc.len());
    assert_eq!(motion.displacements.len(), acc.len());

    let (acc_min, acc_max) = min_max(&motion.accelerations.to_vec());
    let (vel_min, vel_max) = min_max(&motion.velocities.to_vec());
    let (disp_min, disp_max) = min_max(&motion.displacements.to_vec());

    assert!((acc_max - 0.16076).abs() < 1e-5);
    assert!((acc_min + 0.1255).abs() < 1e-5);
    assert!((vel_max - 23.60571).abs() < 1e-5);
    assert!((vel_min + 29.58521).abs() < 1e-5);
    assert!((disp_max - 6.49525).abs() < 1e-5);
    assert!((disp_min + 52.61423).abs() < 1e-5);
}
