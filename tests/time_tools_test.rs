#[cfg(test)]
mod TimeUtilsTests {
    use hong_tool::time_tools::TimeUtils;

    #[test]
    fn test_now(){
        assert!(TimeUtils::now().is_ok())
    }
}
