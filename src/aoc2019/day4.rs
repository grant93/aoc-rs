use std::collections::HashMap;

let directions = HashMap::from(
        ('R': (1, 0)),
        ('L', (-1, 0)),
        ('U', (0, 1)),
        ('D', (0, -1))
    );
