pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // Your code here
    return get_shortest_path(&maze, start, end, vec![(start.0, start.1)]);
}

fn get_shortest_path(
    maze: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    end: (usize, usize),
    traveled_path: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    println!("current_pos: {:?}", current_pos);
    // Your code here
    // base case
    // if end
    if current_pos == end {
        return traveled_path;
    }

    // recursive case
    // get all possible moves
    let possible_moves = get_possible_moves(&maze, current_pos);
    let unique_next_moves: Vec<(usize, usize)> = possible_moves
        .into_iter()
        .filter(|next_move| !traveled_path.contains(&next_move))
        .collect();

    // for each move, get the shortest path
    let mut shortest_path = vec![];
    for next_move in unique_next_moves {
        let mut traveled_path_copy = traveled_path.clone();
        traveled_path_copy.push(next_move);

        let new_path = get_shortest_path(maze, next_move, end, traveled_path_copy);

        // if no path, continue
        if new_path.len() == 0 {
            continue;
        }

        // if no shortest path, set shortest path
        if shortest_path.len() == 0 {
            shortest_path = new_path;
            continue;
        }

        if new_path.len() < shortest_path.len() {
            shortest_path = new_path;
        }
    }
    return shortest_path;
}

fn get_possible_moves(maze: &Vec<Vec<char>>, current_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let y_pos = current_pos.0;
    let x_pos = current_pos.1;

    // get all next moves
    let mut next_moves = vec![];

    // up
    if y_pos > 0 && maze[y_pos - 1][x_pos] != '#' {
        next_moves.push((y_pos - 1, x_pos));
    }

    // down
    if y_pos < maze.len() - 1 && maze[y_pos + 1][x_pos] != '#' {
        next_moves.push((y_pos + 1, x_pos));
    }

    // left
    if x_pos > 0 && maze[y_pos][x_pos - 1] != '#' {
        next_moves.push((y_pos, x_pos - 1));
    }

    // right
    if x_pos < maze[0].len() - 1 && maze[y_pos][x_pos + 1] != '#' {
        next_moves.push((y_pos, x_pos + 1));
    }

    return next_moves;
}
