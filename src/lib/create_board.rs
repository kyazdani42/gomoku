use super::game::Tile;

pub fn create_board(board_size: i32) -> Vec<Vec<Tile>> {
    let mut all_alignments = vec![];

    let board_size_less = board_size - 1;

    for line in 0..board_size {
        let mut line_alignment = vec![];

        for col in 0..board_size {
            let tile = (line, col);
            line_alignment.push(tile);

            if line == 0 {
                let mut col_alignment = vec![];
                let mut align_diag_1 = vec![];
                let mut align_diag_2 = vec![];
                let mut align_diag_3 = vec![];
                let mut align_diag_4 = vec![];
                for line_iter in 0..board_size {
                    let column_tile = (line_iter, col);
                    col_alignment.push(column_tile);

                    let line_iter_reverse = board_size_less - line_iter;

                    let diag_col = line_iter + col;
                    let diag_col_2 = line_iter - col;
                    let diag_col_3 = line_iter_reverse + col;
                    let diag_col_4 = line_iter_reverse - col;

                    if diag_col < board_size {
                        let diagonal_tile = (line_iter, diag_col);
                        align_diag_1.push(diagonal_tile);
                    }
                    if diag_col_3 < board_size {
                        let diagonal_tile = (line_iter_reverse, diag_col_3);
                        align_diag_3.push(diagonal_tile);
                    }

                    if col == 0 || col == board_size_less {
                        continue;
                    }

                    if diag_col_2 > -1 {
                        let diagonal_tile = (line_iter, diag_col_2);
                        align_diag_2.push(diagonal_tile);
                    }

                    if diag_col_4 > -1 {
                        let diagonal_tile = (line_iter_reverse, diag_col_4);
                        align_diag_4.push(diagonal_tile);
                    }
                }
                all_alignments.push(col_alignment);
                if align_diag_1.len() > 3 {
                    all_alignments.push(align_diag_1);
                }
                if align_diag_2.len() > 3 {
                    all_alignments.push(align_diag_2);
                }
                if align_diag_3.len() > 3 {
                    all_alignments.push(align_diag_3);
                }
                if align_diag_4.len() > 3 {
                    all_alignments.push(align_diag_4);
                }
            }
        }
        all_alignments.push(line_alignment);
    }

    all_alignments
}
