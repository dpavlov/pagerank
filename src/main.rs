use std::default::Default;
use std::clone::Clone;
use std::fmt::Display;

struct Matrix<T> {
    cols: usize,
    rows: usize,
    data: Vec<T>
}

impl <T: Default + Clone + Display> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            rows: rows,
            cols: cols,
            data: vec!(T::default(); cols * rows)
        }
    }

    fn set(&mut self, rows: usize, cols: usize, value: T) {
        self.data[(rows * self.cols) + cols] = value
    }

    fn get(&self, rows: usize, cols: usize) -> &T {
        return &self.data[(rows * self.cols) + cols]
    }

    fn print(&self) {
        for row in 0 .. self.rows {
            print!("|");
            for col in 0 .. self.cols {
                print!("{}", self.get(row, col));
                if col != (self.cols - 1) {
                    print!(" ")
                }
            }
            print!("|\n");
        }
    }
}

struct Graph {
    adjacency: Matrix<bool>,
    vertexs: Vec<f32>
}

impl Graph {
    fn new(vertex: usize) -> Graph {
        Graph {
            adjacency: Matrix::new(vertex, vertex),
            vertexs: vec!(0.0_f32; vertex)
        }
    }

    fn connect(&mut self, src: usize, dst: usize) {
        self.adjacency.set(src, dst, true)
    }

    fn contributed_vertexes(&self, vertex: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for dst in 0 .. self.vertexs.len() {
            if dst != vertex && *self.adjacency.get(dst, vertex) {
                result.push(dst)
            }
        }
        return result;
    }

    fn outgoing_links_count(&self, vertex: usize) -> usize {
        let mut count = 0;
        for dst in 0 .. self.vertexs.len() {
            if dst != vertex && *self.adjacency.get(vertex, dst) {
                count = count + 1;
            }
        }
        return count;
    }

    fn pagerank(&mut self, iter_num: usize) -> &Vec<f32> {
        let d = 0.85_f32;
        for _ in 0 .. iter_num {
            for vertex in 0..self.vertexs.len() {
                let contributed_vertexes = self.contributed_vertexes(vertex);
                let mut contributed_value_sum = 0_f32;
                for contributed_vertex in contributed_vertexes {
                    let outgoing_links_count = self.outgoing_links_count(contributed_vertex);
                    let pagerank = self.vertexs[contributed_vertex];
                    let contributed_value = pagerank / outgoing_links_count as f32;
                    contributed_value_sum = contributed_value_sum + contributed_value;
                }
                self.vertexs[vertex] = (1.0 - d) + d * contributed_value_sum;
            }
        }
        return &self.vertexs;
    }
}

fn main() {
    let mut g = Graph::new(4);
    g.connect(0, 1);
    g.connect(0, 2);
    g.connect(1, 2);
    g.connect(2, 0);
    g.connect(3, 2);

    g.adjacency.print();
    let pagerank = g.pagerank(100);
    for vertex in 0 .. pagerank.len() {
        println!("vertex: {}, pagerank: {}", vertex, pagerank.get(vertex).unwrap())
    }

}


//See examples from https://www.cs.princeton.edu/~chazelle/courses/BIB/pagerank.htm
#[test]
fn test_example_1() {
    let mut g = Graph::new(2);
    g.connect(0, 1);
    g.connect(1, 0);

    let pagerank = g.pagerank(100);
    let epsilon: f32 = 0.01;
    assert!((pagerank.get(0).unwrap() - 1.0).abs() < epsilon);
    assert!((pagerank.get(1).unwrap() - 1.0).abs() < epsilon);
}

#[test]
fn test_example_2() {
    let mut g = Graph::new(4);
    g.connect(0, 1);
    g.connect(0, 2);
    g.connect(1, 2);
    g.connect(2, 0);
    g.connect(3, 2);

    let pagerank = g.pagerank(100);
    let epsilon: f32 = 0.01;
    assert!((pagerank.get(0).unwrap() - 1.5).abs() < epsilon);
    assert!((pagerank.get(1).unwrap() - 0.78).abs() < epsilon);
    assert!((pagerank.get(2).unwrap() - 1.58).abs() < epsilon);
    assert!((pagerank.get(3).unwrap() - 0.15).abs() < epsilon);
}

#[test]
fn test_example_3() {
    let mut g = Graph::new(8);
    g.connect(0, 1);
    g.connect(0, 2);
    g.connect(0, 3);
    g.connect(1, 0);
    g.connect(2, 0);
    g.connect(3, 0);
    g.connect(3, 4);
    g.connect(3, 5);
    g.connect(3, 6);
    g.connect(3, 7);
    g.connect(4, 0);
    g.connect(5, 0);
    g.connect(6, 0);
    g.connect(7, 0);

    let pagerank = g.pagerank(100);
    let epsilon: f32 = 0.01;
    assert!((pagerank.get(0).unwrap() - 3.35).abs() < epsilon);
    assert!((pagerank.get(1).unwrap() - 1.1).abs() < epsilon);
    assert!((pagerank.get(2).unwrap() - 1.1).abs() < epsilon);
    assert!((pagerank.get(3).unwrap() - 1.1).abs() < epsilon);
    assert!((pagerank.get(4).unwrap() - 0.34).abs() < epsilon);
    assert!((pagerank.get(5).unwrap() - 0.34).abs() < epsilon);
    assert!((pagerank.get(6).unwrap() - 0.34).abs() < epsilon);
    assert!((pagerank.get(7).unwrap() - 0.34).abs() < epsilon);
}
