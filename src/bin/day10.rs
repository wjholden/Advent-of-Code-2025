use std::collections::HashSet;

use advent_of_code_2025::*;
use nalgebra::{DMatrix, DVector};
use pumpkin_solver::{
    ConstraintOperationError, DefaultBrancher, constraints,
    optimisation::{OptimisationDirection, linear_sat_unsat::LinearSatUnsat},
    results::{OptimisationResult, ProblemSolution, SolutionReference},
    termination::Indefinite,
};

pub const PUZZLE: &str = include_str!("../../puzzles/day10.txt");

/// Part 2:
/// ```julia
/// A1 = [0 0 0 0 1 1; 0 1 0 0 0 1; 0 0 1 1 1 0; 1 1 0 1 0 0]
/// x1 = [1; 3; 0; 3; 1; 2]
/// y1 = [3; 5; 4; 7]
/// @assert A1 * x1 == y1
///
/// A2 = [1 0 1 1 0; 0 0 0 1 1; 1 1 0 1 1; 1 1 0 0 1; 1 0 1 0 1]
/// x2 = [2; 5; 0; 5; 0]
/// y2 = [7; 5; 12; 7; 2]
/// @assert A2 * x2 == y2
///
/// A3 = [1 1 1 0; 1 0 1 1; 1 0 1 1; 1 1 0 0; 1 1 1 0; 0 0 1 0]
/// x3 = [5; 0; 5; 1]
/// y3 = [10; 11; 11; 5; 10; 5]
/// @assert A3 * x3 == y3
/// ```
///
/// So Minizinc. I've built some Minizinc files that I'm proud of, but I've
/// learned that Zelen isn't yet fast or capable enough to compete with Gecode.
///
/// This program outputs to the mzn/ directory.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
struct Machine {
    goal: DVector<usize>,
    components: DMatrix<usize>,
    joltages: DVector<usize>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let words: Vec<&str> = input.split_ascii_whitespace().collect();
        let lights = words[0].len() - 2;
        let schematics = words.len() - 2;
        let goal = DVector::from_iterator(
            lights,
            words[0].chars().skip(1).take(lights).map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("symbol other than '.' or '#' in a light"),
            }),
        );
        let components = DMatrix::from_fn(lights, schematics, |i, j| {
            if words[j + 1].contains(format!("{i}").as_str()) {
                1
            } else {
                0
            }
        });
        let joltages = words[words.len() - 1].replace("{", "").replace("}", "");
        let joltages = DVector::from_iterator(
            lights,
            joltages.split(",").map(|s| s.parse().expect("joltage")),
        );
        Self {
            goal,
            components,
            joltages,
        }
    }

    fn part1(&self) -> usize {
        let n = self.components.ncols();
        let mut x = DVector::zeros(n);
        let mut solutions = HashSet::new();
        'outer: for i in 0..(2usize.pow(n as u32)) {
            for j in 0..n {
                x[j] = (i >> j) & 1;
            }
            let product = &self.components * &x;
            for (&a, &b) in product.iter().zip(self.goal.iter()) {
                // Candidate solution if oddness/evenness matches.
                if (a & 1) != b {
                    continue 'outer;
                }
            }
            solutions.insert(x.iter().sum());
        }
        solutions
            .into_iter()
            .min()
            .expect("minimum number of buttons to construct goal")
    }

    #[cfg(feature = "zelen")]
    /// Not used in the actual solution because we invoke Minizinc instead.
    fn part2_zelen(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let ast = zelen::parse(&self.constraints())?;
        let model_data = Translator::translate_with_vars(&ast)?;
        let objective = model_data.objective_var.ok_or("no objective")?;
        // You don't want model.solve() here -- that will give you any solution
        // that satisfies the constraints, not necessarily the best solution
        // towards your objective.
        //
        // The issue with this library is right here. If I could figure out a
        // way to increase the timeout, maybe we could find the objective.
        // Currently, the solver times out and we never get past some inputs.
        let solution = model_data.model.minimize(objective)?;
        Ok(solution.get_int(objective))
    }

    #[cfg(feature = "pumpkin")]
    fn part2_pumpkin(&self) -> Result<i32, ConstraintOperationError> {
        let mut solver = pumpkin_solver::Solver::default();

        let m = self.components.nrows(); // number of constraints
        let n = self.components.ncols(); // number of variables
        assert_eq!(m, self.joltages.len());

        // println!("{}", self.components);

        let variables: Vec<_> = (0..n).map(|_| solver.new_bounded_integer(0, 300)).collect();

        // joltage[j] = sum(variable[i] if components[i,j] == 1)
        for i in 0..m {
            // Find variables present in this constraint.
            let vars: Vec<_> = (0..n)
                .filter_map(|j| match self.components[(i, j)] {
                    1 => Some(variables[j]),
                    0 => None,
                    _ => panic!("non-binary value in components matrix"),
                })
                .collect();
            // println!("sum of {vars:?} = {}", self.joltages[i]);
            let tag = solver.new_constraint_tag();
            solver
                .add_constraint(constraints::equals(vars, self.joltages[i] as i32, tag))
                .post()?;
        }

        // This is the only way to express x[i] + x[j] = x[k] in Pumpkin.
        // The constraints::equals function expects an i32 as the second arg.
        let objective = (1..n).fold(variables[0], |a, i| {
            let b = variables[i];
            let c = solver.new_bounded_integer(0, 1000);
            let tag = solver.new_constraint_tag();
            solver
                .add_constraint(constraints::plus(a, b, c, tag))
                .post()
                .unwrap();
            c
        });

        let mut termination = Indefinite;
        let mut brancher = solver.default_brancher();

        let callback: fn(&pumpkin_solver::Solver, SolutionReference, &DefaultBrancher) =
            |_, _, _| {};
        let result = solver.optimise(
            &mut brancher,
            &mut termination,
            LinearSatUnsat::new(OptimisationDirection::Minimise, objective, callback),
        );

        if let OptimisationResult::Optimal(solution) = result {
            return Ok(solution.get_integer_value(objective));
        } else {
            panic!("failed to find a solution");
        }
    }

    #[cfg(not(feature = "pumpkin"))]
    fn constraints(&self) -> String {
        let mut s = String::new();
        let m = self.joltages.len();
        assert_eq!(m, self.components.nrows());
        let n = self.components.ncols();
        // Yeah, it's not very general. There aren't more than 14 sets of
        // schematics per machine.
        let variables = &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n",
        ][0..n];
        for var in variables {
            // Zelen doesn't like the "0.." syntax. Zelen needs an explicit
            // upper bound.
            s.push_str(format!("var 0..300: {var};\n").as_str());
        }
        // The solver will work with "var int: z", but it should speed things
        // up a little to add the lower bound.
        //
        // Actually, you can speed things up a lot if you add an upper bound,
        // but with a loss of generality.
        s.push_str("var int: z;\n");
        for i in 0..m {
            let vars: Vec<&str> = (0..n)
                .filter_map(|j| {
                    if self.components[(i, j)] == 1 {
                        Some(variables[j])
                    } else {
                        None
                    }
                })
                .collect();
            s.push_str(
                format!("constraint {} = {};\n", self.joltages[i], vars.join(" + ")).as_str(),
            );
        }
        let sum = variables.join(" + ");
        s.push_str(format!("constraint z = {sum};\n").as_str());
        s.push_str("solve minimize z;\n");
        s.push_str(r#"output show(z);"#);
        s
    }
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    machines: Vec<Machine>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        instance.machines = input.lines().map(Machine::new).collect();
        instance
    }

    fn solve(mut self) -> Self {
        self.part1 = self.machines.iter().map(Machine::part1).sum();
        self.part2 = self.part2();

        // for (_i, machine) in self.machines.iter().enumerate() {
        //     self.part2 += machine._part2().unwrap() as usize;
        //     self._pumpkin += machine._part2_pumpkin().unwrap();
        // }
        self
    }
}

