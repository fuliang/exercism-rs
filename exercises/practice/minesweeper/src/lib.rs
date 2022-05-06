const BLANK: u8 = ' ' as u8;
const MINE: u8 = '*' as u8;
const ZERO: u8 = '0' as u8;

const ROW_COL: &'static [(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1)
];

pub fn annotate_count(board: &Vec<&[u8]>, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;

    for (i, j) in ROW_COL {
        let r = row as isize + i;
        let c = col as isize + j;

        if r >= 0 && r < board.len() as isize && c >= 0 && c < board[row].len() as isize {
            if board[r as usize][c as usize] == MINE {
                count += 1;
            }
        }
    }
    return count;
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = minefield.clone().into_iter().map(|&row| row.as_bytes()).collect::<Vec<&[u8]>>();
    let mut annotation_board: Vec<Vec<u8>> = Vec::new();

    for row in 0..board.len() {
        let mut annotation_row = vec![];

        
        for col in 0..board[row].len() {
            if board[row][col] == MINE {
                annotation_row.push(MINE);
                continue;
            }

            let count = annotate_count(&board, row, col);
            annotation_row.push(if count == 0 {BLANK} else {count + ZERO});
        }

        annotation_board.push(annotation_row);
    }

    annotation_board
        .iter()
        .map(|row| 
            row.iter()
                .map(|e| *e as char).collect::<String>())
                .collect::<Vec<String>>()
}
