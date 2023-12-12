use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

use crate::generator::Coordinates;
use crate::utils::{Coordinate, Slice, slice_vec_2d};

// TODO doc street

#[derive(Debug, Eq, Hash)]
struct Edge {
    start: Coordinates,
    end: Coordinates,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end) || (self.start == other.end && self.end == other.start)
    }
}

pub(crate) fn street_spawn(street_quantity: usize, elevation_map: &[Vec<f64>], n_slice_side: usize, lower_threshold: f64) -> Vec<Vec<Coordinates>> {
    // get local maxima
    let mut local_maxima: Vec<Coordinates> = get_local_maxima(elevation_map, n_slice_side, lower_threshold);

    // combine near local maxima
    let combined_local_maxima: Vec<Coordinates> = combine_local_maxima(elevation_map, &mut local_maxima, n_slice_side, elevation_map.len() / 100);

    // get voronoi diagram
    let diagram = get_voronoi_diagram(elevation_map, &combined_local_maxima);

    // get unique edges extremes from diagram
    let unique_extremes: HashSet<Edge> = get_edges_extremes_from_diagram(diagram);

    // fix edges extremes
    let fixed_extremes = fix_extremes(unique_extremes, elevation_map.len() - 1);

    // trace the streets edges
    fixed_extremes.iter().map(|edge| connect_points(edge.start, edge.end)).collect()
}

#[inline(always)]
fn get_edges_extremes_from_diagram(diagram: VoronoiDiagram<Point>) -> HashSet<Edge> {
    let mut unique_extremes: HashSet<Edge> = HashSet::new();
    for cell in diagram.cells().iter() {
        let extremes: Vec<Coordinates> = cell.points().iter().map(|x| (x.x as usize, x.y as usize)).collect();

        // connect the points
        for i in 0..extremes.len() - 1 {
            unique_extremes.insert(Edge {
                start: extremes[i],
                end: extremes[i + 1],
            });
        }
        unique_extremes.insert(Edge {
            start: extremes[0],
            end: extremes[extremes.len() - 1],
        });
    }
    unique_extremes
}

#[inline(always)]
fn get_voronoi_diagram(elevation_map: &[Vec<f64>], centers: &[Coordinates]) -> VoronoiDiagram<Point> {
    // convert centers to (f64,f64)
    let points: Vec<(f64, f64)> = centers.iter().map(|(y, x)| (*x as f64, *y as f64)).collect();

    // voronoi diagram
    VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &((elevation_map.len() - 1) as f64, (elevation_map.len() - 1) as f64), &points).unwrap()
}

#[inline(always)]
fn fix_extremes(edges: HashSet<Edge>, size: usize) -> Vec<Edge> {
    let mut edges: Vec<Edge> = edges.into_iter().collect();
    for edge in edges.iter_mut() {
        edge.start.0 = if edge.start.0 >= size - 2 { size } else { edge.start.0 };
        edge.start.1 = if edge.start.1 >= size - 2 { size } else { edge.start.1 };
        edge.end.0 = if edge.end.0 >= size - 2 { size } else { edge.end.0 };
        edge.end.1 = if edge.end.1 >= size - 2 { size } else { edge.end.1 };
    }
    edges
}

#[allow(dead_code)]
fn are_extremes_on_border(e1: Coordinates, e2: Coordinates, size: usize) -> bool {
    (e1.0 == 0 && e2.0 == 0) || (e1.0 == size && e2.0 == size) || (e1.1 == 0 && e2.1 == 0) || (e1.1 == size && e2.1 == size)
}

// Function to connect two points with a line segment using Bresenham's algorithm
#[inline(always)]
fn connect_points(start: Coordinates, end: Coordinates) -> Vec<Coordinates> {
    // Vector to store the points along the line segment
    let mut line_segments: Vec<Coordinates> = Vec::new();

    // Calculate the differences in x and y coordinates
    let dx = end.0 as isize - start.0 as isize;
    let dy = end.1 as isize - start.1 as isize;

    // Determine the number of steps needed for the line using the larger of dx and dy
    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() } as f64;

    // Calculate the increments for x and y based on the number of steps
    let x_increment = dx as f64 / steps;
    let y_increment = dy as f64 / steps;

    // Initialize starting coordinates
    let mut x = start.0 as f64;
    let mut y = start.1 as f64;

    // Generate points along the line segment and round them to the nearest integer
    for index in 0..=steps as usize {
        let rounded_x = x.round() as usize;
        let rounded_y = y.round() as usize;

        // avoid diagonal steps
        if index > 0 && index < steps as usize && rounded_x != line_segments[line_segments.len() - 1].0 && rounded_y != line_segments[line_segments.len() - 1].1 {
            line_segments.push((rounded_x, line_segments[line_segments.len() - 1].1));
            line_segments.push((line_segments[line_segments.len() - 1].0, rounded_y));
        }

        line_segments.push((rounded_x, rounded_y));

        x += x_increment;
        y += y_increment;
    }

    //get only the first ten points
    //line_segments.truncate(line_segments.len()/2);

    // Return the vector of points representing the line segment
    // let mut line_segments: Vec<Coordinates> = Vec::new();
    // line_segments.push((start.0, start.1));
    // line_segments.push((end.0, end.1));
    line_segments
}

