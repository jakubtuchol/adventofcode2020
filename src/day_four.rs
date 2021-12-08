use std::collections::HashSet;

pub fn check_valid_passports(records: &str) -> usize {
    records
        .split("\n\n")
        .filter(|&record| check_record(record))
        .count()
}

fn check_record(record: &str) -> bool {
    let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .into_iter()
        .collect();
    let data: Vec<&str> = record
        .split_ascii_whitespace()
        .map(|data| data.split(":").nth(0).unwrap())
        .collect();

    let mut cid = false;
    for &d in data.iter() {
        if d == "cid" {
            cid = true;
        }
        if !required.contains(d) {
            return false;
        }
    }
    if cid {
        return data.len() == 8;
    }
    data.len() == 7
}

#[cfg(test)]
mod test {
    use super::check_valid_passports;

    #[test]
    fn test_valid_passports() {
        let records = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(2, check_valid_passports(records));
    }
}
