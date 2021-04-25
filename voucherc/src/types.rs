use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::svm::{svc::SVC, RBFKernel};

#[allow(dead_code)]
pub type SVM = SVC<f64, DenseMatrix<f64>, RBFKernel<f64>>;

#[allow(dead_code)]
pub type Feature = Vec<f64>;
