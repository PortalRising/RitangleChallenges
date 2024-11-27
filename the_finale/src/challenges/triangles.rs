use itertools::Itertools;

use crate::{
    // lookup_tables::pythagorean_triples::PythagoreanTriplesTable,
    lookup_tables::pythagorean_triples::PythagoreanTriplesTable,
    puzzle::{
        position::{GridPosition, IdentifierVector, PuzzleDirection, PuzzleIdentifier},
        wall::WallDirection,
        Puzzle,
    },
    rules::{
        executor::RuleExecutor,
        triangles::{
            pythagorean_triangle::PythagoreanTriangleRule, square::SumSquare,
            triangle_trio::OddCompositeCoprimeRule,
        },
    },
};

pub struct TrianglesChallenge;

impl TrianglesChallenge {
    /// Compute the solution to the triangle problem
    pub fn compute() {
        // Create a new puzzle with the given rules
        let puzzle = Puzzle::new(
            6,
            4,
            vec![
                (PuzzleIdentifier(1), GridPosition::new(0, 0)),
                (PuzzleIdentifier(2), GridPosition::new(2, 0)),
                (PuzzleIdentifier(3), GridPosition::new(3, 0)),
                (PuzzleIdentifier(4), GridPosition::new(4, 0)),
                (PuzzleIdentifier(5), GridPosition::new(5, 0)),
                (PuzzleIdentifier(6), GridPosition::new(0, 1)),
                (PuzzleIdentifier(7), GridPosition::new(1, 1)),
                (PuzzleIdentifier(8), GridPosition::new(3, 1)),
                (PuzzleIdentifier(9), GridPosition::new(4, 1)),
                (PuzzleIdentifier(10), GridPosition::new(0, 2)),
                (PuzzleIdentifier(11), GridPosition::new(2, 2)),
                (PuzzleIdentifier(12), GridPosition::new(5, 2)),
                (PuzzleIdentifier(13), GridPosition::new(0, 3)),
                (PuzzleIdentifier(14), GridPosition::new(3, 3)),
            ],
            vec![
                (GridPosition::new(1, 0), WallDirection::Both),
                (GridPosition::new(3, 0), WallDirection::Right),
                (GridPosition::new(4, 0), WallDirection::Down),
                (GridPosition::new(0, 1), WallDirection::Down),
                (GridPosition::new(2, 1), WallDirection::Right),
                (GridPosition::new(5, 1), WallDirection::Down),
                (GridPosition::new(0, 2), WallDirection::Right),
                (GridPosition::new(1, 2), WallDirection::Right),
                (GridPosition::new(2, 2), WallDirection::Down),
                (GridPosition::new(3, 2), WallDirection::Both),
                (GridPosition::new(4, 2), WallDirection::Right),
                (GridPosition::new(2, 3), WallDirection::Right),
            ],
        );

        let mut rule_executor = RuleExecutor::new(
            puzzle,
            vec![
                &OddCompositeCoprimeRule([
                    IdentifierVector {
                        identifier: PuzzleIdentifier(1),
                        direction: PuzzleDirection::Across,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(2),
                        direction: PuzzleDirection::Across,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(4),
                        direction: PuzzleDirection::Across,
                    },
                ]),
                &PythagoreanTriangleRule([
                    IdentifierVector {
                        identifier: PuzzleIdentifier(11),
                        direction: PuzzleDirection::Across,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(10),
                        direction: PuzzleDirection::Down,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(12),
                        direction: PuzzleDirection::Down,
                    },
                ]),
                &PythagoreanTriangleRule([
                    IdentifierVector {
                        identifier: PuzzleIdentifier(13),
                        direction: PuzzleDirection::Across,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(7),
                        direction: PuzzleDirection::Down,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(1),
                        direction: PuzzleDirection::Down,
                    },
                ]),
                &PythagoreanTriangleRule([
                    IdentifierVector {
                        identifier: PuzzleIdentifier(14),
                        direction: PuzzleDirection::Across,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(9),
                        direction: PuzzleDirection::Down,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(5),
                        direction: PuzzleDirection::Down,
                    },
                ]),
                &SumSquare([
                    IdentifierVector {
                        identifier: PuzzleIdentifier(2),
                        direction: PuzzleDirection::Down,
                    },
                    IdentifierVector {
                        identifier: PuzzleIdentifier(3),
                        direction: PuzzleDirection::Down,
                    },
                ]),
            ],
        );

        println!("Triples: {}", PythagoreanTriplesTable::triples().len());

        let valid_puzzles = rule_executor.compute();

        println!("Total: {}", valid_puzzles.len());
        let perimeter_vectors = [
            IdentifierVector {
                identifier: PuzzleIdentifier(1),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(2),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(4),
                direction: PuzzleDirection::Across,
            },
        ];

        let across_clues = [
            IdentifierVector {
                identifier: PuzzleIdentifier(1),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(2),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(4),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(6),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(8),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(11),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(13),
                direction: PuzzleDirection::Across,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(14),
                direction: PuzzleDirection::Across,
            },
        ];

        let down_clues = [
            IdentifierVector {
                identifier: PuzzleIdentifier(10),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(12),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(7),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(1),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(9),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(5),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(2),
                direction: PuzzleDirection::Down,
            },
            IdentifierVector {
                identifier: PuzzleIdentifier(3),
                direction: PuzzleDirection::Down,
            },
        ];

        // Go through every valid puzzle and get unique perimeters
        let mut unique_perimeters = hashbrown::HashSet::new();
        for puzzle in valid_puzzles {
            let perimeter: usize = perimeter_vectors
                .iter()
                .map(|&vector| {
                    puzzle
                        .number_at(vector)
                        .expect("This is a complete puzzle, the number should exist")
                })
                .sum();

            unique_perimeters.insert(perimeter);

            let across_sum: usize = across_clues
                .iter()
                .map(|&vector| (vector, puzzle.number_at(vector).unwrap()))
                .map(|(vector, num)| {
                    println!("{:?}: {}", vector, num);
                    num
                })
                .sum();

            let down_sum: usize = down_clues
                .iter()
                .map(|&vector| (vector, puzzle.number_at(vector).unwrap()))
                .map(|(vector, num)| {
                    println!("{:?}: {}", vector, num);
                    num
                })
                .sum();

            if across_sum.abs_diff(down_sum) == perimeter {
                println!("Hello");
            }
            println!(
                "A{} - D{} = {}",
                across_sum,
                down_sum,
                across_sum.abs_diff(down_sum)
            );
            println!("{}", perimeter);

            println!("{}", puzzle);
            println!();
        }

        // If there are two perimeters get the difference
        // if unique_perimeters.len() == 2 {
        //     let (perimeters_a, perimeter_b) =
        //         unique_perimeters.into_iter().collect_tuple().unwrap();

        //     println!(
        //         "Perimeter difference, {}",
        //         perimeters_a.abs_diff(perimeter_b)
        //     );
        // } else {
        //     println!(
        //         "{} unique perimeters, too few/many to consolidate a final answer, we need two",
        //         unique_perimeters.len()
        //     )
        // }
    }
}
