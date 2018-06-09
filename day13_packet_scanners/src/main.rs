use std::fs::File;
use std::io::Read;

struct Layer {
    range: i32,
    scanner_pos: i32,
    scanner_velocity: i32,
}

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    // Populate layers
    let mut firewall = Vec::new();
    for line in input.lines() {
        let layer_info: Vec<i32> = line.split(':').map(|x| x.trim().parse().unwrap()).collect();
        let depth = layer_info[0];
        let range = layer_info[1];

        let layer = Layer {
            range,
            scanner_pos: 0,
            scanner_velocity: 1,
        };

        // Add empty layers if necessary
        while firewall.len() < depth as usize {
            firewall.push(None);
        }

        firewall.push(Some(layer));
    }

    let mut severity = 0;
    for cur_depth in 0..firewall.len() {
        // Check if we got caught
        if let Some(ref layer) = firewall[cur_depth] {
            if layer.scanner_pos == 0 {
                severity += cur_depth * (layer.range as usize);
            }
        }
        // Advance scanners
        for maybe_layer in &mut firewall {
            if let Some(ref mut layer) = maybe_layer {
                advance_scanner(layer);
            }
        }
    }
    println!("{}", severity);
}

fn advance_scanner(layer: &mut Layer) {
    // Switch scanner direction if it is about to go out of bounds
    if (layer.scanner_pos + layer.scanner_velocity >= layer.range)
        || (layer.scanner_pos + layer.scanner_velocity < 0)
    {
        layer.scanner_velocity = -layer.scanner_velocity;
    }
    layer.scanner_pos += layer.scanner_velocity;
}
