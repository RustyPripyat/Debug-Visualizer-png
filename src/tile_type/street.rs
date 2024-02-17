use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

use crate::utils::{slice_vec_2d, Coordinate, Slice};

// TODO doc street

#[derive(Debug, Eq, Clone)]
struct Edge {
    start: Coordinate,
    end: Coordinate,
}

fn remove_duplicates(edges: Vec<Edge>) -> Vec<Edge> {
    let unique_edges: HashSet<Edge> = edges.into_iter().collect();
    unique_edges.into_iter().collect()
}

impl Edge {
    #[allow(dead_code)]
    pub fn is_near(&self, other: &Self) -> bool {
        //check if the edges are near
        (self.start.is_neighbor(&other.start) && self.end.is_neighbor(&other.end)) || (self.end.is_neighbor(&other.start) && self.start.is_neighbor(&other.end))
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Sort the coordinates
        let (first, second) = match self.start.cmp(&self.end) {
            | Ordering::Less => (&self.start, &self.end),
            | _ => (&self.end, &self.start),
        };

        // Hash the sorted coordinates
        first.hash(state);
        second.hash(state);
    }
}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end) || (self.start == other.end && self.end == other.start)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start == other.start {
            self.end.cmp(&other.end)
        } else {
            self.start.cmp(&other.start)
        }
    }
}

pub(crate) fn street_spawn(elevation_map: &[Vec<f64>], n_slice_side: usize, lower_threshold: f64) -> Vec<Vec<Coordinate>> {
    // get local maxima
    let mut local_maxima: Vec<Coordinate> = get_local_maxima(elevation_map, n_slice_side, lower_threshold);

    // combine near local maxima
    let combined_local_maxima: Vec<Coordinate> = combine_local_maxima(elevation_map, &mut local_maxima, n_slice_side, elevation_map.len() / 100);

    // get voronoi diagram
    let diagram = get_voronoi_diagram(elevation_map, &combined_local_maxima);

    // get unique edges extremes from diagram
    let unique_extremes: HashSet<Edge> = get_edges_extremes_from_diagram(diagram);

    // fix edges extremes
    let fixed_extremes = fix_extremes(unique_extremes, elevation_map.len() - 1);

    // remove duplicates
    let unique_edges = remove_duplicates(fixed_extremes);

    unique_edges.iter().map(|edge| connect_points(edge.start, edge.end)).collect()
}