#[inline(always)]
fn combine_local_maxima(elevation_map: &[Vec<f64>], all_local_maxima: &mut [Coordinates], n_slice_per_side: usize, band_width: usize) -> Vec<Coordinates> {
    let mut hs: HashSet<Coordinates> = HashSet::new();
    let qnt_per_slice = elevation_map.len() / n_slice_per_side;

    //combine the local maxima in the same slice
    for index in 1..n_slice_per_side {
        //for lower_index in higher_index + 1..local_maxima_in_slice.len() {

        //vertical slices
        combine_local_maxima_in_same_slice(
            index,
            get_vertical_slice,
            is_inside_vertical_slice,
            get_delta_x,
            elevation_map,
            all_local_maxima,
            qnt_per_slice,
            band_width,
        )
            .iter()
            .for_each(|x| {
                hs.insert(*x);
            });

        //horizontal slices
        combine_local_maxima_in_same_slice(
            index,
            get_horizontal_slice,
            is_inside_horizontal_slice,
            get_delta_y,
            elevation_map,
            all_local_maxima,
            qnt_per_slice,
            band_width,
        )
            .iter()
            .for_each(|x| {
                hs.insert(*x);
            });
    }

    hs.into_iter().collect()
}

#[inline(always)]
fn combine_local_maxima_in_same_slice(
    index: usize,
    get_slice: fn(usize, usize, usize, usize) -> Slice,
    is_inside_slice: fn(&Coordinates, &Slice) -> bool,
    get_delta: fn(&Coordinates, &Coordinates) -> usize,
    elevation_map: &[Vec<f64>],
    all_local_maxima: &mut [Coordinates],
    qnt_per_slice: usize,
    band_width: usize,
) -> Vec<Coordinates> {
    let slice: Slice = get_slice(elevation_map.len(), index, qnt_per_slice, band_width);

    //get the local maxima in the slice
    let mut local_maxima_in_slice = Vec::new();
    for local_maximum in all_local_maxima.iter() {
        if is_inside_slice(local_maximum, &slice) {
            local_maxima_in_slice.push(*local_maximum);
        }
    }

    //sort the local maxima in the slice by elevation (highest first)
    local_maxima_in_slice.sort_by(|a, b| elevation_map[b.0][b.1].partial_cmp(&elevation_map[a.0][a.1]).unwrap());

    //if the delta (Δx or Δy) is < band_width, remove the local maxima with the lowest elevation
    let mut higher_index = 0;
    let mut lower_index;
    if local_maxima_in_slice.len() > 1 {
        while higher_index < local_maxima_in_slice.len() - 1 {
            lower_index = higher_index + 1;
            while lower_index < local_maxima_in_slice.len() {
                if get_delta(&local_maxima_in_slice[higher_index], &local_maxima_in_slice[lower_index]) <= band_width {
                    local_maxima_in_slice.remove(lower_index); //remove lower local maximum near to higher local maximum
                }
                lower_index += 1;
            }
            higher_index += 1;
        }
    }
    local_maxima_in_slice
}

#[inline(always)]
fn get_delta_x(higher: &Coordinates, lower: &Coordinates) -> usize {
    higher.1.abs_diff(lower.1)
}

#[inline(always)]
fn get_delta_y(higher: &Coordinates, lower: &Coordinates) -> usize {
    higher.0.abs_diff(lower.0)
}

#[inline(always)]
fn is_inside_horizontal_slice(local_maximum: &Coordinates, slice: &Slice) -> bool {
    local_maximum.0 >= slice.start.row && local_maximum.0 <= slice.end.row
}

#[inline(always)]
fn is_inside_vertical_slice(local_maximum: &Coordinates, slice: &Slice) -> bool {
    local_maximum.1 >= slice.start.col && local_maximum.1 <= slice.end.col
}

#[inline(always)]
fn get_horizontal_slice(map_len: usize, row: usize, qnt_per_slice: usize, band_width: usize) -> Slice {
    Slice {
        start: Coordinate {
            row: (row * qnt_per_slice) - (band_width / 2),
            col: 0,
        },
        end: Coordinate {
            row: (row * qnt_per_slice) + (band_width / 2),
            col: map_len - 1,
        },
    }
}

#[inline(always)]
fn get_vertical_slice(map_len: usize, col: usize, qnt_per_slice: usize, band_width: usize) -> Slice {
    Slice {
        start: Coordinate {
            row: 0,
            col: (col * qnt_per_slice) - (band_width / 2),
        },
        end: Coordinate {
            row: map_len - 1,
            col: (col * qnt_per_slice) + (band_width / 2),
        },
    }
}

#[inline(always)]
fn get_local_maxima(elevation_map: &[Vec<f64>], n_slice_side: usize, lower_threshold: f64) -> Vec<Coordinates> {
    let mut local_maxima: Vec<Coordinates> = Vec::new();
    let mut found_local_maximum;
    let slices = slice_vec_2d(elevation_map, n_slice_side);
    for slice in slices {
        found_local_maximum = false;
        let mut local_maximum_point = (slice.start.row, slice.start.col); // set initially the local maximum to the first point in the slice
        for row_index in slice.start.row..slice.end.row {
            for col_index in slice.start.col..slice.end.col {
                if elevation_map[row_index][col_index] >= elevation_map[local_maximum_point.0][local_maximum_point.1] {
                    local_maximum_point = (row_index, col_index);
                    found_local_maximum = true;
                }
            }
        }
        if found_local_maximum && elevation_map[local_maximum_point.0][local_maximum_point.1] > lower_threshold {
            local_maxima.push((local_maximum_point.0, local_maximum_point.1));
        }
    }
    local_maxima
}

// get the maximum value from a slice
#[inline(always)]
#[allow(dead_code)]
fn get_maximum(slice: &[Vec<f64>]) -> Coordinates {
    slice
        .iter()
        .enumerate()
        .flat_map(|(row_index, inner)| inner.iter().enumerate().map(move |(col_index, &value)| (row_index, col_index, value)))
        .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(row_index, col_index, _)| (row_index, col_index))
        .unwrap()
}
