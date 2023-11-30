pub fn percentage(target_percentage: f64, min: f64, max: f64) -> f64 {
    // MappedValue= [(x-a)/(b-a)]⋅(d−c)+c
    let x = target_percentage;
    // let a = 0.0;
    let b = 100.0;
    let c = min;
    let d = max;
    // ((x - a) / (b - a)) * (d - c) + c
    (x / b) * (d - c) + c //simplified a = 0
}

pub fn find_min_value(matrix: &Vec<Vec<f64>>) -> Option<f64> {
    // Ensure the matrix is not empty
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }

    let mut min_value = matrix[0][0];

    for row in matrix {
        for &value in row {
            if value < min_value {
                min_value = value;
            }
        }
    }

    Some(min_value)
}

pub fn find_max_value(matix: &Vec<Vec<f64>>) -> Option<f64> {
    // Ensure the matrix is not empty
    if matix.is_empty() || matix[0].is_empty() {
        return None;
    }

    let mut max_value = matix[0][0];

    for row in matix {
        for &value in row {
            if value > max_value {
                max_value = value;
            }
        }
    }

    Some(max_value)
}

pub fn map_value_to_range(value: f64, from: std::ops::Range<f64>, to: std::ops::Range<f64>) -> f64 {
    let from_min = from.start;
    let from_max = from.end;
    let to_min = to.start;
    let to_max = to.end;

    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}