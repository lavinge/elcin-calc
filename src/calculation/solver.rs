use super::error::Error;
use super::profile::Profile;
use crate::fertilizers::fertilizer::Fertiliser;
use ellp::{problem::VariableId, Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};
use std::collections::HashMap;

pub struct Solver<'a> {
    profile: &'a Profile<'a>,
    variables: HashMap<VariableId, &'a Fertiliser>,
    problem: Problem,
}

impl<'a> Solver<'a> {
    pub fn new(profile: &'a Profile<'a>) -> Result<Self, Vec<Error>> {
        let mut solver = Self {
            profile,
            variables: HashMap::new(),
            problem: Problem::new(),
        };

        solver.setup()?;

        Ok(solver)
    }

    fn setup(&mut self) -> Result<(), Vec<Error>> {
        let fertilizers = self.profile.fertilizers();

        fertilizers.for_each(|fertilizer| {
            let variable_name = format!("x{}", self.problem.variables.len() + 1);

            // цель вычисления - минимизировать количество удобрений, которое
            // потребуется для приготовления раствора, поэтому коэффициент всех переменных
            // равен 1

            // переменные могут иметь только положительные значения,
            // потому что нельзя удалить удобрение из раствора,
            // поэтому для всех переменных указана нижняя граница значения 0
            let variable_id = self
                .problem
                .add_var(1., Bound::Lower(0.), Some(variable_name.clone()))
                .unwrap();

            self.variables.insert(variable_id, fertilizer);
        });

        let mut errors: Vec<Error> = Vec::new();

        self.profile
            .composition()
            .nutrients()
            .iter()
            .for_each(|value| {
                let mut coefficients: Vec<(VariableId, f64)> = Vec::new();

                self.variables.iter().for_each(|variable| {
                    if let Some(percent) = variable.1.percent_of(value.0) {
                        coefficients.push((*variable.0, percent.into()));
                    }
                });

                if coefficients.len() == 0 {
                    let message =
                        format!("\"{}\" does not covered by provided fertilizers", value.0);

                    errors.push(Error::new(message));
                }

                match value.0 {
                    "N" | "P" | "K" | "Mg" | "Ca" => {
                        self.problem
                            .add_constraint(coefficients, ConstraintOp::Eq, value.1.into())
                            .unwrap();
                    }
                    "S" => {
                        self.problem
                            .add_constraint(coefficients, ConstraintOp::Gte, 0.)
                            .unwrap();
                    }
                    "Fe" | "Mn" | "Zn" | "B" | "Cu" | "Mo" => {
                        self.problem
                            .add_constraint(coefficients, ConstraintOp::Eq, value.1.into())
                            .unwrap();
                    }
                    _ => {}
                };
            });

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn solve(&self, tank_size: i32) -> Result<(), Error> {
        println!("{}", self.problem);

        let result = DualSimplexSolver::default()
            .solve(self.problem.clone())
            .unwrap();

        if let SolverResult::Optimal(sol) = result {
            let result = sol.x();

            result.iter().enumerate().for_each(|(idx, amount)| {
                let variable_id = self.problem.variables.get(idx).unwrap().id;

                if let Some(fertilizer) = self.variables.get(&variable_id) {
                    println!(
                        "{:#?} amount = {}",
                        fertilizer.name(),
                        amount / 10. * tank_size as f64
                    );
                }
            });

            Ok(())
        } else {
            Err(Error::new("impossible profile".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculation::composition::Composition;
    use crate::calculation::profile::Profile;
    use crate::fertilizers::fertilizer::Fertiliser;
    use crate::formula::builder::Builder;

    use super::Solver;

    #[test]
    fn basic_nutrient_profile() {
        let formula_builder = Builder::new();

        let mut composition = Composition::new();
        composition.add_nutrient("N", 189.);
        composition.add_nutrient("P", 39.);
        composition.add_nutrient("K", 341.);
        composition.add_nutrient("Ca", 170.);
        composition.add_nutrient("Mg", 48.);
        composition.add_nutrient("S", 150.);
        composition.add_nutrient("Fe", 2.);
        composition.add_nutrient("Mn", 0.55);
        composition.add_nutrient("Zn", 0.33);
        composition.add_nutrient("B", 0.28);
        composition.add_nutrient("Cu", 0.05);
        composition.add_nutrient("Mo", 0.05);

        let mut profile = Profile::new(composition);

        profile.add_fertilizer(Fertiliser::from_formula(
            "calcium nitrate",
            "",
            formula_builder.build("Ca(NO3)2").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "pottasium nitrate",
            "",
            formula_builder.build("KNO3").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "ammonium nitrate",
            "",
            formula_builder.build("NH4NO3").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "magnesium sulfate",
            "",
            formula_builder.build("MgSO4*7H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "pottasium sulfate",
            "",
            formula_builder.build("K2SO4").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "monopottasium phosphate",
            "",
            formula_builder.build("KH2PO4").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "iron chelate",
            "",
            formula_builder.build("C14H18N3O10Fe(NH4)2").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "manganese sulfate",
            "",
            formula_builder.build("MnSO4*H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "boric acid",
            "",
            formula_builder.build("H3BO3").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "molibden acid",
            "",
            formula_builder.build("Na2MoO4*2H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "zink sulfate",
            "",
            formula_builder.build("ZnSO4*7H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "cuprum sulfate",
            "",
            formula_builder.build("CuSO4*5H2O").unwrap(),
        ));

        match Solver::new(&profile) {
            Ok(solver) => match solver.solve(1) {
                Ok(_) => {
                    println!("solved");
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            },
            Err(errors) => {
                println!("{:#?}", errors);
            }
        }
    }
}