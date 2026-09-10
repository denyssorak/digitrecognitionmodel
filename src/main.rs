use std::fs::File;
use std::io::Read;

mod matrix;
use matrix::Matrix;

fn load_labels(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(&path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let _magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    let _count = u32::from_be_bytes(bytes[4..8].try_into().unwrap());

    let labels = bytes[8..].to_vec();

    Ok(labels)
}

fn load_images(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(&path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let _magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    let _count = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
    let _rows = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    let _cols = u32::from_be_bytes(bytes[12..16].try_into().unwrap());

    let images = bytes[16..].to_vec();

    Ok(images)
}

fn main() -> std::io::Result<()> {
    let _labels = load_labels("data/train-labels-idx1-ubyte")?;
    //  println!("{:?}", &labels[0..10]);

    let _images = load_images("data/train-images-idx3-ubyte")?;
    //  println!("{:?}", &images[0..10]);
    //  println!("{:?}", &images[200..210]);
    //  println!("{:?}", &images[300..310]);
    /*
        let mut m1 = Matrix::new(2, 3);
        m1.set(0, 0, 1.0);
        m1.set(0, 1, 2.0);
        m1.set(0, 2, 3.0);
        m1.set(1, 0, 4.0);
        m1.set(1, 1, 5.0);
        m1.set(1, 2, 6.0);

        let mut m2 = Matrix::new(3, 4);
        m2.set(0, 0, 1.0);
        m2.set(0, 1, 2.0);
        m2.set(0, 2, 3.0);
        m2.set(0, 3, 4.0);
        m2.set(1, 0, 5.0);
        m2.set(1, 1, 6.0);
        m2.set(1, 2, 7.0);
        m2.set(1, 3, 8.0);
        m2.set(2, 0, 9.0);
        m2.set(2, 1, 10.0);
        m2.set(2, 2, 11.0);
        m2.set(2, 3, 12.0);


        let result = m1.matmul(&m2);

        println!(
            "{:?} {:?} {:?} {:?}",
            result.get(0, 0),
            result.get(0, 1),
            result.get(0, 2),
            result.get(0, 3)
        );
        println!(
            "{:?} {:?} {:?} {:?}",
            result.get(1, 0),
            result.get(1, 1),
            result.get(1, 2),
            result.get(1, 3)
        );
    */
    Ok(())
}
