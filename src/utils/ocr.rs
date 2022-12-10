use array2d::Array2D;

/// Converts a 2D array of booleans, representing a screen, to a string.
/// This string follows the AoC convension of printed characters.
pub fn screen_to_string(screen: &Array2D<bool>) -> String {
    let mut builder = string_builder::Builder::new((screen.column_len() + 1) * screen.row_len());
    for (r, col) in screen.rows_iter().enumerate() {
        for lit in col {
            builder.append(if *lit { '#' } else { '.' });
        }

        if r < screen.column_len() - 1 {
            builder.append('\n');
        }
    }

    builder.string().unwrap()
}
