use std::collections::HashMap;
use std::io;

macro_rules! read_int(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        line.trim().parse::<usize>().expect("Something went wrong when parsing the input to an usize value")
    }}
);

macro_rules! read_tuple(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        let v: Vec<u32> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let lived_years: (u32, u32) = (v[0], v[1]);
        lived_years
    }}
);

fn main() {
    let number_lines = read_int!();
    let _number_columns = read_int!();

    let mut population_info = Vec::new();
    for _ in 0..number_lines {
        population_info.push(read_tuple!());
    }

    println!("{}", find_highest_population_year(population_info));
}

fn find_highest_population_year(population_info: Vec<(u32, u32)>) -> u32 {
    let mut count_population_by_year: HashMap<u32, u32> = HashMap::new();
    for (born_year, death_year) in population_info {
        for lived_year in born_year..=death_year {
            count_population_by_year
                .entry(lived_year)
                .and_modify(|population_count| *population_count += 1)
                .or_insert(1);
        }
    }

    let max_population_count = count_population_by_year
        .iter()
        .max_by(|year, year_to_compare| year.1.cmp(&year_to_compare.1))
        .map(|(_year, population_count)| population_count)
        .expect("It couldn't find the year");

    let filtered_years = count_population_by_year
        .iter()
        .filter(|(_year, population_count)| *population_count == max_population_count)
        .collect::<HashMap<_, _>>();
    let earliest_year = filtered_years
        .iter()
        .min_by(|year, year_to_compare| year.0.cmp(&year_to_compare.0))
        .map(|(year, _population_count)| *year)
        .expect("It couldn't find the year");
    *earliest_year
}

#[cfg(test)]
mod tests {
    use crate::find_highest_population_year;

    #[test]
    fn test_case_1() {
        let mut population_info = Vec::new();
        population_info.push((1830, 1842));
        population_info.push((1810, 1845));
        population_info.push((1805, 1835));
        population_info.push((1844, 1885));
        println!("{}", find_highest_population_year(population_info));
    }
}
