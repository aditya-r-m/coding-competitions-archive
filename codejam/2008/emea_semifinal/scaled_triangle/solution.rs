use lib::collections::SquareMatrix;

lib::run!();

fn read() -> [[f64; 6]; 2] {
    lib::input!([f64; 6] as large_triangle);
    lib::input!([f64; 6] as small_triangle);
    [large_triangle, small_triangle]
}

fn distance([(x0, y0), (x1, y1)]: [(f64, f64); 2]) -> f64 {
    ((x1 - x0).powf(2.) + (y1 - y0).powf(2.)).sqrt()
}

fn parse_edges(
    [large_triangle, small_triangle]: [[f64; 6]; 2],
) -> ([(f64, f64); 2], [(f64, f64); 2]) {
    let mut max_distance = 0f64;
    let mut large_edge = [(0f64, 0f64); 2];
    let mut small_edge = [(0f64, 0f64); 2];
    for i in 0..3 {
        for j in i + 1..3 {
            let cur_distance = distance([
                (large_triangle[2 * i], large_triangle[2 * i + 1]),
                (large_triangle[2 * j], large_triangle[2 * j + 1]),
            ]);
            if max_distance < cur_distance {
                max_distance = cur_distance;
                large_edge = [
                    (large_triangle[2 * i], large_triangle[2 * i + 1]),
                    (large_triangle[2 * j], large_triangle[2 * j + 1]),
                ];
                small_edge = [
                    (small_triangle[2 * i], small_triangle[2 * i + 1]),
                    (small_triangle[2 * j], small_triangle[2 * j + 1]),
                ];
            }
        }
    }
    (large_edge, small_edge)
}

fn solve(triangles: [[f64; 6]; 2]) -> String {
    let (large_edge, small_edge) = parse_edges(triangles);
    let translation_initial_matrix: SquareMatrix<f64> = SquareMatrix::new(vec![
        vec![1., 0., -large_edge[0].0],
        vec![0., 1., -large_edge[0].1],
        vec![0., 0., 1.],
    ]);
    let scaling_factor = distance(small_edge) / distance(large_edge);
    let scaling_matrix: SquareMatrix<f64> = SquareMatrix::new(vec![
        vec![scaling_factor, 0., 0.],
        vec![0., scaling_factor, 0.],
        vec![0., 0., 1.],
    ]);
    let theta = (small_edge[1].1 - small_edge[0].1).atan2(small_edge[1].0 - small_edge[0].0)
        - (large_edge[1].1 - large_edge[0].1).atan2(large_edge[1].0 - large_edge[0].0);
    let rotation_matrix: SquareMatrix<f64> = SquareMatrix::new(vec![
        vec![theta.cos(), -theta.sin(), 0.],
        vec![theta.sin(), theta.cos(), 0.],
        vec![0., 0., 1.],
    ]);
    let translation_final_matrix: SquareMatrix<f64> = SquareMatrix::new(vec![
        vec![1., 0., small_edge[0].0],
        vec![0., 1., small_edge[0].1],
        vec![0., 0., 1.],
    ]);
    let mut transformation_matrix =
        translation_final_matrix * scaling_matrix * rotation_matrix * translation_initial_matrix;
    for _ in 0..64 {
        transformation_matrix = transformation_matrix.clone() * transformation_matrix;
    }
    let fixed_point = transformation_matrix * vec![large_edge[0].0, large_edge[0].1, 1.];
    format!("{} {}", fixed_point[0], fixed_point[1])
}
