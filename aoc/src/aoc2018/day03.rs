use crate::{error, Solution};
use pest::{self, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "aoc2018/day03.pest"]
struct Day03Parser;

#[derive(Debug, Clone)]
struct Claim<'a> {
    id: &'a str,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

type Cloth = HashMap<(u32, u32), u32>;

pub fn run(input: &str) -> error::Result<Solution> {
    let claims = parse(&input)?;

    if claims.is_empty() {
        Err(anyhow::anyhow!(&"No claims parsed from input"))?
    }

    let cloth = create_cloth(&claims);

    Ok(Solution {
        part_one: count_overlapping_claims(&cloth).to_string(),
        part_two: find_single_claim(&claims, &cloth).unwrap().to_string(),
    })
}

fn create_cloth(claims: &[Claim]) -> Cloth {
    let mut cloth: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in claims.iter() {
        let x_range = claim.left..(claim.width + claim.left);
        let y_range = claim.top..(claim.height + claim.top);

        for x in x_range {
            for y in y_range.clone() {
                let value = cloth.entry((x, y)).or_insert(0);

                *value += 1;
            }
        }
    }

    cloth
}

fn count_overlapping_claims(cloth: &Cloth) -> u32 {
    cloth.iter().filter(|&(_, &v)| v > 1).count() as u32
}

fn find_single_claim<'a>(claims: &'a [Claim], cloth: &Cloth) -> Option<&'a str> {
    let mut claim_id: Option<&str> = None;

    for claim in claims.iter() {
        let mut overlapping = false;
        let x_range = claim.left..(claim.width + claim.left);
        let y_range = claim.top..(claim.height + claim.top);
        for x in x_range {
            for y in y_range.clone() {
                let cood = (x, y);
                let value = cloth.get(&cood);

                if let Some(v) = value {
                    if *v > 1 {
                        overlapping = true;
                        break;
                    }
                }
            }
        }

        if !overlapping {
            claim_id = Some(claim.id);
            break;
        }
    }

    claim_id
}

fn parse(input: &str) -> error::Result<Vec<Claim>> {
    let mut claims = vec![];

    let files = Day03Parser::parse(Rule::file, input).map_err(|err| anyhow::anyhow!(err))?;

    for file in files {
        for record in file.into_inner() {
            if let Rule::record = record.as_rule() {
                let mut record_pair = record.into_inner();

                // if the pest libary is to be believed, it is impossible
                // for these Results to be Result::Err at this point
                claims.push(Claim {
                    id: record_pair.next().unwrap().as_str(),
                    left: record_pair.next().unwrap().as_str().parse()?,
                    top: record_pair.next().unwrap().as_str().parse()?,
                    width: record_pair.next().unwrap().as_str().parse()?,
                    height: record_pair.next().unwrap().as_str().parse()?,
                });
            }
        }
    }

    Ok(claims)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn original_example() {
        let s = r#"
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
        "#;

        let claims = parse(s).unwrap();

        assert_eq!(claims[0].id, "1");
        assert_eq!(claims[0].left, 1);
        assert_eq!(claims[0].top, 3);
        assert_eq!(claims[0].width, 4);
        assert_eq!(claims[0].height, 4);

        assert_eq!(claims[1].id, "2");
        assert_eq!(claims[1].left, 3);
        assert_eq!(claims[1].top, 1);
        assert_eq!(claims[1].width, 4);
        assert_eq!(claims[1].height, 4);

        assert_eq!(claims[2].id, "3");
        assert_eq!(claims[2].left, 5);
        assert_eq!(claims[2].top, 5);
        assert_eq!(claims[2].width, 2);
        assert_eq!(claims[2].height, 2);

        let cloth = create_cloth(&claims);

        assert_eq!(count_overlapping_claims(&cloth), 4);
        assert_eq!(find_single_claim(&claims, &cloth), Some("3"));
    }
}
