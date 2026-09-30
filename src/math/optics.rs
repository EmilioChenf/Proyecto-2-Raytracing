use super::Vec3;

#[must_use]
pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * (2.0 * incident.dot(&normal))
}

/// Ley de Snell. `normal` debe apuntar contra el rayo incidente.
#[must_use]
pub fn refract(incident: Vec3, normal: Vec3, index_from: f32, index_to: f32) -> Option<Vec3> {
    let eta = index_from / index_to;
    let cosine = (-incident).dot(&normal).clamp(0.0, 1.0);
    let perpendicular = (incident + normal * cosine) * eta;
    let parallel_squared = 1.0 - perpendicular.norm_squared();

    if parallel_squared < 0.0 {
        return None;
    }

    Some((perpendicular - normal * parallel_squared.sqrt()).normalize())
}

/// Aproximación de Schlick a la reflectancia de Fresnel.
#[must_use]
pub fn schlick(cosine: f32, index_from: f32, index_to: f32) -> f32 {
    let ratio = (index_from - index_to) / (index_from + index_to);
    let base = ratio * ratio;
    base + (1.0 - base) * (1.0 - cosine.clamp(0.0, 1.0)).powi(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_incidence_does_not_bend() {
        let incident = Vec3::new(0.0, -1.0, 0.0);
        let direction = refract(incident, Vec3::new(0.0, 1.0, 0.0), 1.0, 1.5)
            .expect("normal incidence must refract");
        assert!((direction - incident).norm() < 1.0e-6);
    }

    #[test]
    fn detects_total_internal_reflection() {
        let incident = Vec3::new(0.866_025_4, 0.5, 0.0);
        assert!(refract(incident, Vec3::new(0.0, -1.0, 0.0), 1.5, 1.0).is_none());
    }

    #[test]
    fn glass_reflectance_is_four_percent_head_on() {
        assert!((schlick(1.0, 1.0, 1.5) - 0.04).abs() < 1.0e-6);
    }
}
