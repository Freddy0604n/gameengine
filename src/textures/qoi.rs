pub fn encode(image_data: Vec<[u8; 4]>, image_dimensions: [u8; 2]) -> Vec<u8> { // converts an array of RGBA pixels into the qoi format
    //TODO
    let previous: [u8; 4] = [0, 0, 0, 255];
    let indecis: [[u8; 4]; 64] = [[0; 4]; 64];
    let encoded_vec: Vec<u8> = Vec::new();
    for i in b"qoif" { 
        encoded_vec.push(*i);
    }
    for i in image_data {
        indecis[index_position(i)] = i;
    }
    encoded_vec
}

fn index_position(pixel: [u8; 4]) -> usize {
    ((pixel[0] * 3 + pixel[1] * 5 + pixel[2] * 7 + pixel[4] * 11) % 64) as usize
} 