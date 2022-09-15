use pgx::*;
use serde::Deserialize;

#[derive(PostgresEnum, Copy, Clone, Eq, PartialEq, Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Algorithm {
    linear,
    xgboost,
    svm,
    lasso,
    elastic_net,
    // ridge,
    // kmeans,
    // dbscan,
    // knn,
    // random_forest,
}

impl std::str::FromStr for Algorithm {
    type Err = ();

    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "linear" => Ok(Algorithm::linear),
            "xgboost" => Ok(Algorithm::xgboost),
            "svm" => Ok(Algorithm::svm),
            "lasso" => Ok(Algorithm::lasso),
            "elastic_net" => Ok(Algorithm::elastic_net),
            _ => Err(()),
        }
    }
}

impl std::string::ToString for Algorithm {
    fn to_string(&self) -> String {
        match *self {
            Algorithm::linear => "linear".to_string(),
            Algorithm::xgboost => "xgboost".to_string(),
            Algorithm::svm => "svm".to_string(),
            Algorithm::lasso => "lasso".to_string(),
            Algorithm::elastic_net => "elastic_net".to_string(),
        }
    }
}
