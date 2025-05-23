// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0


#[defines_primitive(u8)]
module std::u8 {
    /// Return the larger of `x` and `y`
    public fun max(x: u8, y: u8): u8 {
        if (x > y) {
            x
        } else {
            y
        }
    }

    /// Return the smaller of `x` and `y`
    public fun min(x: u8, y: u8): u8 {
        if (x < y) {
            x
        } else {
            y
        }
    }

    /// Return the absolute value of x - y
    public fun diff(x: u8, y: u8): u8 {
        if (x > y) {
            x - y
        } else {
            y - x
        }
    }

    /// Calculate x / y, but round up the result.
    public fun divide_and_round_up(x: u8, y: u8): u8 {
        if (x % y == 0) {
            x / y
        } else {
            x / y + 1
        }
    }

    /// Returns x * y / z with as little loss of precision as possible and avoid overflow
    public fun multiple_and_divide(x: u8, y: u8, z: u8): u8 {
        if (y == z) {
            return x
        };
        if (x == z) {
            return y
        };

        let a = x / z;
        let b = x % z;
        let c = y / z;
        let d = y % z;
        let res = a * c * z + a * d + b * c + b * d / z;

        res
    }

    /// Return the value of a base raised to a power
    public fun pow(base: u8, exponent: u8): u8 {
        let res = 1;
        while (exponent >= 1) {
            if (exponent % 2 == 0) {
                base = base * base;
                exponent = exponent / 2;
            } else {
                res = res * base;
                exponent = exponent - 1;
            };
        };

        res
    }

    /// Get a nearest lower integer Square Root for `x`. Given that this
    /// function can only operate with integers, it is impossible
    /// to get perfect (or precise) integer square root for some numbers.
    ///
    /// Example:
    /// ```
    /// u8::sqrt(9) => 3
    /// u8::sqrt(8) => 2 // the nearest lower square root is 4;
    /// ```
    public fun sqrt(x: u8): u8 {
        let bit = 1u16 << 8;
        let res = 0u16;
        let x = (x as u16);

        while (bit != 0) {
            if (x >= res + bit) {
                x = x - (res + bit);
                res = (res >> 1) + bit;
            } else {
                res = res >> 1;
            };
            bit = bit >> 2;
        };

        (res as u8)
    }
}