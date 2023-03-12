pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

pub fn div(left: usize,right:usize) -> usize {
    left / right
}

pub fn mul(left:usize,right:usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 2),0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(2, 2),1);
    }
    #[test]
     fn test_mul() {
        assert_eq!(mul(2, 2),4);
     }

}
