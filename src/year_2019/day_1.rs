use crate::SolverError;

/*
Fuel required to launch a given module is based on its mass. Specifically, 
to find the fuel required for a module, take its mass, divide by three, 
round down, and subtract 2.

The Fuel Counter-Upper needs to know the total fuel requirement. To find 
it, individually calculate the fuel needed for the mass of each module 
(your puzzle input), then add together all the fuel values.
*/

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    
    for line in input.lines() {
        let mass = line.trim().parse().unwrap();
        part_1_sum += calc_fuel(mass);
        part_2_sum += calc_fuel_recursive(mass);
    }

    Ok((part_1_sum.to_string(), part_2_sum.to_string()))
}

fn calc_fuel(mass: i64) -> i64 {
    // Remove remainder to make rounding down unnecessary.
    let mass = mass - (mass % 3);
    mass / 3 - 2
}

fn calc_fuel_recursive(mass: i64) -> i64 {
    let mut fuel_total = 0;
    let mut mass = mass;

    while mass > 0 {
        let fuel = calc_fuel(mass);
        mass = fuel;
        fuel_total += fuel.max(0);
    }

    fuel_total
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
    */
    #[test]
    fn correctly_calculate_fuel() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    /*
    A module of mass 14 requires 2 fuel. This fuel requires no further 
    fuel (2 divided by 3 and rounded down is 0, which would call for a 
    negative fuel), so the total fuel required is still just 2.

    At first, a module of mass 1969 requires 654 fuel. Then, this fuel 
    requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, 
    which requires 21 fuel, which requires 5 fuel, which requires 
    no further fuel. So, the total fuel required for a module of mass 1969 
    is 654 + 216 + 70 + 21 + 5 = 966.

    The fuel required by a module of mass 100756 and its fuel is: 
    33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    */
    #[test]
    fn correctly_calculate_fuel_recursively() {
        assert_eq!(calc_fuel_recursive(14), 2);
        assert_eq!(calc_fuel_recursive(1969), 966);
        assert_eq!(calc_fuel_recursive(100756), 50346);
    }
}