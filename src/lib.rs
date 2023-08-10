// Build a sieve of Eratosthenes.
pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut values = vec![true; max + 1];
    values[0] = false;
    values[1] = false;
    let upper: usize = ((max as f64).sqrt() as usize) + 1;
    for i in 2..upper {
        if values[i] {
            // println!("-----\nSweeping {i}");
            for j in ((i*i)..=max).step_by(i) {
                // println!("Marking {j}");
                values[j] = false;
            }
        }
    }
    values
}

pub fn sieve_to_primes(sieve: &[bool]) -> Vec<usize> {
    sieve.iter()
        .enumerate()
        .filter(|pair| *pair.1) // Include pair if prime flag is true
        .map(|pair| pair.0) // Change type by passing on only the usize position/prime-number
        .collect::<Vec::<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn sieve_x () {
    //    for i in 2..100 {
    //        let result = sieve_of_eratosthenes(i);
    //        println!("----- {i}\n{:?}", result);
    //    }
    //    assert!(false);
    //}

    #[test]
    fn primes_1 () {
        let sieve = sieve_of_eratosthenes(1);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, vec![]);
    }
    
    #[test]
    fn primes_2 () {
        let sieve = sieve_of_eratosthenes(2);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, vec![2]);
    }
    
    #[test]
    fn primes_3_4 () {
        let expected = vec![2, 3];
        let sieve = sieve_of_eratosthenes(3);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(4);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_5_6 () {
        let expected = vec![2, 3, 5];
        let sieve = sieve_of_eratosthenes(5);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(6);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_7_8_9_10 () {
        let expected = vec![2, 3, 5, 7];
        let sieve = sieve_of_eratosthenes(7);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(8);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(9);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(10);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_11_12 () {
        let expected = vec![2, 3, 5, 7, 11];
        let sieve = sieve_of_eratosthenes(11);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(12);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_13_14_15_16 () {
        let expected = vec![2, 3, 5, 7, 11, 13];
        let sieve = sieve_of_eratosthenes(13);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(14);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(15);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(16);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_17_18 () {
        let expected = vec![2, 3, 5, 7, 11, 13, 17];
        let sieve = sieve_of_eratosthenes(17);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(18);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn primes_19_20 () {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let sieve = sieve_of_eratosthenes(19);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
        let sieve = sieve_of_eratosthenes(20);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(primes, expected);
    }

    #[test]
    fn sieve_2 () {
        let result = sieve_of_eratosthenes(1);
        assert_eq!(2, result.len());
        assert!(! result[0]);
        assert!(! result[1]);
    }

    #[test]
    fn sieve_6 () {
        let result = sieve_of_eratosthenes(6);
        println!("{:?}", result);
        assert_eq!(7, result.len());
        assert!(result[2]);
        assert!(result[3]);
        assert!(result[5]);
        for i in [4, 6].iter() {
            assert!(! result[*i]);
        }
    }

    #[test]
    fn sieve_10 () {
        let result = sieve_of_eratosthenes(20);
        println!("{:?}", result);
        assert_eq!(21, result.len());
        assert!(result[2]);
        assert!(result[3]);
        assert!(result[5]);
        assert!(result[7]);
        assert!(result[11]);
        assert!(result[13]);
        assert!(result[17]);
        assert!(result[19]);
        for i in [4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20].iter() {
            assert!(! result[*i]);
        }
    }
}
