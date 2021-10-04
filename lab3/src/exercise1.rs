use std::cmp::Ordering;

/// Represents a track with a rank and a title.
#[derive(Debug)]
pub struct Song {
    rank: u32,
    title: String,
}

/// Returns the average rank of a list of songs.
pub fn average_rank(songs: &Vec<Song>) -> f64 {
    if songs.is_empty() {
        panic!("no songs provided!");
    }

    let mut average_rank: f64 = 0.0;

    for song in songs {
        average_rank += song.rank as f64;
    }
    average_rank /= songs.len() as f64;

    average_rank
}

/// Filter the tracks in the `songs` list by only keeping those which have
/// a rank strictly higher than `min_rank`.
// It would be more idiomatic to use `&[Song]` (slices) rather than `&Vec<Song>`
pub fn filter_songs(songs: &Vec<Song>, min_rank: u32) -> Vec<Song> {
    let mut filtered_songs: Vec<Song> = Vec::new();

    for song in songs {
        match song.rank.cmp(&min_rank) {
            Ordering::Greater => {
                // Kinda not good to create a copy here
                // It requires a call to `clone` in order to copy
                // the title which can be expensinve on a long `String`
                let filtered = Song {
                    rank: song.rank,
                    title: song.title.clone(),
                };
                filtered_songs.push(filtered);
            },
            _ => continue,
        }
    }

    filtered_songs
}

/// Filter the tracks in the `songs` list by only keeping those which have
/// a rank strictly higher than the average rank of the list of songs.
// It would be more idiomatic to use `&[Song]` (slices) rather than `&Vec<Song>`
pub fn good_songs(songs: &Vec<Song>) -> Vec<Song> {
    let average_rank = average_rank(&songs);
    filter_songs(songs, average_rank as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Song> {
        vec![
            Song {
                rank: 4,
                title: String::from("Stairway to Heaven"),
            },
            Song {
                rank: 2,
                title: String::from("Never Gonna Give You Up"),
            },
            Song {
                rank: 5,
                title: String::from("Nigerian Marketplace"),
            },
        ]
    }

    #[test]
    #[should_panic]
    fn empty() {
        let sgs = Vec::new();
        let _m = average_rank(&sgs);
    }

    #[test]
    fn average_with_one_song() {
        let sgs = vec![Song {
            rank: 4,
            title: String::from("Stairway to Heaven"),
        }];

        let m = average_rank(&sgs);
        assert_eq!(m, 4.0);
    }

    #[test]
    fn average_with_multiple_songs() {
        let sgs = example();
        let m = average_rank(&sgs);
        assert!((m - 3.66666).abs() < 0.001);
    }

    #[test]
    fn filter() {
        let sgs = example();
        let f = filter_songs(&sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");
    }

    #[test]
    fn bests() {
        let sgs = example();
        let f = good_songs(&sgs);
        assert!(f.len() == 2);
        for m in &f {
            assert!(m.rank >= 4);
        }
    }
}
