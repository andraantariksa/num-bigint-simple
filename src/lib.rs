//! Module scope to avoid namespace polution
pub mod bigint {
    // Don't change the base
    const BASE: i32 = 1000;

    /// BigInteger (Integer)
    #[derive(Clone, Debug)]
    pub struct BigInteger {
        /// Sign are used to determine wether the number is positive or negative
        /// * true for positive
        /// * false for negative
        pub sign: bool,
        pub number: Vec<i32>,
    }

    impl BigInteger {
        pub fn from_str(number_string: &str) -> BigInteger {
            // Check if it's a negative value or not by it's sign
            let sign = number_string.chars().nth(0).unwrap();

            // Remove the sign
            let number_string_after_sign = match sign {
                '-' | '+' => &number_string[1..],
                _ => number_string,
            };

            BigInteger {
                sign: sign != '-',
                number: BigUInteger::from_str(number_string_after_sign).number,
            }
        }
    }

    //#[allow(clippy::suspicious_arithmetic_impl)]
    impl std::ops::Sub for BigInteger {
        type Output = BigInteger;

        fn sub(self, other: BigInteger) -> BigInteger {
            let mut other_ = BigInteger { ..other };
            let mut self_ = BigInteger { ..self };
            // To make the bigger number goes to left (self_) and the smaller goes to right (other_)
            let swapped = if BigUInteger::from_BigInteger(self_.clone())
                < BigUInteger::from_BigInteger(other_.clone())
            {
                std::mem::swap(&mut self_, &mut other_);
                true
            } else {
                false
            };
            // (1+) - (2+)
            if self_.sign && other_.sign {
                BigInteger {
                    sign: !swapped,
                    number: (BigUInteger::from_BigInteger(self_.clone())
                        - BigUInteger::from_BigInteger(other_.clone()))
                    .number,
                }
            // (1-) - (2-) == -((1+) - (2+))
            } else if !self_.sign && !other_.sign {
                BigInteger {
                    sign: swapped,
                    number: (BigUInteger::from_BigInteger(self_.clone())
                        - BigUInteger::from_BigInteger(other_.clone()))
                    .number,
                }
            // (1-) - (2+) == -((1+) + (2+))
            } else if !self_.sign && other_.sign {
                BigInteger {
                    sign: swapped,
                    number: (BigUInteger::from_BigInteger(self_.clone())
                        + BigUInteger::from_BigInteger(other_.clone()))
                    .number,
                }
            // (1+) - (2-) == (1+) + (2+)
            } else {
                BigInteger {
                    sign: !swapped,
                    number: (BigUInteger::from_BigInteger(self_.clone())
                        + BigUInteger::from_BigInteger(other_.clone()))
                    .number,
                }
            }
        }
    }

    //#[allow(clippy::suspicious_arithmetic_impl)]
    impl std::ops::Add for BigInteger {
        type Output = BigInteger;

        fn add(self, other: BigInteger) -> BigInteger {
            let mut other_ = BigInteger { ..other };
            let mut self_ = BigInteger { ..self };
            // (1+) + (2+)
            if self_.sign == other_.sign {
                BigInteger {
                    sign: self_.sign,
                    number: (BigUInteger::from_BigInteger(self_.clone())
                        + BigUInteger::from_BigInteger(other_.clone()))
                    .number,
                }
            } else {
                // To make the bigger number goes to left (self_) and the smaller goes to right (other_)
                if BigUInteger::from_BigInteger(self_.clone())
                    < BigUInteger::from_BigInteger(other_.clone())
                {
                    std::mem::swap(&mut self_, &mut other_);
                }
                // (1+) +  (2-) == (1+) - (2+)
                if self_.sign && !other_.sign {
                    BigInteger {
                        sign: true,
                        number: (BigUInteger::from_BigInteger(self_.clone())
                            - BigUInteger::from_BigInteger(other_.clone()))
                        .number,
                    }
                // (1-) + (2+) == -((1+) - (2+))
                } else {
                    BigInteger {
                        sign: false,
                        number: (BigUInteger::from_BigInteger(self_.clone())
                            - BigUInteger::from_BigInteger(other_.clone()))
                        .number,
                    }
                }
            }
        }
    }

    impl std::cmp::Eq for BigInteger {}

    impl std::cmp::PartialEq for BigInteger {
        fn eq(&self, other: &BigInteger) -> bool {
            self.sign == other.sign
                && BigUInteger::from_BigInteger(self.clone())
                    == BigUInteger::from_BigInteger(other.clone())
        }
    }

