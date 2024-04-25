use eigenrank::EigenTrustContract;

#[test]
fn test_simple_cases() {
    let contract = EigenTrustContract::default();
    assert_eq!(contract.some_method(), true);
}
