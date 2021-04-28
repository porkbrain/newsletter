use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::svm::{svr::SVR, RBFKernel};

#[allow(dead_code)]
pub type SVM = SVR<f64, DenseMatrix<f64>, RBFKernel<f64>>;

#[allow(dead_code)]
pub type Feature = Vec<f64>;
