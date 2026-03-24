#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates a MovieTheater struct for testing examples.
//
// This struct will be used in subsequent testing examples.
//
// Run: cargo test --example s22_movie_theater

/// Represents a movie.
#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    duration_mins: u32,
    rating: f64,
}

/// A movie theater with a schedule of movies.
#[derive(Debug)]
struct MovieTheater {
    name: String,
    capacity: u32,
    movies: Vec<Movie>,
}

impl MovieTheater {
    /// Create a new empty theater.
    fn new(name: &str, capacity: u32) -> Self {
        Self {
            name: name.to_string(),
            capacity,
            movies: Vec::new(),
        }
    }

    /// Add a movie to the schedule.
    fn add_movie(&mut self, title: &str, duration: u32, rating: f64) {
        self.movies.push(Movie {
            title: title.to_string(),
            duration_mins: duration,
            rating,
        });
    }

    /// Get the highest-rated movie.
    fn best_movie(&self) -> Option<&Movie> {
        self.movies
            .iter()
            .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap())
    }

    /// Get movies longer than a given duration.
    fn long_movies(&self, min_duration: u32) -> Vec<&Movie> {
        self.movies
            .iter()
            .filter(|m| m.duration_mins >= min_duration)
            .collect()
    }

    /// Calculate average rating.
    fn average_rating(&self) -> f64 {
        if self.movies.is_empty() {
            return 0.0;
        }
        self.movies.iter().map(|m| m.rating).sum::<f64>() / self.movies.len() as f64
    }

    /// Check if the theater has room for a group.
    fn can_seat(&self, group_size: u32) -> bool {
        group_size <= self.capacity
    }
}

fn main() {
    let mut theater = MovieTheater::new("Grand Cinema", 200);
    theater.add_movie("Inception", 148, 8.8);
    theater.add_movie("The Matrix", 136, 8.7);
    theater.add_movie("Interstellar", 169, 8.6);

    println!("Theater: {}", theater.name);
    println!("Best: {:?}", theater.best_movie());
    println!("Avg rating: {:.1}", theater.average_rating());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_theater() -> MovieTheater {
        let mut t = MovieTheater::new("Test Cinema", 100);
        t.add_movie("Short Film", 90, 7.0);
        t.add_movie("Epic Movie", 180, 9.0);
        t.add_movie("Average Film", 120, 8.0);
        t
    }

    #[test]
    fn test_new_theater() {
        let t = MovieTheater::new("Test", 50);
        assert_eq!(t.name, "Test");
        assert_eq!(t.capacity, 50);
        assert!(t.movies.is_empty());
    }

    #[test]
    fn test_add_movie() {
        let mut t = MovieTheater::new("Test", 50);
        t.add_movie("Film", 120, 8.0);
        assert_eq!(t.movies.len(), 1);
        assert_eq!(t.movies[0].title, "Film");
    }

    #[test]
    fn test_best_movie() {
        let t = setup_theater();
        let best = t.best_movie().unwrap();
        assert_eq!(best.title, "Epic Movie");
        assert_eq!(best.rating, 9.0);
    }

    #[test]
    fn test_long_movies() {
        let t = setup_theater();
        let long = t.long_movies(120);
        assert_eq!(long.len(), 2); // 180 and 120
    }

    #[test]
    fn test_average_rating() {
        let t = setup_theater();
        let avg = t.average_rating();
        assert!((avg - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_average_empty() {
        let t = MovieTheater::new("Empty", 10);
        assert_eq!(t.average_rating(), 0.0);
    }

    #[test]
    fn test_can_seat() {
        let t = MovieTheater::new("Small", 50);
        assert!(t.can_seat(50));
        assert!(t.can_seat(1));
        assert!(!t.can_seat(51));
    }
}