#[inline(always)]
fn get_edges_extremes_from_diagram(diagram: VoronoiDiagram<Point>) -> HashSet<Edge> {
    let mut unique_extremes: HashSet<Edge> = HashSet::new();
    for cell in diagram.cells().iter() {
        let extremes: Vec<Coordinate> = cell
            .points()
            .iter()
            .map(|point| Coordinate {
                col: point.x as usize,
                row: point.y as usize,
            })
            .collect();

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
fn get_voronoi_diagram(elevation_map: &[Vec<f64>], centers: &[Coordinate]) -> VoronoiDiagram<Point> {
    // convert centers to (f64,f64)
    let points: Vec<(f64, f64)> = centers.iter().map(|c| (c.col as f64, c.row as f64)).collect();

    // voronoi diagram
    VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &((elevation_map.len() - 1) as f64, (elevation_map.len() - 1) as f64), &points).unwrap()
}

#[inline(always)]
fn fix_extremes(edges: HashSet<Edge>, size: usize) -> Vec<Edge> {
    let mut edges: Vec<Edge> = edges.into_iter().collect();
    for edge in edges.iter_mut() {
        edge.start.col = if edge.start.col >= size - 2 { size } else { edge.start.col };
        edge.start.row = if edge.start.row >= size - 2 { size } else { edge.start.row };
        edge.end.col = if edge.end.col >= size - 2 { size } else { edge.end.col };
        edge.end.row = if edge.end.row >= size - 2 { size } else { edge.end.row };
    }
    edges
}

#[allow(dead_code)]
fn are_extremes_on_border(e1: Coordinate, e2: Coordinate, size: usize) -> bool {
    (e1.col == 0 && e2.col == 0) || (e1.col == size && e2.col == size) || (e1.row == 0 && e2.row == 0) || (e1.row == size && e2.row == size)
}

// Function to connect two points with a line segment using Bresenham's algorithm
#[inline(always)]
fn connect_points(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    let mut line_segments: Vec<Coordinate> = Vec::new();

    let mut x = start.col as isize;
    let mut y = start.row as isize;

    let dx = (end.col as isize - start.col as isize).abs();
    let dy = -(end.row as isize - start.row as isize).abs();

    let sx = if start.col < end.col { 1 } else { -1 };
    let sy = if start.row < end.row { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        let next_step = Coordinate {
            row: y as usize,
            col: x as usize,
        };

        // add step between diagonal
        if let Some(step) = add_step_between_diagonal(&line_segments, next_step) {
            line_segments.push(step);
        }

        line_segments.push(next_step);

        if x == end.col as isize && y == end.row as isize {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            if x != end.col as isize {
                err += dy;
            }
            x += sx;
        }
        if e2 <= dx {
            if y != end.row as isize {
                err += dx;
            }
            y += sy;
        }
    }

    line_segments
}

#[inline(always)]
fn add_step_between_diagonal(segments: &[Coordinate], next_step: Coordinate) -> Option<Coordinate> {
    if segments.is_empty() {
        return None;
    }

    let last = segments.last().unwrap();
    let is_diagonal = next_step.col != last.col && next_step.row != last.row;

    if is_diagonal {
        Some(Coordinate {
            row: last.row,
            col: next_step.col,
        })
    } else {
        None
    }
}

#[inline(always)]
fn combine_local_maxima(elevation_map: &[Vec<f64>], all_local_maxima: &mut [Coordinate], n_slice_per_side: usize, band_width: usize) -> Vec<Coordinate> {
    let mut hs: HashSet<Coordinate> = HashSet::new();
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
    is_inside_slice: fn(&Coordinate, &Slice) -> bool,
    get_delta: fn(&Coordinate, &Coordinate) -> usize,
    elevation_map: &[Vec<f64>],
    all_local_maxima: &mut [Coordinate],
    qnt_per_slice: usize,
    band_width: usize,
) -> Vec<Coordinate> {
    let slice: Slice = get_slice(elevation_map.len(), index, qnt_per_slice, band_width);

    //get the local maxima in the slice
    let mut local_maxima_in_slice = Vec::new();
    for local_maximum in all_local_maxima.iter() {
        if is_inside_slice(local_maximum, &slice) {
            local_maxima_in_slice.push(*local_maximum);
        }
    }

    //sort the local maxima in the slice by elevation (highest first)
    local_maxima_in_slice.sort_by(|a, b| elevation_map[b.row][b.col].partial_cmp(&elevation_map[a.row][a.col]).unwrap());

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
fn get_delta_x(higher: &Coordinate, lower: &Coordinate) -> usize {
    higher.col.abs_diff(lower.col)
}

#[inline(always)]
fn get_delta_y(higher: &Coordinate, lower: &Coordinate) -> usize {
    higher.row.abs_diff(lower.row)
}

#[inline(always)]
fn is_inside_horizontal_slice(local_maximum: &Coordinate, slice: &Slice) -> bool {
    local_maximum.row >= slice.start.row && local_maximum.row <= slice.end.row
}

#[inline(always)]
fn is_inside_vertical_slice(local_maximum: &Coordinate, slice: &Slice) -> bool {
    local_maximum.col >= slice.start.col && local_maximum.col <= slice.end.col
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
fn get_local_maxima(elevation_map: &[Vec<f64>], n_slice_side: usize, lower_threshold: f64) -> Vec<Coordinate> {
    let mut local_maxima: Vec<Coordinate> = Vec::new();
    let mut found_local_maximum;
    let slices = slice_vec_2d(elevation_map, n_slice_side);
    for slice in slices {
        found_local_maximum = false;

        let mut local_maximum_point = Coordinate {
            row: slice.start.row,
            col: slice.start.col,
        }; // set initially the local maximum to the first point in the slice
        for row in slice.start.row..slice.end.row {
            for col in slice.start.col..slice.end.col {
                if elevation_map[row][col] >= elevation_map[local_maximum_point.row][local_maximum_point.col] {
                    local_maximum_point = Coordinate { row, col };
                    found_local_maximum = true;
                }
            }
        }
        if found_local_maximum && elevation_map[local_maximum_point.row][local_maximum_point.col] > lower_threshold {
            local_maxima.push(local_maximum_point);
        }
    }
    local_maxima
}

// get the maximum value from a slice
#[inline(always)]
#[allow(dead_code)]
fn get_maximum(slice: &[Vec<f64>]) -> Coordinate {
    slice
        .iter()
        .enumerate()
        .flat_map(|(row_index, inner)| inner.iter().enumerate().map(move |(col_index, &value)| (row_index, col_index, value)))
        .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(row_index, col_index, _)| Coordinate {
            row: row_index,
            col: col_index,
        })
        .unwrap()
}
