use crate::drawing::GRID_SIZE;
use crate::preprocessing::center_drawing;

// A template for a number
type Template = [[u8; GRID_SIZE]; GRID_SIZE];

fn load_templates() -> Vec<(char, Template)> {
    let mut templates = Vec::new();
    const X_S: usize = 8; // Start X
    const X_E: usize = 19; // End X
    const Y_S: usize = 5; // Start Y
    const Y_E: usize = 22; // End Y
    const X_M: usize = (X_S + X_E) / 2; // Mid X
    const Y_M: usize = (Y_S + Y_E) / 2; // Mid Y

    // Template for '0'
    let mut zero: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for y in Y_S..=Y_E { zero[y][X_S] = 255; zero[y][X_E] = 255; }
    for x in X_S..=X_E { zero[Y_S][x] = 255; zero[Y_E][x] = 255; }
    templates.push(('0', zero));

    // Template for '1'
    let mut one: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for y in Y_S..=Y_E { one[y][X_M] = 255; }
    one[Y_S+2][X_M-1] = 255; one[Y_S+1][X_M-2] = 255;
    for x in (X_M-2)..=(X_M+2) { one[Y_E][x] = 255; }
    templates.push(('1', one));

    // Template for '2'
    let mut two: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { two[Y_S][x] = 255; two[Y_M][x] = 255; two[Y_E][x] = 255; }
    for y in Y_S..=Y_M { two[y][X_E] = 255; }
    for y in Y_M..=Y_E { two[y][X_S] = 255; }
    templates.push(('2', two));

    // Template for '3'
    let mut three: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { three[Y_S][x] = 255; three[Y_M][x] = 255; three[Y_E][x] = 255; }
    for y in Y_S..=Y_E { three[y][X_E] = 255; }
    templates.push(('3', three));

    // Template for '4'
    let mut four: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { four[Y_M][x] = 255; }
    for y in Y_S..=Y_M { four[y][X_S] = 255; }
    for y in Y_S..=Y_E { four[y][X_E] = 255; }
    templates.push(('4', four));

    // Template for '5'
    let mut five: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { five[Y_S][x] = 255; five[Y_M][x] = 255; five[Y_E][x] = 255; }
    for y in Y_S..=Y_M { five[y][X_S] = 255; }
    for y in Y_M..=Y_E { five[y][X_E] = 255; }
    templates.push(('5', five));

    // Template for '6'
    let mut six: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { six[Y_S][x] = 255; six[Y_M][x] = 255; six[Y_E][x] = 255; }
    for y in Y_S..=Y_E { six[y][X_S] = 255; }
    for y in Y_M..=Y_E { six[y][X_E] = 255; }
    templates.push(('6', six));

    // Template for '7'
    let mut seven: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { seven[Y_S][x] = 255; }
    for y in Y_S..=Y_E { seven[y][X_E] = 255; }
    templates.push(('7', seven));

    // Template for '8'
    let mut eight: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { eight[Y_S][x] = 255; eight[Y_M][x] = 255; eight[Y_E][x] = 255; }
    for y in Y_S..=Y_E { eight[y][X_S] = 255; eight[y][X_E] = 255; }
    templates.push(('8', eight));
    
    // Template for '9'
    let mut nine: Template = [[0; GRID_SIZE]; GRID_SIZE];
    for x in X_S..=X_E { nine[Y_S][x] = 255; nine[Y_M][x] = 255; nine[Y_E][x] = 255; }
    for y in Y_S..=Y_M { nine[y][X_S] = 255; }
    for y in Y_S..=Y_E { nine[y][X_E] = 255; }
    templates.push(('9', nine));
    
    templates
}


// Compares the user's grid to a single template using the Jaccard Index.
fn compare_to_template(grid: &Template, template: &Template) -> f64 {
    let mut intersection = 0.0;
    let mut union = 0.0;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let user_pixel = grid[y][x] > 0;
            let template_pixel = template[y][x] > 0;

            if user_pixel && template_pixel {
                intersection += 1.0;
            }
            if user_pixel || template_pixel {
                union += 1.0;
            }
        }
    }
    
    if union == 0.0 { 0.0 } else { intersection / union }
}

pub fn recognize_digit(grid: &Template) {
    println!("Preprocessing image...");
    let processed_grid = center_drawing(grid);
    let templates = load_templates();
    let mut best_match = ('?', -1.0); // (character, score)

    println!("Comparing to templates...");
    for (digit, template) in templates.iter() {
        let score = compare_to_template(&processed_grid, template);
        println!("  - Compared to '{}', overlap score: {:.2}", digit, score);
        if score > best_match.1 {
            best_match = (*digit, score);
        }
    }

    println!("\n✅ I think you drew a: {}\n", best_match.0);
}