pub struct TestStruct
{
    pub text: String
}

impl TestStruct {
    pub fn new(text: String) -> TestStruct {
        return TestStruct {text};
    }
}