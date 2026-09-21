use demos::novelty::Novelty;

#[test]
fn the_meter_reads_the_grid_and_the_zeros() {
    let meter = Novelty::new(16.0, 16, 10).unwrap();
    let heights = meter.heights();
    assert_eq!(heights.len(), 129);
    assert_eq!((heights[0], heights[128]), (8.0, 16.0));
    assert_eq!(meter.sieve(), 131_072);
    assert_eq!(meter.dots(false).len(), 129);
    assert_eq!(meter.dots(true).len(), 129);
    let gammas = meter.gammas();
    assert_eq!(gammas.len(), 10);
    assert!((gammas[0] - 14.134_725).abs() < 1e-6);
    assert!((gammas[9] - 49.773_832).abs() < 1e-5);
    let amplitudes = meter.amplitudes();
    assert!((amplitudes[0] - 0.1879).abs() < 5e-4);
    assert!((amplitudes[9] - 4.286e-3).abs() < 5e-6);
}

#[test]
fn the_wave_of_the_first_zeros_closes_on_the_dots() {
    let meter = Novelty::new(16.0, 16, 10).unwrap();
    assert_eq!(meter.miss(0).unwrap(), 1.0);
    let one = meter.miss(1).unwrap();
    let ten = meter.miss(10).unwrap();
    assert!(one > 0.3 && one < 0.7, "{one}");
    assert!(ten < 5e-2, "{ten}");
    assert_eq!(meter.wave(0, false, &[8.0, 12.0]).unwrap(), vec![0.0, 0.0]);
    let smooth = meter.wave(1, false, &[10.0]).unwrap()[0];
    let sharp = meter.wave(1, true, &[10.0]).unwrap()[0];
    assert!((sharp - smooth / 32.0).abs() < 1e-12);
    let rough = meter.dots(true);
    assert!(rough.iter().all(|v| v.abs() < 2.0));
    assert!(rough.iter().any(|v| v.abs() > 0.1));
}

#[test]
fn the_full_meter_is_the_first_thirty_zeros() {
    let meter = Novelty::new(20.0, 16, 30).unwrap();
    assert_eq!(meter.heights().len(), 193);
    assert_eq!(meter.sieve(), 2_097_152);
    assert!(meter.miss(30).unwrap() < 1e-2);
    let peak = meter.dots(false).iter().fold(0.0f64, |a, v| a.max(v.abs()));
    assert!(peak > 0.3 && peak < 0.6, "{peak}");
}

#[test]
fn the_meter_refuses_what_it_cannot_hold() {
    assert!(Novelty::new(8.0, 16, 1).is_err());
    assert!(Novelty::new(22.0, 16, 1).is_err());
    assert!(Novelty::new(12.0, 0, 1).is_err());
    assert!(Novelty::new(12.0, 33, 1).is_err());
    assert!(Novelty::new(12.0, 16, 139).is_err());
    let meter = Novelty::new(12.0, 8, 3).unwrap();
    assert!(meter.wave(4, false, &[9.0]).is_err());
    assert!(meter.miss(4).is_err());
}
