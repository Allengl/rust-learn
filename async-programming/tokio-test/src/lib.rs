pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

async fn double(x: u64) -> u64 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn double_works() {
        let result = double(2).await;
        assert_eq!(result, 4);
    }
}
