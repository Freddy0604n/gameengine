pub fn encode(image_data: Vec<[u8; 4]>, image_dimensions: [u8; 2]) -> Vec<u8> { // converts an array of RGBA pixels into the qoi format
    //TODO
    let previous: [u8; 4] = [0, 0, 0, 255];
    let indecis: [[u8; 4]; 64] = [[0; 4]; 64];
    let encoded_vec: Vec<u8> = Vec::new();
    for i in b"qoif" {
        encoded_vec.push(*i);
    }

    for i in image_data {
        let index = index_position(i);
        if indecis[index] == i {
            encoded_vec.push(index as u8);
        } else {
            indecis[index] = i;
            encoded_vec.push(0xff); // rgba 8bit tag
            for j in i {
                encoded_vec.push(j);
            }
        }
    }

    // closing bytes
    for i in encoded_vec {
        encoded_vec.push(0);
    }
    encoded_vec.push(0x01);

    encoded_vec
}

pub fn decode(qoi: Vec<u8>) -> Vec<[u8; 4]> { // decodes a qoi array to an array of rgba pixels
    // TODO
    let decoded_vec: Vec<[u8; 4]> = Vec::new();
    decoded_vec
}

fn index_position(pixel: [u8; 4]) -> usize { 
    ((pixel[0] * 3 + pixel[1] * 5 + pixel[2] * 7 + pixel[4] * 11) % 64) as usize
} 