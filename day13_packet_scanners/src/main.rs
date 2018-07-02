use std::fs::File;
use std::io::*;

#[derive(Clone, Debug)]
struct Layer {
    range: i32,
    scanner_pos: i32,
    scanner_velocity: i32,
}

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .collect();

    // Populate layers
    let mut firewall = Vec::new();
    for line in lines {
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

    let detection_severity_when_starting_immedialetly =
        simulate_packet_travel(&firewall).unwrap_or(0);
    println!(
        "Detection severity when immediately sending package: {}",
        detection_severity_when_starting_immedialetly
    );

    // Determine smallest delay after which a package can be send undetected
    let mut firewall_copy = firewall.clone();
    let mut delay_in_picoseconds = 0;
    while simulate_packet_travel(&firewall_copy).is_some() {
        delay_in_picoseconds += 1;
        advance_scanners(&mut firewall_copy);
    }
    println!(
        "Necessary minimal delay needed to send package undetected: {}",
        delay_in_picoseconds
    );
}

/// Calculates detection severity after simulating package travel through given firewall
/// Returns None if package was not detected
fn simulate_packet_travel(firewall: &[Option<Layer>]) -> Option<usize> {
    let mut firewall_copy = firewall.to_owned();

    let mut detection_severity = None;
    for cur_depth in 0..firewall_copy.len() {
        // Check if we got caught
        if let Some(ref layer) = firewall_copy[cur_depth] {
            if layer.scanner_pos == 0 {
                detection_severity =
                    Some(detection_severity.unwrap_or(0) + cur_depth * (layer.range as usize))
            }
        }
        advance_scanners(&mut firewall_copy);
    }
    detection_severity
}

/// Advances all scanners in every layer of a given firewall one time-step forward
fn advance_scanners(firewall: &mut [Option<Layer>]) {
    for maybe_layer in firewall {
        if let Some(ref mut layer) = maybe_layer {
            advance_scanner(layer);
        }
    }
}

/// Advances a scanner in a given firewall layer one time-step forward
fn advance_scanner(layer: &mut Layer) {
    // Switch scanner direction if it is about to go out of bounds
    if (layer.scanner_pos + layer.scanner_velocity >= layer.range)
        || (layer.scanner_pos + layer.scanner_velocity < 0)
    {
        layer.scanner_velocity = -layer.scanner_velocity;
    }
    layer.scanner_pos += layer.scanner_velocity;
}