    impl std::cmp::Ord for BigInteger {
        fn cmp(&self, other: &BigInteger) -> std::cmp::Ordering {
            if self.sign != other.sign {
                if !self.sign {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else if self.sign {
                BigUInteger::from_BigInteger(self.clone())
                    .cmp(&BigUInteger::from_BigInteger(other.clone()))
            } else {
                BigUInteger::from_BigInteger(other.clone())
                    .cmp(&BigUInteger::from_BigInteger(self.clone()))
            }
        }
    }

    impl std::cmp::PartialOrd for BigInteger {
        fn partial_cmp(&self, other: &BigInteger) -> std::option::Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::fmt::Display for BigInteger {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let sign = if self.sign {
                "" // Positive sign
            } else {
                "-" // Negative sign
            };

            write!(
                f,
                "{}{}",
                sign,
                BigUInteger::from_BigInteger(self.clone()).to_string()
            )
        }
    }

    /// BigUInteger (Whole number)
    #[derive(Clone, Debug)]
    pub struct BigUInteger {
        pub number: Vec<i32>,
    }

    impl BigUInteger {
        pub fn from_str(number_string: &str) -> BigUInteger {
            let mut number_vector: Vec<i32> = Vec::new();
            let base_dig = f64::from(BASE).log10() as usize;
            // Backward version compatibility, siome of online judger doesn't support step_by yet
            // - .step_by(base_dig)
            // + .filter(|x| (number_string.len() - x) % base_dig == 0)
            //#[allow(clippy::range_plus_one)]
            for i in (base_dig..number_string.len() + 1)
                .rev()
                .filter(|x| (number_string.len() - x) % base_dig == 0)
            {
                let temp = &number_string[i - base_dig..i].parse::<i32>().unwrap();
                number_vector.push(*temp);
            }
            if number_string.len() % base_dig != 0 {
                let temp = &number_string[..number_string.len() % base_dig]
                    .parse::<i32>()
                    .unwrap();
                number_vector.push(*temp);
            }
            BigUInteger {
                number: number_vector.iter().rev().cloned().collect::<Vec<i32>>(),
            }
        }

        /// Convertion from BigInteger to BigUInteger by dropping the sign
        #[allow(non_snake_case)]
        #[inline]
        fn from_BigInteger(big_integer: BigInteger) -> BigUInteger {
            BigUInteger {
                number: big_integer.number,
            }
        }

        fn to_string(&self) -> String {
            let mut output_string: String = self
                .number
                .iter()
                .map(|&x| format!("{:03}", x))
                .collect::<Vec<String>>()
                .join("");
            while &output_string[0..1] == "0" && output_string.len() > 1 {
                output_string.remove(0);
            }
            output_string
        }
    }

    //#[allow(clippy::suspicious_arithmetic_impl)]
    impl std::ops::Sub for BigUInteger {
        type Output = BigUInteger;

        fn sub(self, other: BigUInteger) -> BigUInteger {
            // Implemented from: https://www.geeksforgeeks.org/difference-of-two-large-numbers/
            let mut other_ = BigUInteger { ..other };
            let mut self_ = BigUInteger { ..self };
            if self_.number.len() < other_.number.len() {
                std::mem::swap(&mut self_, &mut other_);
            }
            let mut number_output: Vec<i32> = Vec::new();
            let digit_difference = self_.number.len() - other_.number.len();
            let mut carry: i32 = 0;
            for i in (0..other_.number.len()).rev() {
                let mut sub: i32 = self_.number[i + digit_difference] - other_.number[i] - carry;
                if sub < 0 {
                    sub += BASE;
                    carry = 1;
                } else {
                    carry = 0;
                }
                number_output.push(sub);
            }
            for i in (0..self_.number.len() - other_.number.len()).rev() {
                if self_.number[i] == 0 && carry != 0 {
                    number_output.push(BASE - 1);
                    continue;
                }
                let sub = self_.number[i] - carry;
                if i > 0 || sub > 0 {
                    number_output.push(sub);
                }
                carry = 0;
            }
            BigUInteger {
                number: number_output.iter().rev().cloned().collect::<Vec<i32>>(),
            }
        }
    }

    //#[allow(clippy::suspicious_arithmetic_impl)]
    impl std::ops::Add for BigUInteger {
        type Output = BigUInteger;

        fn add(self, other: BigUInteger) -> BigUInteger {
            // Implemented from https://www.geeksforgeeks.org/sum-two-large-numbers/
            let mut other_ = BigUInteger { ..other };
            let mut self_ = BigUInteger { ..self };
            if self_.number.len() > other_.number.len() {
                std::mem::swap(&mut self_, &mut other_);
            }
            let mut number_output: Vec<i32> = Vec::new();
            let digit_difference = other_.number.len() - self_.number.len();
            let mut carry: i32 = 0;
            for i in (0..self_.number.len()).rev() {
                let sum = self_.number[i] + other_.number[i + digit_difference] + carry;
                number_output.push(sum % BASE);
                carry = sum / BASE;
            }
            for i in (0..other_.number.len() - self_.number.len()).rev() {
                let sum = other_.number[i] + carry;
                number_output.push(sum % BASE);
                carry = sum / BASE;
            }
            if carry != 0 {
                number_output.push(carry);
            }
            BigUInteger {
                number: number_output.iter().rev().cloned().collect::<Vec<i32>>(),
            }
        }
    }

    impl std::cmp::Eq for BigUInteger {}

    impl std::cmp::PartialEq for BigUInteger {
        fn eq(&self, other: &BigUInteger) -> bool {
            self.number == other.number
        }
    }

    impl std::cmp::Ord for BigUInteger {
        fn cmp(&self, other: &BigUInteger) -> std::cmp::Ordering {
            if self.number.len() < other.number.len() {
                std::cmp::Ordering::Less
            } else if self.number.len() > other.number.len() {
                std::cmp::Ordering::Greater
            } else {
                for (self_i, other_i) in self.number.iter().zip(&other.number) {
                    if self_i < other_i {
                        return std::cmp::Ordering::Less;
                    } else if self_i > other_i {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            }
        }
    }

    impl std::cmp::PartialOrd for BigUInteger {
        fn partial_cmp(&self, other: &BigUInteger) -> std::option::Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::fmt::Display for BigUInteger {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }
}