impl Puzzle {
    #[cfg(not(feature = "pumpkin"))]
    #[cfg(not(feature = "zelen"))]
    fn part2(&self) -> usize {
        match fs::create_dir("mzn") {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(()), // suppress error if directory already exists
            Err(other) => Err(other),
        }
        .unwrap();

        for (i, machine) in self.machines.iter().enumerate() {
            let filename = format!("mzn/{i:0>3}.mzn");
            fs::write(filename, machine.constraints()).expect("should write MZN file to disk");
        }

        let command = r#"
(
Get-ChildItem -Path mzn/ |
ForEach-Object -Parallel { & minizinc.exe $_.FullName --solution-separator "" } |
Select-String -NotMatch "==========" |
ForEach-Object { [int]"$($_)" } |
Measure-Object -Sum
).Sum
    "#;
        let output = Command::new("pwsh")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-NoLogo",
                "-Command",
                command,
            ])
            .output()
            .expect("PowerShell 7 pipeline to Minizinc");
        let stdout = String::from_utf8(output.stdout).expect("standard output from command");
        let stdout = stdout.trim();
        self.part2 = stdout.parse().unwrap();

        fs::remove_dir_all("mzn").expect("cleanup temporary files");
    }

    #[cfg(feature = "zelen")]
    fn part2(&self) -> usize {
        self.machines.map(machine::part2).sum()
    }

    #[cfg(feature = "pumpkin")]
    fn part2(&self) -> usize {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        self.machines
            .par_iter()
            .map(Machine::part2_pumpkin)
            .map(|v| v.unwrap())
            .sum::<i32>() as usize
    }
}

#[cfg(test)]
mod factory {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day10.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 33);
    }
}
