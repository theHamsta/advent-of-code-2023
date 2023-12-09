use convolutions_rs::{convolutions::ConvolutionLayer, Padding};
use itertools::Itertools;
use ndarray::*;

const EPS: f64 = 1e-3;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");

    let numbers = raw_input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect_vec())
        .collect::<Vec<Vec<i64>>>();
    let len = numbers.iter().map(|n| n.len()).collect_vec();

    let mut array = Array3::<f64>::default((1, numbers.len(), len[0]));
    for (i, mut row) in array.axis_iter_mut(Axis(1)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = numbers[i][j] as f64;
        }
    }

    let kernel: Array4<f64> = Array::from_shape_vec((1, 1, 1, 2), vec![-1., 1.]).unwrap();

    let conv_layer = ConvolutionLayer::new(kernel.clone(), None, 1, Padding::Valid);

    let mut pyramid = vec![array];
    while pyramid.last().unwrap().shape()[2] >= 2
        && !pyramid.last().unwrap().iter().all(|a| a.abs() < EPS)
    {
        dbg!(&pyramid.last().unwrap().shape());
        let next = conv_layer.convolve(pyramid.last().unwrap());
        pyramid.push(next);
    }

    let mut extrapolated = Array1::<f64>::default(numbers.len());

    for i in (0..pyramid.len()).rev() {
        let cur = &pyramid[i];
        let last_cur = cur.shape()[2] - 1;
        for j in 0..cur.shape()[1] {
            extrapolated[j] = extrapolated[j] + cur[(0, j, last_cur)];
        }
    }

    let part1: f64 = extrapolated.iter().sum();
    dbg!(&part1);

    let mut extrapolated = Array1::<f64>::default(numbers.len());

    for i in (0..pyramid.len()).rev() {
        let cur = &pyramid[i];
        for j in 0..cur.shape()[1] {
            extrapolated[j] = -extrapolated[j] + cur[(0, j, 0)];
        }
    }

    let part2: f64 = extrapolated.iter().sum();
    dbg!(&part2);

    Ok(())
}
